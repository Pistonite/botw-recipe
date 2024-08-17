import yaml
import util

IN = [
    "output/actor-names.yaml",
    "data/important-tags.yaml",
    "botw-data/Actor/GeneralParamList/*.gparamlist.yml",
    "botw-data/Actor/ActorLink/*.yml",
]
OUT = ["output/actor-data.yaml"]
util.print_stage(__file__, IN, OUT)
util.extend_yaml()

with open(IN[0], "r", encoding="utf-8") as f:
    actors = yaml.safe_load(f)
with open(IN[1], "r", encoding="utf-8") as f:
    IMPORTANT_TAGS = yaml.safe_load(f)
data = {}

for actor, name in util.progress(actors, "load actors"):
    with open(f"botw-data/Actor/ActorLink/{actor}.yml", "r", encoding="utf-8") as f:
        actor_link= yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
    actor_gpuser = actor_link["LinkTarget"]["GParamUser"]
    with open(f"botw-data/Actor/GeneralParamList/{actor_gpuser}.gparamlist.yml", "r", encoding="utf-8") as f:
        actor_data= yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
    if "CureItem" not in actor_data:
        actor_data["CureItem"] = {
            "HitPointRecover": 0,
            "EffectType": "None",
            "EffectLevel": 0,
            "EffectiveTime": 0
        }
    if len(actor_data["CureItem"]) != 4:
        raise ValueError(f"{actor} has {len(actor_data['CureItem'])} CureItem params!??")
    if "CookSpice" not in actor_data:
        actor_data["CookSpice"] = {
            "BoostHitPointRecover": 0,
            "BoostEffectiveTime": 0,
            "BoostSuccessRate": 0,
            "BoostMaxHeartLevel": 0,
            "BoostStaminaLevel": 0
        }
    if len(actor_data["CookSpice"]) != 5:
        raise ValueError(f"{actor} has {len(actor_data['CookSpice'])} CookSpice params!??")
    if "Item" not in actor_data:
        raise ValueError(f"{actor} has no Item!??")
    actor_tags = actor_link["Tags"]
    tags = list(sorted([actor_tags[x] for x in actor_tags if actor_tags[x] in IMPORTANT_TAGS]))
    the_data = {
        "cureItemHitPointRecover": actor_data["CureItem"]["HitPointRecover"],
        "cureItemEffectType": actor_data["CureItem"]["EffectType"],
        "cureItemEffectLevel": actor_data["CureItem"]["EffectLevel"],
        "cureItemEffectiveTime": actor_data["CureItem"]["EffectiveTime"],
        "cookSpiceBoostHitPointRecover": actor_data["CookSpice"]["BoostHitPointRecover"],
        "cookSpiceBoostEffectiveTime": actor_data["CookSpice"]["BoostEffectiveTime"],
        "cookSpiceBoostSuccessRate": actor_data["CookSpice"]["BoostSuccessRate"],
        "cookSpiceBoostMaxHeartLevel": actor_data["CookSpice"]["BoostMaxHeartLevel"],
        "cookSpiceBoostStaminaLevel": actor_data["CookSpice"]["BoostStaminaLevel"],
        "itemSellingPrice": actor_data["Item"]["SellingPrice"],
        "itemBuyingPrice": actor_data["Item"]["BuyingPrice"],
        "tags": tags,
    }
    data[actor] = the_data

with open(OUT[0], "w", encoding="utf-8") as f:
    for actor, name in util.progress(actors, "save output"):
        obj = yaml.dump({actor: data[actor]})
        lines = obj.split("\n")
        lines[0] = f"{lines[0]:<40}# {name}"
        for line in lines:
            f.write(line + "\n")
    
