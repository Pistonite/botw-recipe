# Check botw-data message pack to get the English names for the ingredient actors
import yaml
import util

IN = [
    "data/inventory-actors.yaml",
    "botw-data/Message/Msg_USen.product.sarc/ActorType/Item.msyt",
    "botw-data/Message/Msg_USen.product.sarc/ActorType/CapturedActor.msyt",
    "botw-data/Message/Msg_USen.product.sarc/ActorType/PlayerItem.msyt",
]
OUT = ["output/actor-names.yaml"]
util.print_stage(__file__, IN, OUT)

with open(IN[0], "r", encoding="utf-8") as f:
    actors = yaml.safe_load(f)

items = {}

for file in IN[1:]:
    with open(file, "r", encoding="utf-8") as f:
        x = yaml.safe_load(f)["entries"]
        for n in x:
            items[n] = x[n]

output = []

for actor in util.progress(actors, "process actors"):
    key = actor+"_Name"
    if key not in items:
        name = actor
    else:
        data = items[actor+"_Name"]["contents"]
        if len(data) != 1:
            raise ValueError(f"{actor} has {len(data)} names!??")
        name = data[0]["text"]
    output.append((actor, name))

with open(OUT[0], "w", encoding="utf-8") as f:
    for actor, name in util.progress(output, "save output"):
        f.write(f"- [{actor:<20}, {name}]\n")