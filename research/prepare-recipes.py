# Process recipes and remove the ones that are uncookable (i.e. has monster extract)

import json
import yaml
import util

IN = [
    "cooking-rs/src/cook_recipes.json",
    "data/known-filtered-actors.yaml",
]
OUT = ["output/recipes.yaml"]
util.print_stage(__file__, IN, OUT)

with open(IN[0], "r", encoding="utf-8") as f:
    recipes = json.load(f)

with open(IN[1], "r", encoding="utf-8") as f:
    filter = set(yaml.load(f, Loader=yaml.FullLoader))

filtered_recipes = []
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
    if not filtered:
        filtered_recipes.append(recipe)

with open(OUT[0], "w", encoding="utf-8") as f:
    yaml.dump(filtered_recipes, f)

print(f"filtered {len(recipes) - len(filtered_recipes)} recipes")
