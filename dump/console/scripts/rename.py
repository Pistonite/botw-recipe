import os
import shutil
from multiprocessing import Pool

def copy(filename):
    id = int(filename[3:-4])
    target = f"data/chunk_{id}.rawdat"
    if os.path.exists(target):
        os.remove(target)
    shutil.copy(f"raw/{filename}", target)
    return filename, target

if __name__ == "__main__":
    if not os.path.exists("raw"):
        exit(1)

    os.makedirs("data", exist_ok=True)

    jobs = []
    for filename in os.listdir("raw"):
        if filename.startswith("ck_") and filename.endswith(".bin"):
            jobs.append(filename)

    with Pool() as pool:
        for (filename, target) in pool.imap_unordered(copy, jobs):
            print(f"cp raw/{filename} {target}")

