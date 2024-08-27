import os
import sys
import shutil
from multiprocessing import Pool

def copy(args):
    filename, keep = args
    id = int(filename[3:-4])
    target = f"data/chunk_{id}.rawdat"
    if keep and os.path.exists(target):
        return filename, target, False
    if os.path.exists(target):
        os.remove(target)
    shutil.copy(f"raw/{filename}", target)
    return filename, target, True

if __name__ == "__main__":
    keep = len(sys.argv) > 1 and (sys.argv[1] == "-k" or sys.argv[1] == "--keep")

    if not os.path.exists("raw"):
        exit(1)

    os.makedirs("data", exist_ok=True)

    jobs = []
    for filename in os.listdir("raw"):
        if filename.startswith("ck_") and filename.endswith(".bin"):
            jobs.append((filename, keep))

    with Pool() as pool:
        for (filename, target, moved) in pool.imap_unordered(copy, jobs):
            if moved:
                print(f"cp raw/{filename} {target}")
