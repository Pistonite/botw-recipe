"""
Generate the code!
"""

HEADER = """
//! Automatically generated.
//!
//! DO NOT EDIT. See packages/generated/README.md for more information.

"""

from dataclasses import dataclass
import os
import yaml
import shutil
import subprocess
import sys

def main():
    actors, groups, actor_to_group_idx = load_actors_and_groups()
    pe_only_actors, actor_data, english_names = load_actor_data()
    tags = load_tags()
    print("Tags:", len(tags))
    run_rustfmt([
        generate_cook_item(),
        generate_actor(actors, english_names, groups, actor_to_group_idx, pe_only_actors),
        generate_group(english_names, groups, pe_only_actors),
        generate_tags(tags),
        generate_actor_data(actors, english_names, actor_data),
    ])


COMMON_ENUM_DERIVES = [
    "Copy", "Clone", "PartialEq", "Eq", "PartialOrd", "Ord", "Hash"
]

# Tags referenced in the cooking code
EXTRA_TAGS = [
    "CookLowPrice",
    "CookEnemy",
    "CookSpice"
]

@dataclass
class ActorData:
    effect: str # CookEffect enum name
    recipe_tag: str # Tag enum name
    boost_effect_time: int
    boost_hp: int
    boost_max_hearts: int
    boost_stamina: int
    boost_success_rate: int
    effect_level: int
    effect_time: int
    hp: int
    buy_price: int
    sell_price: int
    tags: list[str]
    matchable_recipes: tuple[int, int, int] # bitset

def generate_actor_data(
    actors: list[str],
    english_names: dict[str, str],
    actor_data: dict[str, ActorData],
):
    actor_data_lines = [
        "// None",
        "ActorData::empty(),",
    ]

    progress = spp.printer(len(actors), "Generate ActorData")
    for (i, actor) in enumerate(actors):
        progress.print(i, actor)
        english = english_names[actor]
        data = actor_data[actor]
        a1, a2, a3 = data.matchable_recipes
        actor_data_lines += [
            "// " + english,
            "ActorData {",
            f"actor: Actor::{actor},",
            f"recipe_tag: Tag::{data.recipe_tag},",
            "boost: Boost {",
            f"effective_time: {data.boost_effect_time},",
            f"hit_point_recover: {data.boost_hp},",
            f"max_heart_level: {data.boost_max_hearts},",
            f"stamina_level: {data.boost_stamina},",
            f"success_rate: {data.boost_success_rate},",
            "},",
            f"effect: CookEffect::{data.effect},",
            f"effect_level: {data.effect_level},",
            f"effect_time: {data.effect_time},",
            f"hp: {data.hp},",
            f"buy_price: {data.buy_price},",
            f"sell_price: {data.sell_price},",
            f"tags: enum_set! {{ {"|".join(f"Tag::{tag}" for tag in data.tags)} }},",
            f"matchable_recipes: RecipeSet::new(0x{a1:016x}, 0x{a2:016x}, 0x{a3:016x}),",
            "},",
        ]

    progress.done()

    lines = [
        "use enumset::{EnumSet, enum_set};",
        "use super::{ActorData, Boost};",
        "use crate::{Tag, Actor, CookEffect, RecipeSet};",
        "",
        # add 1 for the None variant
        f"pub(crate) static ACTOR_DATA: [ActorData; {len(actors) + 1}] = [",
    ] + actor_data_lines + [
        "];",
    ]

    return write_rust_source(src_file("actor_data", "gen.rs"), lines)

