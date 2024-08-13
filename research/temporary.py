import yaml
import json


with open("data/inventory-names.yaml", "r", encoding="utf-8") as f:
    names = yaml.safe_load(f)

with open("botw-tools/botw_names.json", "r", encoding="utf-8") as f:
    botw_names = json.load(f)

names_set = set(names)
names_to_actors = {}

for actor in botw_names:
    if botw_names[actor] in names_set:
        name = botw_names[actor]
        if name in names_to_actors:
            names_to_actors[name].append(actor)
        else:
            names_to_actors[name] = [actor]

with open("data/inventory-actors.yaml", "w", encoding="utf-8") as f:
    # keep sort order for items
    for name in names:
        if name not in names_to_actors:
            f.write(f"- <Missing> # {name}\n")
            continue
        actors = names_to_actors[name]
        if len(actors) == 1:
            f.write(f"- {actors[0]} # {name}\n")
        else:
            actor_str = ", ".join(actors)
            f.write(f"- [{actor_str}] # {name}\n")
    