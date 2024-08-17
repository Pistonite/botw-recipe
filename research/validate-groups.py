import yaml
import util

IN = [
    "output/items-grouped.yaml",
    "data/ignored-tags.yaml",
    "data/important-tags.yaml",
    "botw-data/Actor/GeneralParamList/*.gparamlist.yml",
    "botw-data/Actor/ActorLink/*.yml",
]
OUT = ["output/total.txt"]
util.print_stage(__file__, IN, OUT)
util.extend_yaml()

with open(IN[0], "r", encoding="utf-8") as file:
    groups = yaml.safe_load(file)

with open(IN[1], "r", encoding="utf-8") as file:
    f_str = file.read()
    IGNORE_TAGS = set(yaml.safe_load(f_str))
    o = yaml.dump(list(sorted(IGNORE_TAGS)))
    if o != f_str:
        with open(IN[1], "w", encoding="utf-8") as file:
            file.write(o)

with open(IN[2], "r", encoding="utf-8") as file:
    f_str = file.read()
    important_tags = yaml.safe_load(f_str)
    for tag in important_tags:
        if tag in IGNORE_TAGS:
            raise ValueError(f"{tag} is in both important and ignored tags!??")
    important_tags.sort()
    o = yaml.dump(important_tags)
    if o != f_str:
        with open(IN[2], "w", encoding="utf-8") as file:
            file.write(o)

seen = set()
for group in util.progress(groups, "check unique items"):
    for item in group["actors"]:
        if item in seen:
            raise ValueError(f"{item} is in multiple groups!??")
        seen.add(item)
for group in util.progress(groups, "check data"):
    for item in group["actors"]:
        with open(f"botw-data/Actor/ActorLink/{item}.yml", "r", encoding="utf-8") as f:
            actor_link = yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
        tags = list(sorted(set([actor_link["Tags"][tag_n] for tag_n in actor_link["Tags"]]) - IGNORE_TAGS))
        if tags != group["data"]["tags"]:
            print(f"left: {tags}")
            print(f"right: {group['data']['tags']}")
            s = set(tags) - IGNORE_TAGS - set(important_tags)
            print("---")
            for tag in s:
                print(f"- {tag}")
            print("---")
            raise ValueError(f"{item} has different tags!??")
        gpuser = actor_link["LinkTarget"]["GParamUser"]
        with open(f"botw-data/Actor/GeneralParamList/{gpuser}.gparamlist.yml", "r", encoding="utf-8") as f:
            actor_param = yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
        data = group["data"]
        if "CookSpice" not in actor_param:
            valid = (data["cookSpiceBoostHitPointRecover"] == 0
                 and data["cookSpiceBoostEffectiveTime"] == 0
                 and data["cookSpiceBoostSuccessRate"] == 0
                 and data["cookSpiceBoostMaxHeartLevel"] == 0
                 and data["cookSpiceBoostStaminaLevel"] == 0)
            if not valid:
                raise ValueError(f"{item} has no CookSpice data, and grouped data is not all zero")
        else:
            valid = (data["cookSpiceBoostHitPointRecover"] == actor_param["CookSpice"]["BoostHitPointRecover"]
                and data["cookSpiceBoostEffectiveTime"] == actor_param["CookSpice"]["BoostEffectiveTime"]
                and data["cookSpiceBoostSuccessRate"] == actor_param["CookSpice"]["BoostSuccessRate"]
                and data["cookSpiceBoostMaxHeartLevel"] == actor_param["CookSpice"]["BoostMaxHeartLevel"]
                and data["cookSpiceBoostStaminaLevel"] == actor_param["CookSpice"]["BoostStaminaLevel"])
            if not valid:
                raise ValueError(f"{item} has different CookSpice data")
        if "CureItem" not in actor_param:
            valid = (data["cureItemHitPointRecover"] == 0
                 and data["cureItemEffectType"] == "None"
                 and data["cureItemEffectLevel"] == 0
                 and data["cureItemEffectiveTime"] == 0)
            if not valid:
                raise ValueError(f"{item} has no CureItem data, and grouped data is not all zero")
        else:
            valid = (data["cureItemHitPointRecover"] == actor_param["CureItem"]["HitPointRecover"]
                and data["cureItemEffectType"] == actor_param["CureItem"]["EffectType"]
                and data["cureItemEffectLevel"] == actor_param["CureItem"]["EffectLevel"]
                and data["cureItemEffectiveTime"] == actor_param["CureItem"]["EffectiveTime"])
            if not valid:
                raise ValueError(f"{item} has different CureItem data")
        valid = data["itemSellingPrice"] == actor_param["Item"]["SellingPrice"] and data["itemBuyingPrice"] == actor_param["Item"]["BuyingPrice"]
        if not valid:
            raise ValueError(f"{item} has different Item data")

NUM_ITEMS = len(groups)
NUM_INGR = 5
# Compute constants
# bionmial(n, k), k<=NUM_INGR is bino[n][k]
bino = [[0]*(NUM_INGR+1)]*(NUM_ITEMS+NUM_INGR)

for n in range(NUM_ITEMS+NUM_INGR):
    bino[n][0] = 1

for k in range(NUM_INGR+1):
    bino[k][k] = 1

for n in util.progress(range(1,NUM_ITEMS+NUM_INGR), "compute binomial"):
    for k in range(1,NUM_INGR+1):
        bino[n][k] = bino[n-1][k-1] + bino[n-1][k]
NUM_RECORD = bino[NUM_ITEMS+NUM_INGR-1][NUM_INGR]
print(f"total: {NUM_RECORD}")
with open(OUT[0], "w", encoding="utf-8") as f:
    f.write(f"{NUM_RECORD}\n")