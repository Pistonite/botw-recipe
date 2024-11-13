"""
    Process recipes and remove the ones that are uncookable 
    (i.e. has monster extract)
"""

import json
import yaml
import util

IN = [
    "cooking-rs/src/cook_recipes.json",
    "data/known-filtered-actors.yaml",
    "botw-data/Message/Msg_USen.product.sarc/ActorType/CookResult.msyt",
]
OUT = [
    util.output("recipes.yaml"), 
    util.output("recipe-actors.yaml"),
    util.output("recipes-with-monster.yaml"), 
]
util.print_stage(__file__, IN, OUT)

with open(IN[0], "r", encoding="utf-8") as f:
    recipes = json.load(f)

with open(IN[1], "r", encoding="utf-8") as f:
    filter = set(yaml.load(f, Loader=yaml.FullLoader))

items = {}
with open(IN[2], "r", encoding="utf-8") as f:
    x = yaml.safe_load(f)["entries"]
    for n in x:
        items[n] = x[n]

name_to_actor = {}
for entry in util.progress(items, "process lang entries"):
    if not entry.endswith("_Name"):
        continue
    actor = entry[:-5]
    contents = items[entry]["contents"]
    for x in contents:
        if "text" in x:
            name = x["text"]
            if name in name_to_actor:
                if name_to_actor[name] != actor:
                    raise ValueError(f"conflicting actors for {name}")
            name_to_actor[name] = actor
            break

all_recipes = []
filtered_recipes = []
output_actor_to_name = {}
for recipe in util.progress(recipes, "filter recipes"):
    filtered = False
    for actors in recipe["actors"]:
        if not isinstance(actors, list):
            actors = [actors]
        for actor in actors:
            if actor in filter:
                filtered = True
                break
        if filtered:
            break
    name = recipe["name"]
    if name not in name_to_actor:
        raise ValueError(f"no actor for {name}")
    actor = name_to_actor[name]
    recipe["name"] = actor
    if not filtered:
        output_actor_to_name[actor] = name
        filtered_recipes.append(recipe)
    all_recipes.append(recipe)

with open(OUT[2], "w", encoding="utf-8", newline="\n") as f:
    yaml.dump(all_recipes, f)

with open(OUT[0], "w", encoding="utf-8", newline="\n") as f:
    yaml.dump(filtered_recipes, f)

print(f"filtered {len(recipes) - len(filtered_recipes)} recipes")
with open(OUT[1], "w", encoding="utf-8", newline="\n") as f:
    yaml.dump(output_actor_to_name, f, sort_keys=True)
