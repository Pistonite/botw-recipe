# Ensure that all other actors are in the same equivalence class as Sheikah Slate
import yaml
import util
import os
from multiprocessing import Pool

# manual extend needed because of multiprocessing
util.extend_yaml()

IN = [
    "data/inventory-actors.yaml",
    "data/important-tags.yaml",
    "output/items-grouped.yaml",
    "data/known-filtered-actors.yaml",
    "output/actor-names.yaml",
    "botw-data/Actor/ActorLink/AirWall.yml",
]
OUT = [
    "output/extra-actors.yaml",
    "output/actor-map.yaml"
]

def is_actor(actor):
    return os.path.isfile(f"botw-data/Actor/ActorLink/{actor}.yml")

# Check if the actor is equal to sheika slate
# if not, return (actor, same_group_actor), where same_group_actor can be used
# to compare if it's the same as another known actor
def check_actor(actor):
    return check_actor_recur(actor, set())

def check_actor_recur(actor, seen):
    if actor in seen:
        return (actor, None)
    seen.add(actor)
    with open(IN[1], "r", encoding="utf-8") as f:
        important_tags = set(yaml.safe_load(f))
    with open(f"botw-data/Actor/ActorLink/{actor}.yml", "r", encoding="utf-8") as f:
        actor_link = yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
    model_user = actor_link["LinkTarget"]["ModelUser"]
    # Actors without model will crash the game when held. Meaning it's unlikely they can be used in cooking
    if model_user == "None": 
        return (None, None)
    profile_user = actor_link["LinkTarget"]["ProfileUser"]
    if profile_user in (
        "WeaponShield", "WeaponSpear", "WeaponSmallSword", "WeaponLargeSword",
        "WeaponBow", "Bullet",
        "ArmorLower", "ArmorUpper", "ArmorHead"
    ):
        # Weapon, Armor and Arrow are not holdable
        # Ok, no invalid actors found
        return (None, None)
    
    if "Tags" not in actor_link:
        tags = set()
    else:
        tags = set([actor_link["Tags"][tag_n] for tag_n in actor_link["Tags"] if actor_link["Tags"][tag_n] in important_tags])
    gpuser = actor_link["LinkTarget"]["GParamUser"]
    if gpuser == "Dummy":
        if not tags:
            # Ok, actor is same as Dummy (Sheikah Slate)
            return (None, None)
        return (actor, None)
    with open(f"botw-data/Actor/GeneralParamList/{gpuser}.gparamlist.yml", "r", encoding="utf-8") as f:
        data = yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
        if is_equal_to_dummy(data):
            # Ok, actor is same as Dummy (Sheikah Slate)
            return (None, None)
    # Check if there is a same group actor name
    if "System" in data:
        if "SameGroupActorName" in data["System"]:
            sgan = data["System"]["SameGroupActorName"]
            # some requires recursive check, like arrows
            if is_actor(sgan):
                check, _ = check_actor_recur(sgan, seen)
                if not check:
                    return (None, None)
    # Returning a second param as this actor might be equal to a known one
            if sgan:
                return (actor, sgan)
    return (actor, gpuser)

