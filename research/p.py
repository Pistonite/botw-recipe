import json

with open("out.json", "r", encoding="utf-8") as f:
    d = json.load(f)
print(len(d))