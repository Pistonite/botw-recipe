import sys
import os
import subprocess
import shutil

CLEAN = False

def sparse_checkout(repo, path, branch, checkout_paths):
    if CLEAN:
        if os.path.exists(path):
            shutil.rmtree(path)
    if not os.path.exists(path):
        os.makedirs(path)
    else:
        print(f"{path} already exists, skipping. use --clean to force re-checkout")
        return
    subprocess.run(["git", "init"], cwd=path)
    subprocess.run(["git", "remote", "add", "origin", repo], cwd=path)
    subprocess.run(["git", "config", "core.sparseCheckout", "true"], cwd=path)
    with open(os.path.join(path, ".git", "info", "sparse-checkout"), "w", encoding="utf-8") as f:
        for checkout_path in checkout_paths:
            f.write(checkout_path + "\n")
    subprocess.run(["git", "pull", "--depth=1", "origin", branch], cwd=path)


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "clean":
        CLEAN = True
    # prepare data
    sparse_checkout(
        "https://github.com/MrCheeze/botw-tools",
        "botw-tools",
        "master",
        [
            "botw_names.json"
        ]
    )
    