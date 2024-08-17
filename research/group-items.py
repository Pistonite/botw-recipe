import yaml
import util

IN = [
    "data/non-group-items.yaml",
    "data/non-group-tags.yaml",
    "output/actor-names.yaml",
    "output/actor-data.yaml"
]
OUT = ["output/items-grouped.yaml", "output/items-grouped-report.txt"]
util.print_stage(__file__, IN, OUT)

KEYS = (
        "cookSpiceBoostEffectiveTime",
        "cookSpiceBoostHitPointRecover",
        "cookSpiceBoostMaxHeartLevel",
        "cookSpiceBoostStaminaLevel",
        "cookSpiceBoostSuccessRate",
        "cureItemEffectLevel",
        "cureItemEffectType",
        "cureItemEffectiveTime",
        "cureItemHitPointRecover",
        "itemBuyingPrice",
        "itemSellingPrice",
    )

with open(IN[0], "r", encoding="utf-8") as file:
    non_group_items = set(yaml.load(file, Loader=yaml.FullLoader))
with open(IN[1], "r", encoding="utf-8") as file:
    non_group_tags = set(yaml.load(file, Loader=yaml.FullLoader))
with open(IN[2], "r", encoding="utf-8") as file:
    actor_to_name = {}
    actors = []
    for actor, name in yaml.load(file, Loader=yaml.FullLoader):
        actor_to_name[actor] = name
        actors.append(actor)
with open(IN[3], "r", encoding="utf-8") as file:
    actor_data = yaml.load(file, Loader=yaml.FullLoader)

def item_equivalent(item1, item2):
    if item1 == item2:
        return True
    # If the item exists in some recipe data, it should not be grouped
    if actor_to_name[item1] in non_group_items or actor_to_name[item2] in non_group_items:
        return False
    item1 = actor_data[item1]
    item2 = actor_data[item2]
    if set(item1["tags"]) != set(item2["tags"]):
        return False
    # If the item has any tag that's important in recipes, it should not be grouped
    for tag in item1["tags"]:
        if tag in non_group_tags:
            return False
    for tag in item2["tags"]:
        if tag in non_group_tags:
            return False
    for k in KEYS:
        if item1[k] != item2[k]:
            return False
    return True

items_grouped_actors = []
"""Output Format:

- actors:                         # GROUP 000
  - Actor1                        # - Name 1
  ...
  data:                           
    cookSpiceBoostEffectiveTime: 0
    ....
    tags: [tag1, tag2, ...]

"""

for actor in util.progress(actors, "grouping"):
    found = False
    for group in items_grouped_actors: # list of actors
        if item_equivalent(actor, group[0]):
            group.append(actor)
            found = True
            break
    if not found:
        items_grouped_actors.append([actor])

group_lens = {}
max_len = 0
for group in items_grouped_actors:
    l = len(group)
    if l not in group_lens:
        group_lens[l] = 0
    group_lens[l] += 1
    if l > max_len:
        max_len = l
report = "group sizes:\n"
for i in range(1, max_len+1):
    if i in group_lens:
        report += f"  {i}: {group_lens[i]}\n"

with open(OUT[1], "w", encoding="utf-8") as f:
    f.write(report)
print(report)

with open(OUT[0], "w", encoding="utf-8") as f:
    for i, group in enumerate(util.progress(items_grouped_actors, "save groups")):
        s = "- actors:"
        f.write(f"{s:<40}# GROUP {i:03}\n")
        for actor in group:
            f.write(f"  - {actor:<36}# - {actor_to_name[actor]}\n")
        s = "  data:"
        f.write(f"{s:<40}\n")
        for k in KEYS:
            f.write(f"    {k}: {actor_data[group[0]][k]}\n")
        tags = list(sorted(actor_data[group[0]]["tags"]))
        tags_str = ", ".join(tags)
        f.write(f"    tags: [{tags_str}]\n")