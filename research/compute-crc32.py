"""Compute CRC32 hash for actors and tags"""
import yaml
import util
import zlib
import os

IN = {
    "actor-data": util.output("actor-data.yaml"),
    "*": "botw-data/Actor/ActorLink/AirWall.yml",
}
OUT = {
    "crc32": util.output("crc32.yaml")
}
util.print_stage(__file__, IN, OUT)

hash_map = {}
with open(IN["actor-data"], "r", encoding="utf-8") as f:
    actor_data = yaml.safe_load(f)

def add_crc32(s, typ):
    hash = zlib.crc32(bytes(s, "utf-8"))
    hash_map[f"{hash:08x}"] = { typ: s }

for actor, data in util.progress(actor_data.items(), "compute crc32 for recipes", len(actor_data)):
    add_crc32(actor, "actor")
    add_crc32(data["cureItemEffectType"], "effect")
    for tag in data["tags"]:
        add_crc32(tag, "tag")

# CRC32 for all actors are needed because
# we don't yet know what actors are recipe outputs
actors = [ x[:-4] for x in os.listdir("botw-data/Actor/ActorLink") if x.endswith(".yml") ]
for actor in util.progress(actors, "compute crc32 for actors"):
    add_crc32(actor, "actor")

with open(OUT["crc32"], "w", encoding="utf-8", newline="\n") as f:
    for hash in sorted(hash_map):
        data = hash_map[hash]
        value = next(f"{key+':':<10}{v}" for key, v in data.items())
        f.write(f"'{hash}': {{ {value} }}\n")


