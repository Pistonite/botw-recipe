import yaml

with open("recipes_clean.yaml", "r", encoding="utf-8") as file:
    recipes = yaml.load(file, Loader=yaml.FullLoader)

tags = set([
    "CookLowPrice",
    "CookEMedicine",
    "CookFailure"
])
unique_tags = set()
items = set()
for recipe in recipes:
    if not "recipe" in recipe:
        raise ValueError("recipe key not found")
    r = recipe["recipe"]
    unique = "n" in recipe
    for ingredient in r:
        if "tags" in ingredient:
            for t in ingredient["tags"]:
                tags.add(t)
                if unique:
                    unique_tags.add(t)
        if "items" in ingredient:
            for item in ingredient["items"]:
                items.add(item)

        # if a tag can be unique, it cannot be merged


with open("tags_that_matter.yaml", "w", encoding="utf-8") as file:
    yaml.dump(list(tags), file, allow_unicode=True)

with open("tags_unique.yaml", "w", encoding="utf-8") as file:
    yaml.dump(list(unique_tags), file, allow_unicode=True)

with open("items_in_recipe.yaml", "w", encoding="utf-8") as file:
    yaml.dump(list(items), file, allow_unicode=True)
print(tags)
print(items)