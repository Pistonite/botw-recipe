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

# Save System
KEYWORDS = """
# Keys are based on guesses from the data and decomp
# CEI:    cook_effect_index
# BT:     base_time
# MR:     multiplier
# Ma:     max
# Mi:     min
# SSA:    super_success_amount
# T:      type
# FA:     failure_actor
# FALR:   faliure_actor_life_recover
# FALRMR: faliure_actor_life_recover_multiplier
# FCA:    fairy_cook_actor
# LRMR:   life_recover_multiplier
# MEA:    monster_extract_actor
# NMMR:   num_matrial_multiplier
# NMSSR:  num_material_super_success_rate
# SFALR:  stone_food_actor_life_recover
# SSAET:  super_success_additional_effect_time
"""

with open(OUT["system"], "w", encoding="utf-8", newline="\n") as f:
    f.write(KEYWORDS)
    f.write("cook_effect_index:\n")
    for entry in system["CEI"]:
        f.write(f"  - {{ type: {entry["T"] + ",":<24} base_time: {entry["BT"]:<10},\n")
        f.write(f"      max: {entry["Ma"]:<3}, min: {entry["Mi"]:<3}, super_success_amount: {entry["SSA"]:<10},\n")
        f.write(f"      multiplier: {entry["MR"]:<39} }}\n")
    f.write(f"failure_actor: {system["FA"]}\n")
    f.write(f"failure_actor_life_recover: {system["FALR"]}\n")
    f.write(f"failure_actor_life_recover_multiplier: {system["FALRMR"]}\n")
    f.write(f"fairy_cook_actor: {system["FCA"]}\n")
    f.write(f"life_recover_multiplier: {system["LRMR"]}\n")
    f.write(f"monster_extract_actor: {system["MEA"]}\n")
    f.write(f"num_material_multiplier: {system["NMMR"]}\n")
    f.write(f"num_material_super_success_rate: {system["NMSSR"]}\n")
    f.write(f"stone_food_actor_life_recover: {system["SFALR"]}\n")
    f.write(f"super_success_additional_effect_time: {system["SSAET"]}\n")
