import sys
import os
import util
import subprocess
import shutil

O = "output"

def run_script(script):
    status = subprocess.run(["python", script])
    if status.returncode != 0:
        print(f"{script} failed")
        sys.exit(status.returncode)

if __name__ == "__main__":
    clean = len(sys.argv) > 1 and sys.argv[1] == "--clean"

    # prepare data
    # TODO - get recipe from botw
    util.sparse_checkout(
        clean,
        "https://github.com/Pistonight/cooking.rs",
        "cooking-rs",
        "consistency",
        [
            "src/cook_recipes.json",
        ]
    )
    util.sparse_checkout(
        clean,
        "https://github.com/leoetlino/botw",
        "botw-data",
        "master",
        [
            "Actor/ActorLink/",
            "Actor/GeneralParamList/",
            "Message/"
        ]
    )
    if clean:
        if os.path.exists(O):
            shutil.rmtree(O)
    if not os.path.exists(O):
        os.makedirs(O)
    run_script("get-actor-names.py")
    run_script("get-actor-data.py")
    run_script("group-items.py")
    run_script("validate-groups.py")
    run_script("ensure-exhaustiveness.py")
    run_script("prepare-recipes.py")

    run_script("generate-source.py")
    run_script("generate-localization.py")
