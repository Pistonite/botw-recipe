"""Decode the CookData parameter file to cookdata-recipes.yaml"""
import yaml
import util

IN = {
    "crc32": util.output("crc32.yaml"),
    "cookdata": "botw-data/Cooking/CookData.yml"
}

OUT = {
    "recipes": util.output("cookdata-recipes.yaml"),
    "system":  util.output("cookdata-system.yaml")
}
util.print_stage(__file__, IN, OUT)
util.extend_yaml()

with open(IN["cookdata"], "r", encoding="utf-8") as f:
    cook_data = yaml.load(f, Loader=yaml.FullLoader)

crc32 = {}
with open(IN["crc32"], "r", encoding="utf-8") as f:
    hm = yaml.safe_load(f)
    for key, data in hm.items():
        crc32[int("0x"+key,0)] = data

def decode(obj, typ):
    if isinstance(obj, list):
        return [decode(o, typ) for o in obj]
    util.assertion(isinstance(obj, int), f"expecting crc32 hash to be an integer! got {obj}")
    util.assertion(obj in crc32, f"missing identifier for crc32 hash: 0x{obj:08x}")
    entry = crc32[obj]
    if typ in entry:
        return entry[typ]
    actual_typ = next(k for k in entry)
    raise ValueError(f"expecting identifier of type {typ}, actual type is {actual_typ}")

recipes = []
for data in util.progress(cook_data["Recipes"], "decode recipes", len(cook_data["Recipes"])):
    util.assertion("Recipe" in data)
    name = decode(data["Recipe"], "actor")
    actors = []
    if "Actors" in data:
        actors = decode(data["Actors"], "actor")
    tags = []
    if "Tags" in data:
        tags = decode(data["Tags"], "tag")
    hb = 0
    if "HB" in data:
        hb = data["HB"]
    util.assertion(isinstance(hb, int), f"HB must be int, got: {hb}")
    recipes.append({
        "actors": actors,
        "tags": tags,
        "hb": hb,
        "name": name,
        "num": 0
    })
for data in cook_data["SingleRecipes"]:
    util.assertion("Recipe" in data)
    name = decode(data["Recipe"], "actor")
    actors = []
    if "Actors" in data:
        actors = decode(data["Actors"], "actor")
    tags = []
    if "Tags" in data:
        tags = decode(data["Tags"], "tag")
    hb = 0
    if "HB" in data:
        hb = data["HB"]
    util.assertion(isinstance(hb, int), f"HB must be int, got: {hb}")
    util.assertion(data["Num"] == 1)
    recipes.append({
        "actors": actors,
        "tags": tags,
        "hb": hb,
        "name": name,
        "num": 1
    })
system = cook_data["System"]
for entry in system["CEI"]:
    entry["T"] = decode(entry["T"], "effect")

with open(OUT["recipes"], "w", encoding="utf-8", newline="\n") as f:
    yaml.dump(recipes, f)

with open(OUT["system"], "w", encoding="utf-8", newline="\n") as f:
    yaml.dump(system, f)
