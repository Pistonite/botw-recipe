import yaml, json

def clean_ingredients(alldata):
    all = {}
    for data in alldata:
        output = {}
        name = data ["Name"]
        output["sell_price"] = data["SellingPrice"] # default 0
        output["buy_price"] = data["BuyingPrice"]   # default 0

        for tag in data["Tags"]:
            if tag == "CookLowPrice":
                output["cook_price_override"] = 1

        output["effect_type"] = data["EffectType"]
        output["hearts"]=data["HitPointRecover"]
        output["hearts_boost"]=data["BoostHitPointRecover"]
        output["crit_boost"]=data["BoostSuccessRate"]

        all[name] = output
    print(len(all))
    return all

def clean_recipes(data):
    all = []
    for recipeData in data["Recipes"]:
        name = recipeData["Recipe"]
        output = {}
        output["name"] = name
        if "HB" in recipeData:
            output["base_hearts"] = int(recipeData["HB"])
        output["recipe"] = []
        if "Actors" in recipeData:
            for actor_group in recipeData["Actors"]:
                output["recipe"].append({
                    "items": actor_group
                })

        if "Tags" in recipeData:
            for tag_group in recipeData["Tags"]:
                group = []
                for tag in tag_group:
                    if not tag: # empty list
                        print(name)
                        continue
                    if len(tag) > 1:
                        raise ValueError(name)
                    group.append(tag[0])

                output["recipe"].append({
                    "tags": group
                })
        all.append(output)

    for recipeData in data["SingleRecipes"]:
        name = recipeData["Recipe"]
        output = {}
        output["name"] = name
        if recipeData["Num"] != "1":
            print(recipeData["Num"])
            raise ValueError(name)
        output["n"] = 1
        if "HB" in recipeData:
            output["base_hearts"] = int(recipeData["HB"])
        output["recipe"] = []
        if "Actors" in recipeData:
            # if len(recipeData["Actors"]) > 1:
            #     raise ValueError(name)
            output["recipe"].append({"items":recipeData["Actors"]})

        if "Tags" in recipeData:
            if len(recipeData["Tags"]) > 1:
                raise ValueError(name)
            output["recipe"].append({
                "tags": recipeData["Tags"][0]
            })

                
        all.append(output)
    return all


with open("recipeData.json", "r", encoding="utf-8") as file:
    data = json.load(file)

with open("recipeData.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(data[0], file2)

ingredients_clean = clean_ingredients(data[0])
with open("ingredients_clean.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(ingredients_clean, file2)

crits = {}
for item in ingredients_clean:
    crit = ingredients_clean[item]["crit_boost"]
    if crit:
        crits[item] = crit
with open("crits.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(crits, file2)

with open("cookData.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(data[1], file2)

with open("recipes_clean.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(clean_recipes(data[1]), file2)

with open("version.yaml", "w+", encoding="utf-8") as file2:
    yaml.dump(data[2], file2)