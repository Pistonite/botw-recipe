import yaml
with open("items_in_recipe.yaml", "r", encoding="utf-8") as file:
    items_in_recipe = set(yaml.load(file, Loader=yaml.FullLoader))
with open("tags_unique.yaml", "r", encoding="utf-8") as file:
    tags_unique = set(yaml.load(file, Loader=yaml.FullLoader))
def item_equivalent(item1, item2):
    if item1["Name"] == item2["Name"]:
        return True
    if item1["Name"] in items_in_recipe or item2["Name"] in items_in_recipe:
        return False
    if item1["BoostHitPointRecover"] != item2["BoostHitPointRecover"]:
        return False
    if item1["BoostSuccessRate"] != item2["BoostSuccessRate"]:
        return False
    if item1["BuyingPrice"] != item2["BuyingPrice"]:
        return False
    if item1["EffectType"] != item2["EffectType"]:
        return False
    if item1["HitPointRecover"] != item2["HitPointRecover"]:
        return False
    if item1["SellingPrice"] != item2["SellingPrice"]:
        return False
    if set(item1["Tags"]) != set(item2["Tags"]):
        return False
    for tag in item1["Tags"]:
        if tag in tags_unique:
            return False
    return True

item_groups = []

with open("all_items_clean_tag.yaml", "r", encoding="utf-8") as file:
    items = yaml.load(file, Loader=yaml.FullLoader)

for item in items:
    found = False
    for group in item_groups:
        if item_equivalent(item, group[0]):
            group.append(item)
            found = True
            break
    if not found:
        item_groups.append([item])

item_grouped = []

for group in item_groups:
    new_group ={
        "BoostHitPointRecover": group[0]["BoostHitPointRecover"],
        "BoostSuccessRate": group[0]["BoostSuccessRate"],
        "BuyingPrice": group[0]["BuyingPrice"],
        "EffectType": group[0]["EffectType"],
        "HitPointRecover": group[0]["HitPointRecover"],
        "SellingPrice": group[0]["SellingPrice"],
        "Tags": group[0]["Tags"],
    }
    names = [x["Name"] for x in group]
    new_group["Names"] = names
    item_grouped.append(new_group)

with open("all_items_grouped.yaml", "w", encoding="utf-8") as file:
    yaml.dump(item_grouped, file, allow_unicode=True)

print(len(item_grouped))