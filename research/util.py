import sys
import os
import glob
from tqdm import tqdm
import subprocess
import shutil

def is_newer(a_list, b_list):
    a_list = a_list + [__file__]
    for a_glob in a_list:
        for a in glob.glob(a_glob):
            for b in b_list:
                if is_newer_single(a, b):
                    return True
    return False

def is_newer_single(a, b):
    return not os.path.exists(b) or os.path.getmtime(a) > os.path.getmtime(b)

def print_stage(stage, input, output):
    if isinstance(input, dict):
        input = list(input.values())
    if isinstance(output, dict):
        output = list(output.values())
    input = input + [ stage ]
    stage = os.path.basename(stage)
    if not is_newer(input, output):
        print(f"=== {stage:<30}: up-to-date")
        exit()
    print(f"=== {stage:<30} ===")

def output(path):
    """Get output file path"""
    return os.path.join(os.path.dirname(__file__), "output", path)

def progress(iterable, desc, len=None):
    return tqdm(
        iterable,
        desc=desc,
        ncols=50,
        total=len,
        bar_format="{desc:<20}{n_fmt:>5}/{total_fmt:>5} {percentage:3.0f}% [{bar}]"
    )

def extend_yaml():
    import yaml
    def dict_ctor(loader, node):
        values = loader.construct_mapping(node)
        return dict(values)

    def str_ctor(loader, node):
        values = loader.construct_scalar(node)
        return str(values)

    def int_ctor(loader, node):
        values = loader.construct_scalar(node)
        return int(values, 0)

    def list_ctor(loader, node):
        values = loader.construct_sequence(node)
        return list(values)

    yaml.add_constructor('!list', dict_ctor)
    yaml.add_constructor('!obj', dict_ctor)
    yaml.add_constructor('!io', dict_ctor)
    yaml.add_constructor('!str64', str_ctor)
    yaml.add_constructor('!str32', str_ctor)
    yaml.add_constructor('!str256', str_ctor)
    yaml.add_constructor('!vec3', list_ctor)
    yaml.add_constructor('!u', int_ctor)

def assertion(value, message = "Assertion failed"):
    if not value:
        raise AssertionError(message)

NUM_INGR = 5

def make_multichoose(num_groups):
    # bionmial(n, k), k<=NUM_INGR is bino[n][k]
    bino = []
    for _ in range(num_groups+NUM_INGR):
        bino.append([0]*(NUM_INGR+1))
    for n in range(num_groups+NUM_INGR):
        bino[n][0] = 1

    for k in range(NUM_INGR+1):
        bino[k][k] = 1

    for n in range(1,num_groups+NUM_INGR):
        for k in range(1,NUM_INGR+1):
            bino[n][k] = bino[n-1][k-1] + bino[n-1][k]
    # multichoose(n, k) is multichoose[n][k]
    multichoose = []
    for _ in range(num_groups+1):
        multichoose.append([0]*(NUM_INGR+1))
    for n in range(num_groups+1):
        multichoose[n][0] = 1
    for k in range(1, NUM_INGR+1):
        for n in range(num_groups+1):
            multichoose[n][k] = bino[n+k-1][k]
    return multichoose

def total_records(num_groups, multichoose):
    return multichoose[num_groups][NUM_INGR]

CHUNK_SIZE = 4096 * 100
def chunk(total):
    """Returns (chunk_size, chunk_count, last_chunk_size)"""
    assertion(total % CHUNK_SIZE != 0, "total divisible by chunk size")
    chunk_count = total//CHUNK_SIZE + 1
    last_chunk_size = total % CHUNK_SIZE
    assertion(CHUNK_SIZE*(chunk_count-1) + last_chunk_size == total, "chunk size calculation")
    return CHUNK_SIZE, chunk_count, last_chunk_size

COMPACT_CHUNK_SIZE = 4096 * 500
def chunk_compact(total):
    assertion(total % COMPACT_CHUNK_SIZE != 0, "total divisible by chunk size")
    chunk_count = total//COMPACT_CHUNK_SIZE + 1
    last_chunk_size = total % COMPACT_CHUNK_SIZE
    assertion(COMPACT_CHUNK_SIZE*(chunk_count-1) + last_chunk_size == total, "chunk size calculation")
    return COMPACT_CHUNK_SIZE, chunk_count, last_chunk_size

def which(cmd):
    cmd = shutil.which(cmd)
    if not cmd:
        print(f"{cmd} is not installed!")
        sys.exit(1)
    return cmd

def sparse_checkout(clean, repo, path, branch, checkout_paths):
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
    git = which("git")
    subprocess.run([git, "init"], cwd=path)
    subprocess.run([git, "remote", "add", "origin", repo], cwd=path)
    subprocess.run([git, "config", "core.sparseCheckout", "true"], cwd=path)
    with open(os.path.join(path, ".git", "info", "sparse-checkout"), "w", encoding="utf-8") as f:
        for checkout_path in checkout_paths:
            f.write(checkout_path + "\n")
    subprocess.run([git, "pull", "--depth=1", "origin", branch], cwd=path)
