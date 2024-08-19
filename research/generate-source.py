# Generate downstream source code
import yaml
import util
import os
import subprocess

IN = [
    "data/*.yaml",
    "output/*.yaml",
]
OUT = [
    "../app/data/src/generated/constants.rs",
    "../app/data/src/generated/group.rs",
    "../app/data/src/generated/actor.rs",
]
util.print_stage(__file__, IN, OUT)

HEADER = """
//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

"""

def write_doc_comment(o, comments):
    for comment in comments.split("\n"):
        o.write(f"/// {comment.strip()}\n")

def make_group_name(actors, group_id):
    if len(actors) == 1:
        return actors[0]
    name = os.path.commonprefix(actors)
    while name and name[-1] != "_":
        name = name[:-1]
    return name +"Grp_"+ group_id

def gen_numeric_constants(o, data):
    o.write(HEADER)
    write_doc_comment(o, """
        Number of "ingredients" in a recipe

        Equivalent actors are grouped together. This also includes the "<none>" ingredient,
        which indicates empty space (for example, a recipe with 4 items has 1 empty space).
    """)
    o.write(f"pub const NUM_GROUPS: usize = {data['num']};\n\n")
    write_doc_comment(o, """
        Number of ingredients in a recipe record. Always 5
    """)
    o.write(f"pub const NUM_INGR: usize = 5;\n\n")
    write_doc_comment(o, """
        Number of total recipe records

        This is choosing NUM_INGR from NUM_GROUPS, allowing for repetition.
        In other words, binomial(NUM_GROUPS+NUM_INGR-1, NUM_INGR),
        or equivalently, NUM_GROUPS multichoose NUM_INGR.
    """)
    o.write(f"pub const NUM_TOTAL_RECORDS: usize = {data['total']};\n\n")

def gen_group_enum(o, actor_to_name, groups):
    o.write(HEADER)
    o.write("use super::Actor;\n")
    write_doc_comment(o, "Recipe input groups")
    o.write("#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]\n")
    o.write("#[allow(non_camel_case_types)]\n")
    o.write("#[repr(usize)]\n")
    o.write("pub enum Group {\n")
    o.write("    #[default]\n")
    o.write("    None = 0,\n")
    group_names = {}
    for i in range(1, len(groups)):
        id = str(i)
        group = groups[id]
        name = make_group_name(group, id)
        group_names[id] = name
        comment = ", ".join([actor_to_name[actor] for actor in group])
        o.write(f"    /// {comment}\n")
        o.write(f"    {name} = {id},\n")
    o.write("}\n")
    o.write("impl Group {\n")

    o.write("    #[inline] pub const fn id(&self) -> usize { *self as usize }\n")
    o.write("    #[inline] pub const fn from_id_unchecked(id: usize) -> Self { unsafe { std::mem::transmute(id) } }\n")

    write_doc_comment(o, "Get the first actor in the group")
    o.write("pub const fn first_actor(&self) -> Actor {\n")
    o.write("match self {\n")
    o.write("Self::None => Actor::None,\n")
    for i in range(1, len(groups)):
        id = str(i)
        actor = groups[id][0]
        name = group_names[id]
        o.write(f"Self::{name} => Actor::{actor},\n")

    o.write("}}\n")

    o.write("}\n")

def gen_actor_enum(o, actor_to_name, groups):
    o.write(HEADER)
    o.write("use super::Group;\n")
    write_doc_comment(o, "Ingredients (actors)")
    o.write("#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]\n")
    o.write("#[allow(non_camel_case_types)]\n")
    o.write("pub enum Actor {\n")
    o.write("    #[default]\n")
    o.write("    None,\n")
    for actor in actor_to_name:
        name = actor_to_name[actor]
        o.write(f"    /// {name}\n")
        o.write(f"    {actor},\n")
    o.write("}\n")
    o.write("impl Actor {\n")
    o.write("    pub const fn name(&self) -> &'static str {\n")
    o.write("        match self {\n")
    o.write("            Self::None => \"<none>\",\n")
    for actor in actor_to_name:
        name = actor_to_name[actor]
        o.write(f"Self::{actor} => \"{name}\",\n")
    o.write("}}\n")

    o.write("    pub const fn group(&self) -> Group {\n")
    o.write("        match self {\n")
    o.write("            Self::None => Group::None,\n")
    for i in range(1, len(groups)):
        id = str(i)
        group = groups[id]
        group_name = make_group_name(group, id)
        for actor in group:
            o.write(f"Self::{actor} => Group::{group_name},\n")
    o.write("}}\n")

    write_doc_comment(o, """
        Convert item name to actor with case-insenstive comparison

        `<none>` will return `Some(Actor::None)`, while invalid names will return `None`.
    """)
    o.write("    pub fn try_from<S: AsRef<str>>(s: S) -> Option<Self> {\n")
    o.write("        match s.as_ref().to_ascii_lowercase().as_str() {\n")
    o.write("            \"<none>\" => Some(Actor::None),\n")
    for actor in actor_to_name:
        name = actor_to_name[actor].lower()
        o.write(f"\"{name}\" => Some(Actor::{actor}),\n")
    o.write("_ => None,\n")
    o.write("}}\n")

    o.write("}\n")

    o.write("impl std::fmt::Debug for Actor {\n")
    o.write("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n")
    o.write("        f.debug_tuple(\"Actor\").field(&self.name()).finish()\n")
    o.write("    }\n")
    o.write("}\n")

    # for i in range(1, len(groups)):
    #     id = str(i)
    #     group = groups[id]

with open("output/actor-names.yaml", "r", encoding="utf-8") as f:
    actors = []
    actor_to_name = {}
    for actor, name in yaml.safe_load(f):
        actors.append(actor)
        actor_to_name[actor] = name

with open("output/ids.yaml", "r", encoding="utf-8") as f:
    data = yaml.safe_load(f)

with open(OUT[0], "w", encoding="utf-8") as f:
    gen_numeric_constants(f, data)

with open(OUT[1], "w", encoding="utf-8") as f:
    gen_group_enum(f, actor_to_name, data["ids"])

with open(OUT[2], "w", encoding="utf-8") as f:
    gen_actor_enum(f, actor_to_name, data["ids"])

subprocess.run(["rustfmt"] + OUT, check=True)
