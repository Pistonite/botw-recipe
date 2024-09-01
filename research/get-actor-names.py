# Check botw-data message pack to get the English names for the ingredient actors
import yaml
import util

IN = {
    "inventory-actors": "data/inventory-actors.yaml",
    "msg1": "botw-data/Message/Msg_USen.product.sarc/ActorType/Item.msyt",
    "msg2": "botw-data/Message/Msg_USen.product.sarc/ActorType/CapturedActor.msyt",
    "msg3": "botw-data/Message/Msg_USen.product.sarc/ActorType/PlayerItem.msyt", 
}
OUT = {
    "actor-names" :"output/actor-names.yaml"
}
util.print_stage(__file__, IN, OUT)

with open(IN["inventory-actors"], "r", encoding="utf-8") as f:
    actors = yaml.safe_load(f)

items = {}

for file in [IN[x] for x in IN if x.startswith("msg")]:
    with open(file, "r", encoding="utf-8") as f:
        x = yaml.safe_load(f)["entries"]
        for n in x:
            items[n] = x[n]

output = []

for actor in util.progress(actors, "process actors"):
    # hard-coded dye name
    if actor == "dyecolor_00":
        name = "Dye"
    else:
        key = actor+"_Name"
        data = items[actor+"_Name"]["contents"]
        if len(data) != 1:
            raise ValueError(f"{actor} has {len(data)} names!??")
        name = data[0]["text"]
    output.append((actor, name))

with open(OUT["actor-names"], "w", encoding="utf-8", newline="\n") as f:
    for actor, name in util.progress(output, "save output"):
        f.write(f"- [{actor:<20}, {name}]\n")
