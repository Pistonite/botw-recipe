import os
import shutil

if not os.path.exists("raw"):
    exit(1)

os.makedirs("data", exist_ok=True)

for filename in os.listdir("raw"):
    if filename.startswith("ck_") and filename.endswith(".bin"):
        id = int(filename[3:-4])
        target = f"data/chunk_{id}.rawdat"
        if os.path.exists(target):
            os.remove(target)
        shutil.copy(f"raw/{filename}", target)
        print(f"cp {filename} {target}")