def generate_tags(tags: list[str]):
    recipe_tags = set(load_recipe_tags())
    enum_lines = []
    tag_str_lines = []
    tag_from_str_lines = []
    used_in_matching_lines = []
    for tag in tags:
        enum_lines.append(f"    {tag},")
        tag_str_lines.append(f"Self::{tag} => \"{tag}\",")
        tag_from_str_lines.append(f"    \"{tag}\" => Tag::{tag},")
        if tag in recipe_tags:
            used_in_matching_lines.append(f"Self::{tag} => true,")
    used_in_matching_lines.append("_ => false")

    lines = [
        "/// Tags used in the cooking code/recipes",
        f"#[cfg_attr(feature = \"tag-enum-map\", derive(enum_map::Enum))]",
        f"#[cfg_attr(feature = \"tag-enum-set\", derive(enumset::EnumSetType, PartialOrd, Ord, Hash))]",
        f"#[cfg_attr(not(feature = \"tag-enum-set\"), derive({",".join(COMMON_ENUM_DERIVES)}))]",
        "#[allow(non_camel_case_types)]",
        "#[derive(Default)]",
        "#[repr(u8)]",
        "pub enum Tag {",
        "/// No tag. This is used to make recipe matching implementation cleaner",
        "#[default]",
        "None = 0,",
    ] + enum_lines + [
        "}",
        "impl Tag {",
        "/// Get the string representation of the tag",
        "#[cfg(feature = \"tag-to-str\")]",
        "pub const fn as_str(&self) -> &'static str {",
        "match self {",
        "Self::None => \"<none>\",",
    ] + tag_str_lines + [
        "}}",
        "/// Check if the tag is used in recipe matching",
        "///",
        "/// Each actor should have at most 1 of these tags",
        "pub const fn is_used_in_recipe_matching(&self) -> bool {",
        "match self {",
    ] + used_in_matching_lines + [
        "}}",
        "/// Get the tag from string representation",
        "#[cfg(feature = \"tag-from-str\")]",
        "pub fn from_str(name: &str) -> Option<Self> {",
        "TAG_STR_MAP.get(name).copied()",
        "}}",
        "#[cfg(feature = \"tag-from-str\")]",
        "static TAG_STR_MAP: phf::Map<&'static str, Tag> = phf::phf_map! {",
    ] + tag_from_str_lines + [
        "};",
    ] + generate_rust_count_macro("tag", len(tags))

    return write_rust_source(src_file("tag.rs"), lines)

def generate_group(
    english_names: dict[str, str],
    groups: list[tuple[str, list[str]]],
    pe_only_actors: set[str]
):
    enum_lines = [
        "/// \"Empty\" slot in recipe input",
        "#[default]",
        "None = 0,",
    ]
    actors_lines = [
        "Self::None => &[],",
    ]
    any_pe_only_lines = []
    all_pe_only_lines = []

    progress = spp.printer(len(groups), "Generate Group")

    for (i, (group_name, actors_in_group)) in enumerate(groups):
        progress.print(i, group_name)
        # This check is needed, otherwise the first_actor() implementation
        # will panic
        if len(actors_in_group) == 0:
            raise ValueError(f"Group {group_name} has no actors")
        english = ", ".join(english_names[actor] for actor in actors_in_group)
        enum_lines.append(f"    /// {english}")
        enum_lines.append(f"    {group_name} = {i+1},")
        actors_lines.append(f"Self::{group_name} => &[{', '.join(f"Actor::{actor}" for actor in actors_in_group)}],")
        pe_only_count = 0
        for actor in actors_in_group:
            if actor in pe_only_actors:
                pe_only_count += 1
        if pe_only_count > 0:
            any_pe_only_lines.append(f"Self::{group_name} => true,")
        if pe_only_count == len(actors_in_group):
            all_pe_only_lines.append(f"Self::{group_name} => true,")
    progress.done()

    any_pe_only_lines.append("_ => false")
    all_pe_only_lines.append("_ => false")


    lines = [
        "use crate::Actor;",
        "",
        "/// Cookable Item Groups (Input of cooking pot)",
        "///",
        "/// Items are grouped together if the cooking code treat them equivalently.",
        "/// This reduces the number of recipes needed to be computed.",
        f"#[cfg_attr(feature = \"actor-enum-map\", derive(enum_map::Enum))]",
        f"#[cfg_attr(feature = \"actor-enum-set\", derive(enumset::EnumSetType, PartialOrd, Ord, Hash))]",
        f"#[cfg_attr(not(feature = \"actor-enum-set\"), derive({",".join(COMMON_ENUM_DERIVES)}))]",
        "#[derive(Default)]",
        "#[allow(non_camel_case_types)]",
        "#[repr(u8)]",
        "pub enum Group {",
    ] + enum_lines + [
        "}",
        "impl Group {",
    ] + [
        "/// Get the [`Actor`]s in the group",
        "pub const fn actors(&self) -> &'static [Actor] {",
        "match self {",
    ] + actors_lines + [
        "}}",
        "/// Check if any actor in the group is only holdable with Prompt Entanglement (PE)",
        "#[cfg(feature = \"prompt-entanglement\")]",
        "pub const fn any_pe_only(&self) -> bool {",
        "match self {",
    ] + any_pe_only_lines + [
        "}}",
        "/// Check if all actors in the group are only holdable with Prompt Entanglement (PE)",
        "#[cfg(feature = \"prompt-entanglement\")]",
        "pub const fn all_pe_only(&self) -> bool {",
        "match self {",
    ] + all_pe_only_lines + [
        "}}}",
    ] + generate_rust_count_macro("group", len(groups) + 1)

    return write_rust_source(src_file("group.rs"), lines)

