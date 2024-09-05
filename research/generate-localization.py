# Generate localization files for items

import yaml
import util
import os

GEN_TAG = "### AUTOGEN ###"

LOCALE_MAP = {
    "en-US": "USen",
    "ja-JP": "JPja",
    "de-DE": "EUde",
    "es-ES": "EUes",
    "it-IT": "EUit",
    "fr-FR": "EUfr",
    "ru-RU": "EUru",
    "zh-CN": "CNzh",
    "zh-TW": "TWzh",
    "ko-KR": "KRko",
    "nl-NL": "EUnl",
}

NAMELESS = set(["dyecolor_00"])

IN = {
    "inventory-actors": "data/inventory-actors.yaml",
    "dummy": "botw-data/Message/Msg_USen.product.sarc/ActorType/Item.msyt",
}
OUT = {}
for locale in LOCALE_MAP:
    OUT[locale] = f"../app/rdb/src/i18n/locales/{locale}.yaml"

util.print_stage(__file__, IN, OUT)

with open(IN["inventory-actors"], "r", encoding="utf-8") as f:
    actors = yaml.safe_load(f)

def write_names(locale, actors):
    lines = []
    if os.path.exists(OUT[locale]):
        with open(OUT[locale], "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line == GEN_TAG:
                    break
                lines.append(line)
    lines.append(GEN_TAG)
    for actor in actors:
        lines.append(f"actor.{actor}: {actors[actor]}")
    with open(OUT[locale], "w", encoding="utf-8", newline="\n") as f:
        for line in lines:
            f.write(line + "\n")

for locale, locale_code in LOCALE_MAP.items():
    input_files = [
    f"botw-data/Message/Msg_{locale_code}.product.sarc/ActorType/Item.msyt",
    f"botw-data/Message/Msg_{locale_code}.product.sarc/ActorType/CapturedActor.msyt",
    f"botw-data/Message/Msg_{locale_code}.product.sarc/ActorType/PlayerItem.msyt", 
    ]
    items = {}
    for file in input_files:
        with open(file, "r", encoding="utf-8") as f:
            x = yaml.safe_load(f)["entries"]
            for n in x:
                items[n] = x[n]

    output = {}
    for actor in util.progress(actors, f"extracting {locale}"):
        if actor in NAMELESS:
            name = actor
        else:
            key = actor+"_Name"
            data = items[actor+"_Name"]["contents"]
            if len(data) == 1:
                name = data[0]["text"]
            else:
                if locale == "ja-JP":
                    # handling for Japanese: control.zero.zero.field_3 is how many bytes to remove
                    name = ""
                    last_control = None
                    for x in data:
                        if "text" in x:
                            if last_control:
                                name += x["text"][last_control:]
                                last_control = None
                            else:
                                name += x["text"]
                        elif "control" in x:
                            last_control = x["control"]["zero"]["zero"]["field_3"] 
                            # divided by 2 because it's in bytes
                            if last_control % 2 != 0:
                                raise ValueError(f"{actor} has an odd number of bytes to remove!??")
                            last_control = last_control // 2
                        else:
                            raise ValueError(f"{actor} has {x} in its name!??")
                else:
                    # ignore control for other languages
                    name = "".join([x["text"] for x in data if "text" in x])
                    #raise ValueError(f"{actor} has {len(data)} names!??")
        output[actor] = name
    write_names(locale, output)

