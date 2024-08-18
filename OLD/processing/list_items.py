import yaml

with open("all_items_raw.yaml", "r", encoding="utf-8") as file:
    items = yaml.load(file, Loader=yaml.FullLoader)

names = []
for item in items:
    names.append(item["Name"])

with open("item_names.yaml", "w", encoding="utf-8") as file:
    yaml.dump(names, file, allow_unicode=True)