def generate_actor(
    actors: list[str],
    english_names: dict[str, str],
    groups: list[tuple[str, list[str]]],
    actor_to_group_idx: dict[str, int],
    pe_only_actors: set[str]
):
    # note for input items we don't sort, but use the inventory sorting order
    # as defined in seed-actors.yaml
    enum_lines = [
        "/// \"Empty\" slot in recipe input",
        "#[default]",
        "None,",
    ]
    english_name_lines = [
        "Self::None => \"<none>\",",
    ]
    actor_name_lines = [
        "Self::None => \"\",",
    ]
    actor_to_group_lines = [
        "Self::None => Group::None,",
    ]
    actor_pe_only_lines = [
    ]
    from_actor_name_lines = []

    progress = spp.printer(len(actors), "Generate Actor")
    for (i, actor) in enumerate(actors):
        progress.print(i, actor)
        english = english_names[actor]
        enum_lines.append(f"    /// {english}")
        enum_lines.append(f"    {actor},")
        english_name_lines.append(f"Self::{actor} => \"{english}\",")
        actor_name_lines.append(f"Self::{actor} => \"{actor}\",")
        from_actor_name_lines.append(f"    \"{actor}\" => Actor::{actor},")
        actor_to_group_lines.append(f"Self::{actor} => Group::{groups[actor_to_group_idx[actor]][0]},")
        if actor in pe_only_actors:
            actor_pe_only_lines.append(f"Self::{actor} => true,")
    progress.done()

    actor_pe_only_lines.append("_ => false")

    lines = [
        "use crate::Group;",
        "",
        "/// Cookable Item (Input of cooking pot)",
        f"#[cfg_attr(feature = \"actor-enum-map\", derive(enum_map::Enum))]",
        f"#[cfg_attr(feature = \"actor-enum-set\", derive(enumset::EnumSetType, PartialOrd, Ord, Hash))]",
        f"#[cfg_attr(not(feature = \"actor-enum-set\"), derive({",".join(COMMON_ENUM_DERIVES)}))]",
        "#[derive(Default)]",
        "#[allow(non_camel_case_types)]",
        "#[repr(u8)]",
        "pub enum Actor {",
    ] + enum_lines + [
        "}",
        "impl Actor {",
    ] + [
        "/// Get the [`Group`] of the actor",
        "pub const fn group(&self) -> Group {",
        "match self {",
    ] + actor_to_group_lines + [
        "}}",
        "/// Check if the actor is only holdable with Prompt Entanglement (PE)",
        "#[cfg(feature = \"prompt-entanglement\")]",
        "pub const fn pe_only(&self) -> bool {",
        "match self {",
    ] + actor_pe_only_lines + [
        "}}",
        "/// Get the English name of the input item actor",
        "#[cfg(feature = \"actor-english\")]",
        "pub const fn name(&self) -> &'static str {",
        "match self {",
    ] + english_name_lines + [
        "}}",
        "/// Get the actor name of the input item",
        "#[cfg(feature = \"actor-to-actor\")]",
        "pub const fn actor_name(&self) -> &'static str {",
        "match self {",
    ] + actor_name_lines + [
        "}}",
        "/// Get the input item from an actor name",
        "#[cfg(feature = \"actor-from-actor\")]",
        "pub fn from_actor_name(name: &str) -> Option<Self> {",
        "ACTOR_NAME_MAP.get(name).copied()",
        "}}",
        "#[cfg(feature = \"actor-from-actor\")]",
        "static ACTOR_NAME_MAP: phf::Map<&'static str, Actor> = phf::phf_map! {",
    ] + from_actor_name_lines + [
        "};",
    ] + generate_rust_count_macro("actor", len(actors) + 1)

    return write_rust_source(src_file("actor.rs"), lines)