def is_equal_to_dummy(data):
    if "Item" in data:
        if "SellingPrice" in data["Item"]:
            if data["Item"]["SellingPrice"] != -1:
                return False
        if "BuyingPrice" in data["Item"]:
            if data["Item"]["BuyingPrice"] != -1:
                return False
    if "CureItem" in data:
        if "HitPointRecover" in data["CureItem"]:
            if data["CureItem"]["HitPointRecover"] != 0:
                return False
        if "EffectType" in data["CureItem"]:
            if data["CureItem"]["EffectType"] != "None":
                return False
        if "EffectLevel" in data["CureItem"]:
            if data["CureItem"]["EffectLevel"] != 0:
                return False
        if "EffectiveTime" in data["CureItem"]:
            if data["CureItem"]["EffectiveTime"] != 0:
                return False
    if "CookSpice" in data:
        if "BoostHitPointRecover" in data["CookSpice"]:
            if data["CookSpice"]["BoostHitPointRecover"] != 0:
                return False
        if "BoostEffectiveTime" in data["CookSpice"]:
            if data["CookSpice"]["BoostEffectiveTime"] != 0:
                return False
        if "BoostSuccessRate" in data["CookSpice"]:
            if data["CookSpice"]["BoostSuccessRate"] != 0:
                return False
        if "BoostMaxHeartLevel" in data["CookSpice"]:
            if data["CookSpice"]["BoostMaxHeartLevel"] != 0:
                return False
        if "BoostStaminaLevel" in data["CookSpice"]:
            if data["CookSpice"]["BoostStaminaLevel"] != 0:
                return False
    return True

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        # second arg for debugging an actor
        print(check_actor(sys.argv[1]))
        exit()
    util.print_stage(__file__, IN, OUT)

    # Optimization: If the gpuser is Dummy, we can skip the check
    # Given that we already checked Dummy == Sheikah Slate
    with open("botw-data/Actor/GeneralParamList/Dummy.gparamlist.yml", "r", encoding="utf-8") as f:
        dummy = yaml.load(f, Loader=yaml.FullLoader)["param_root"]["objects"]
        util.assertion(is_equal_to_dummy(dummy))
    
    with open(IN[0], "r", encoding="utf-8") as f:
        known_actors = set(yaml.safe_load(f))
    with open(IN[2], "r", encoding="utf-8") as f:
        groups = yaml.safe_load(f)
        sheika_slate = list([entry["data"] for entry in groups if "Obj_DRStone_Get" in entry["actors"]])[0]
        util.assertion(len(sheika_slate["tags"]) == 0)
        util.assertion(sheika_slate["cookSpiceBoostHitPointRecover"] == 0)
        util.assertion(sheika_slate["cookSpiceBoostEffectiveTime"] == 0)
        util.assertion(sheika_slate["cookSpiceBoostSuccessRate"] == 0)
        util.assertion(sheika_slate["cookSpiceBoostMaxHeartLevel"] == 0)
        util.assertion(sheika_slate["cookSpiceBoostStaminaLevel"] == 0)
        util.assertion(sheika_slate["cureItemHitPointRecover"] == 0)
        util.assertion(sheika_slate["cureItemEffectType"] == "None")
        util.assertion(sheika_slate["cureItemEffectLevel"] == 0)
        util.assertion(sheika_slate["cureItemEffectiveTime"] == 0)
        util.assertion(sheika_slate["itemSellingPrice"] == -1)
        util.assertion(sheika_slate["itemBuyingPrice"] == -1)
    # Actors that are filtered out
    with open(IN[3], "r", encoding="utf-8") as f:
        for a in yaml.safe_load(f):
            known_actors.add(a)

    with open(IN[4], "r", encoding="utf-8") as f:
        actor_to_name = {}
        for actor, name in yaml.safe_load(f):
            actor_to_name[actor] = name

    def get_actor_name(p):
        util.assertion(p.endswith(".yml"))
        return p[:-4]

    all_actors = []
    for p in os.listdir("botw-data/Actor/ActorLink"):
        name = get_actor_name(p)
        if name not in known_actors:
            all_actors.append(name)
    
    actor_map = {} # maps a same group actor to the actor
    extra_actors = []
    with Pool() as pool:
        for actor, same_group_actor in util.progress(pool.imap_unordered(check_actor, all_actors), "check actors", len(all_actors)):
            if actor:
                if same_group_actor not in known_actors:
                    extra_actors.append(actor)
                else:
                    actor_map[actor] = same_group_actor
    
    extra_actors.sort()
    with open(OUT[0], "w", encoding="utf-8") as f:
        yaml.dump(extra_actors, f)

    with open(OUT[1], "w", encoding="utf-8", newline="\n") as f:
        for actor in sorted(actor_map):
            target = actor_map[actor]
            name = actor_to_name[target]
            actor += ":"
            f.write(f"{actor:<30} {target:<30} # {name}\n")
    
    if extra_actors:
        print(f"{len(extra_actors)} extra actors found")
    exit(len(extra_actors))


    
