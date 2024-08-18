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
    "../app/data/src/generated.rs"
]
util.print_stage(__file__, IN, OUT)

def gen_numeric_constants(o, data):
    print("num_groups = ", data["num"])
    o.write(f"pub const NUM_GROUPS: usize = {data['num']};\n")
    print("total = ", data["total"])
    o.write(f"pub const NUM_TOTAL_RECORDS: usize = {data['total']};\n")

def gen_group_enum(o, actor_to_name, groups):
    o.write("#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]\n")
    o.write("#[allow(non_camel_case_types)]\n")
    o.write("#[repr(usize)]\n")
    o.write("pub enum Group {\n")
    o.write("    #[default]\n")
    o.write("    None = 0,\n")
    for i in range(1, len(groups)):
        id = str(i)
        group = groups[id]
        if len(group) == 1:
            name = group[0]
        else:
            name = os.path.commonprefix(group)
            name = "Grp_" + name + id
        comment = ", ".join([actor_to_name[actor] for actor in group])
        o.write(f"    /// {comment}\n")
        o.write(f"    {name} = {id},\n")
    o.write("}\n")
    o.write("impl Group {\n")
    o.write("    #[inline] pub fn id(&self) -> usize { *self as usize }\n")
    o.write("    #[inline] pub fn from_id_unchecked(id: usize) -> Self { unsafe { std::mem::transmute(id) } }\n")
    o.write("}\n")


#     for actor in actors:

with open("output/actor-names.yaml", "r", encoding="utf-8") as f:
    actors = []
    actor_to_name = {}
    for actor, name in yaml.safe_load(f):
        actors.append(actor)
        actor_to_name[actor] = name

with open("output/ids.yaml", "r", encoding="utf-8") as f:
    data = yaml.safe_load(f)

with open(OUT[0], "w", encoding="utf-8") as f:
    f.write("//! Automatically generated. DO NOT EDIT\n\n")
    gen_numeric_constants(f, data)
    gen_group_enum(f, actor_to_name, data["ids"])

subprocess.run(["rustfmt", OUT[0]], check=True)