def generate_cook_item():
    with open(output_file("recipe-meta.yaml"), "r", encoding="utf-8") as f:
        recipe_meta = yaml.safe_load(f)
    output_actors = recipe_meta["output_actors"]
    enum_lines = []
    english_name_lines = []
    actor_name_lines = []
    from_actor_name_lines = []

    progress = spp.printer(len(output_actors), "Generate CookItem")

    for (i, actor) in enumerate(output_actors):
        progress.print(i, actor)
        with open(output_file("Actor", f"{actor}.yaml"), "r", encoding="utf-8") as f:
            data = yaml.safe_load(f)
        english = get_actor_english_name(actor, data)
        # not including the {{effect}} prefix, to minimize binary size
        if not english.startswith("{{effect}}"):
            raise ValueError(f"Invalid English name for {actor}: {english}, should start with {{{{effect}}}}")
        english = english[10:]
        enum_lines.append(f"/// {english}")
        enum_lines.append(f"{actor},")
        english_name_lines.append(f"Self::{actor} => \"{english}\",")
        actor_name_lines.append(f"Self::{actor} => \"{actor}\",")
        from_actor_name_lines.append(f"    \"{actor}\" => CookItem::{actor},")
    progress.done()

    lines = [
        "/// Cooked Item (Output of cooking pot)",
        f"#[cfg_attr(feature = \"cook-item-enum-map\", derive(enum_map::Enum))]",
        f"#[cfg_attr(feature = \"cook-item-enum-set\", derive(enumset::EnumSetType, PartialOrd, Ord, Hash))]",
        f"#[cfg_attr(not(feature = \"cook-item-enum-set\"), derive({",".join(COMMON_ENUM_DERIVES)}))]",
        "#[allow(non_camel_case_types)]",
        "#[repr(u8)]",
        "pub enum CookItem {",
    ] + enum_lines + [
        "}",
        "impl CookItem {",
    ] + [
        "/// Get the English name of the cook item actor",
        "#[cfg(feature = \"cook-item-english\")]",
        "pub const fn name(&self) -> &'static str {",
        "match self {",
    ] + english_name_lines + [
        "}}",
        "/// Get the actor name of the cook item",
        "#[cfg(feature = \"cook-item-to-actor\")]",
        "pub const fn actor_name(&self) -> &'static str {",
        "match self {",
    ] + actor_name_lines + [
        "}}",
        "/// Get the cook item from an actor name",
        "#[cfg(feature = \"cook-item-from-actor\")]",
        "pub fn from_actor_name(name: &str) -> Option<Self> {",
        "ACTOR_NAME_MAP.get(name).copied()",
        "}}",
        "#[cfg(feature = \"cook-item-from-actor\")]",
        "static ACTOR_NAME_MAP: phf::Map<&'static str, CookItem> = phf::phf_map! {",
    ] + from_actor_name_lines + [
        "};",
    ] + generate_rust_count_macro("cook_item", len(output_actors))

    return write_rust_source(src_file("cook_item.rs"), lines)

