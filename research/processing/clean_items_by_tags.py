# filter items by tags that matters
import yaml

with open("tags_that_matter.yaml", "r", encoding="utf-8") as file:
    tags_matter = set(yaml.load(file, Loader=yaml.FullLoader))

with open("all_items_raw.yaml", "r", encoding="utf-8") as file:
    items = yaml.load(file, Loader=yaml.FullLoader)

for item in items:
    tags = item["Tags"]
    item["Tags"] = list([t for t in tags if t in tags_matter])

with open("all_items_clean_tag.yaml", "w", encoding="utf-8") as file:
    yaml.dump(items, file, allow_unicode=True)