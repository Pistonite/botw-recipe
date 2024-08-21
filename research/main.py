import sys
import os
import subprocess
import shutil

CLEAN = False
O = "output"

def sparse_checkout(repo, path, branch, checkout_paths):
    clean = CLEAN
    if not clean:
        for p in checkout_paths:
            if not os.path.exists(os.path.join(path, p)):
                clean = True
                print(f"{p} not found, forcing re-checkout")
                break

    if clean:
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

def run_script(script):
    status = subprocess.run(["python", script])
    if status.returncode != 0:
        print(f"{script} failed")
        sys.exit(status.returncode)

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--clean":
        CLEAN = True
    # prepare data
    sparse_checkout(
        "https://github.com/Pistonight/cooking.rs",
        "cooking-rs",
        "main",
        [
            "src/cook_recipes.json"
        ]
    )
    sparse_checkout(
        "https://github.com/leoetlino/botw",
        "botw-data",
        "master",
        [
            "Actor/ActorLink/",
            "Actor/GeneralParamList/",
            "Message/"
        ]
    )
    if CLEAN:
        if os.path.exists(O):
            shutil.rmtree(O)
    if not os.path.exists(O):
        os.makedirs(O)
    run_script("get-actor-names.py")
    run_script("get-actor-data.py")
    run_script("group-items.py")
    run_script("validate-groups.py")
    run_script("ensure-exhaustiveness.py")
    run_script("generate-source.py")
    run_script("prepare-recipes.py")