def generate_rust_count_macro(enum_name: str, count: int) -> list[str]:
    return [
        f"/// Get the count of the {enum_name} enum",
        "///",
        "/// `count - 1` is the last valid enum variant",
        "#[macro_export]",
        f"macro_rules! {enum_name}_count {{",
        f"() => {{ {count} }}",
        "}",
    ]

def write_rust_source(path: str, lines: list[str]) -> str:
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(HEADER)
        for line in lines:
            f.write(line)
            f.write("\n")
    return path

def run_rustfmt(paths: list[str]):
    print("Running rustfmt")
    rustfmt = shutil.which("rustfmt")
    if rustfmt:
        subprocess.run([rustfmt] + paths, check=True)

def src_file(*args):
    return os.path.join(os.path.dirname(__file__), "src", *args)


def load_actor_data() -> tuple[
    set[str], # pe_only_actors
    dict[str, ActorData], # actor_data
    dict[str, str] # english name
]:
    pe_only_actors = set(["Obj_DRStone_Get"])
    actor_data = {}
    english_name = {}

    with open(output_file("recipe-actor-index.yaml"), "r", encoding="utf-8") as f:
        recipe_actor_index = yaml.safe_load(f)

    with open(project_file("data", "seed-actors.yaml"), "r", encoding="utf-8") as f:
        actors: list[str] = yaml.safe_load(f)
    with open(output_file("gpks.yaml"), "r", encoding="utf-8") as f:
        gpl_defaults = yaml.safe_load(f)
    recipe_tags = set(load_recipe_tags())
    important_tags = set(load_tags())

    progress = spp.printer(len(actors), "Load actor data")

    for (i, actor) in enumerate(actors):
        progress.print(i, actor)
        actor_path = output_file("Actor", f"{actor}.yaml")
        with open(actor_path, "r", encoding="utf-8") as f:
            data = yaml.safe_load(f)

        tags = data["tags"]
        actor_impo_tags = [ tag for tag in tags if tag in important_tags ]
        actor_recipe_tags = [ tag for tag in actor_impo_tags if tag in recipe_tags ]
        if not actor_recipe_tags:
            actor_recipe_tag = "None"
        elif len(actor_recipe_tags) == 1:
            actor_recipe_tag = actor_recipe_tags[0]
        else:
            raise ValueError(f"Actor {actor} has multiple recipe tags: {actor_recipe_tags}")

        a1, a2, a3 = recipe_actor_index.get(actor, [0, 0, 0])

        gpl = data["gparamlist"]
        def gplget(key: str):
            return gpl.get(key, gpl_defaults[key])
        the_data = ActorData(
            effect = gplget("cureItemEffectType"),
            recipe_tag = actor_recipe_tag,
            boost_effect_time = gplget("cookSpiceBoostEffectiveTime"),
            boost_hp = gplget("cookSpiceBoostHitPointRecover"),
            boost_max_hearts = gplget("cookSpiceBoostMaxHeartLevel"),
            boost_stamina = gplget("cookSpiceBoostStaminaLevel"),
            boost_success_rate = gplget("cookSpiceBoostSuccessRate"),
            effect_level = gplget("cureItemEffectLevel"),
            effect_time = gplget("cureItemEffectiveTime"),
            hp = gplget("cureItemHitPointRecover"),
            buy_price = gplget("itemBuyingPrice"),
            sell_price = gplget("itemSellingPrice"),
            tags = actor_impo_tags,
            matchable_recipes = (a1, a2, a3)
        )
        actor_data[actor] = the_data

        english_name[actor] = get_actor_english_name(actor, data)

        if actor.startswith("dyecolor_"):
            pe_only_actors.add(actor)
            continue
        if actor.startswith("Obj_Photo"):
            pe_only_actors.add(actor)
            continue
        for tag in tags:
            # Icy are also tagged with Roast*
            if tag.startswith("Roast"):
                pe_only_actors.add(actor)
                break
    progress.done()
    return pe_only_actors, actor_data, english_name

