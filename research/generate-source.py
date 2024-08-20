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
    "../dump/console/src/generated.cpp",
    "../dump/console/src/generated.hpp",
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
    lines = [l.strip() for l in comments.split("\n")]
    if not lines:
        return
    i = 0
    while i < len(lines) and not lines[i]:
        i += 1
    for comment in lines[i:]:
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
    def print_field(name, value):
        print(f"{name:<20} = {value}")
    print_field("num_groups", data['num'])
    write_doc_comment(o, """
        Number of "ingredients" in a recipe

        Equivalent actors are grouped together. This also includes the "<none>" ingredient,
        which indicates empty space (for example, a recipe with 4 items has 1 empty space).
    """)
    o.write(f"pub const NUM_GROUPS: usize = {data['num']};\n\n")
    print_field("num_ingr", 5)
    write_doc_comment(o, """
        Number of ingredients in a recipe record. Always 5
    """)
    o.write(f"pub const NUM_INGR: usize = 5;\n\n")
    total = data['total']
    print_field("total", data['total'])
    write_doc_comment(o, """
        Number of total recipe records

        This is choosing NUM_INGR from NUM_GROUPS, allowing for repetition.
        In other words, binomial(NUM_GROUPS+NUM_INGR-1, NUM_INGR),
        or equivalently, NUM_GROUPS multichoose NUM_INGR.
    """)
    o.write(f"pub const NUM_TOTAL_RECORDS: usize = {data['total']};\n\n")
    chunk_size, chunk_count, last_chunk_size = util.chunk(total)
    print_field("chunk_size", chunk_size)
    write_doc_comment(o, """
        Number of records in each chunk except last in the data dump
    """)
    o.write(f"pub const CHUNK_SIZE: usize = {chunk_size};\n\n")
    # chunk_count is wrong is total is a multiple of chunk_size
    util.assertion(total % chunk_size != 0, "total divisible by chunk size")
    print_field("chunk_count", chunk_count)
    write_doc_comment(o, """
        Number of chunks in the data dump
    """)
    o.write(f"pub const CHUNK_COUNT: usize = {chunk_count};\n\n")
    print_field("last_chunk_size", last_chunk_size)
    write_doc_comment(o, """
        Number of records in the last chunk in the data dump
    """)
    o.write(f"pub const LAST_CHUNK_SIZE: usize = {last_chunk_size};\n\n")
    write_doc_comment(o, """
        Pre-computed multichoose(n, k) values for 0<=n<=NUM_GROUPS and 0<=k<=NUM_INGR

        MULTICHOOSE[n][k] is the number of ways to choose k items from n items with repetition.
    """)
    o.write("pub const MULTICHOOSE: [[usize; NUM_INGR+1]; NUM_GROUPS+1] = [\n")
    multichoose = util.make_multichoose(data['num'])
    for multichoose_n in multichoose:
        o.write("[")
        for k in multichoose_n:
            o.write(f"{k}, ")
        o.write("],\n")
    o.write("];\n")

def gen_numeric_constants_cpp(o, hpp, data):
    o.write(HEADER)
    hpp.write(HEADER)
    hpp.write("#pragma once\n")
    hpp.write("#include <cstdint>\n")
    o.write("#include \"generated.hpp\"\n")
    hpp.write(f"#define NUM_GROUPS {data['num']}\n")
    hpp.write("#define NUM_INGR 5\n")
    hpp.write(f"#define NUM_TOTAL_RECORDS {data['total']}\n")
    chunk_size, chunk_count, last_chunk_size = util.chunk(data['total'])
    hpp.write(f"#define CHUNK_SIZE {chunk_size}\n")
    hpp.write(f"#define CHUNK_COUNT {chunk_count}\n")
    hpp.write(f"#define LAST_CHUNK_SIZE {last_chunk_size}\n")
    hpp.write("namespace botw::rdump {\n")
    o.write("namespace botw::rdump {\n")
    o.write("static uint64_t MULTICHOOSE[NUM_GROUPS+1][NUM_INGR+1] = {\n")
    multichoose = util.make_multichoose(data['num'])
    rows = []
    for multichoose_n in multichoose:
        k_str = ", ".join(str(k) for k in multichoose_n)
        rows.append(f"    {{ {k_str} }}")
    o.write(",\n".join(rows))
    o.write("};\n")
    hpp.write("uint64_t multichoose(uint64_t n, uint64_t k);\n")
    o.write("uint64_t multichoose(uint64_t n, uint64_t k) {\n")
    o.write("    return MULTICHOOSE[n][k];\n")
    o.write("}\n")
    hpp.write("const char* actor_name(uint64_t group);\n")
    o.write("const char* actor_name(uint64_t group) {\n")
    o.write("    switch (group) {\n")
    o.write("    case 0: return \"\";\n")
    groups = data['ids']
    for i in range(1, len(groups)):
        id = str(i)
        actor = groups[id][0]
        o.write(f"    case {id}: return \"{actor}\";\n")
    o.write("    default: return \"\";\n")
    o.write("    }\n")
    o.write("}\n")

    hpp.write("}\n")
    o.write("}\n")



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

print("generating files")

with open("output/ids.yaml", "r", encoding="utf-8") as f:
    data = yaml.safe_load(f)

with open(OUT[0], "w", encoding="utf-8") as f:
    gen_numeric_constants(f, data)

with open(OUT[1], "w", encoding="utf-8") as f:
    gen_group_enum(f, actor_to_name, data["ids"])

with open(OUT[2], "w", encoding="utf-8") as f:
    gen_actor_enum(f, actor_to_name, data["ids"])

with open(OUT[3], "w", encoding="utf-8") as o:
    with open(OUT[4], "w", encoding="utf-8") as hpp:
        gen_numeric_constants_cpp(o, hpp, data)

print("running rustfmt")
subprocess.run(["rustfmt"] + [x for x in OUT if x.endswith(".rs")], check=True)