def get_actor_english_name(actor: str, data) -> str:
    if actor.startswith("dyecolor_"):
        idx = int(actor[9:])
        return [
            "Dye",
            "Blue",
            "Red",
            "Yellow",
            "White",
            "Black",
            "Purple",
            "Green",
            "Light Blue",
            "Navy",
            "Orange",
            "Peach",
            "Crimson",
            "Light Yellow",
            "Brown",
            "Gray",
        ][idx]
    localization = data["localization"]
    if not localization:
        return ""
    return localization["en-US"]["name"]["text"]


def load_actors_and_groups() -> tuple[
    list[str], # list of actors
    list[tuple[str, list[str]]], # list of groups (name, actors)
    dict[str, int] # actor_to_group_idx
]:
    """
        Load the list of actors and put them into their groups

        The list of groups depends on the order of the actors:
        - If Actor A is after Actor B, then Group of A is after or equal to Group of B

        The order of actors in a group also follows the same order as the overall
        actor list

    """

    # This is the grouping of ALL actors, including ones not in seed-actors.yaml
    with open(output_file("recipe-groups.yaml"), "r", encoding="utf-8") as f:
        absolute_groups: list[list[str]] = yaml.safe_load(f)
    with open(project_file("data", "seed-actors.yaml"), "r", encoding="utf-8") as f:
        actors: list[str] = yaml.safe_load(f)

    actor_to_abs_group_idx: dict[str, int] = {}
    for (i, group) in enumerate(absolute_groups):
        for actor in group:
            actor_to_abs_group_idx[actor] = i

    output_abs_group_idxes: list[int] = []
    abs_group_idx_to_actors: dict[int, list[str]] = {}
    progress = spp.printer(len(actors), "Load actors and groups")
    for (i, actor) in enumerate(actors):
        progress.print(i, actor)
        absolute_group_idx = actor_to_abs_group_idx.get(actor)
        if absolute_group_idx is None:
            raise ValueError(f"Actor {actor} not found in any group")
        if absolute_group_idx not in output_abs_group_idxes:
            output_abs_group_idxes.append(absolute_group_idx)
            abs_group_idx_to_actors[absolute_group_idx] = [actor]
        else:
            abs_group_idx_to_actors[absolute_group_idx].append(actor)
    progress.done()

    actor_to_group_idx = {}
    for actor in actors:
        abs_group_idx = actor_to_abs_group_idx[actor]
        actor_to_group_idx[actor] = output_abs_group_idxes.index(abs_group_idx)

    groups = []
    for (group_idx, abs_group_idx) in enumerate(output_abs_group_idxes):
        actors_in_group = abs_group_idx_to_actors[abs_group_idx]
        group_name = make_group_name(actors_in_group, group_idx)
        groups.append((group_name, actors_in_group))

    return actors, groups, actor_to_group_idx

def load_tags() -> list[str]:
    return list(sorted(set(load_recipe_tags() + EXTRA_TAGS)))

def load_recipe_tags() -> list[str]:
    with open(output_file("recipe-meta.yaml"), "r", encoding="utf-8") as f:
        recipe_meta = yaml.safe_load(f)
    return recipe_meta["tags_used_for_matching"]

def make_group_name(actors: list[str], output_group_idx: int):
    if len(actors) == 1:
        return actors[0]
    name = os.path.commonprefix(actors)
    while name and name[-1] != "_":
        name = name[:-1]
    # add 1 to align with the value of the enum
    return name +"Grp_"+ str(output_group_idx+1)

def output_file(*args):
    return script_home("output", *args)

def script_home(*args):
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), "research-scripts", *args)

def project_file(*args):
    return os.path.join(os.path.dirname(__file__), *args)

if __name__ == "__main__":
    sys.path.append(script_home("src"))
    import spp
    main()
