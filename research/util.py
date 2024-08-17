import os
import glob
from tqdm import tqdm

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
    input = input + [stage]
    stage = os.path.basename(stage)
    if not is_newer(input, output):
        print(f"=== {stage:<20}: up-to-date")
        exit()
    print(f"=== {stage:<20} ===")

def progress(iterable, desc):
    return tqdm(
        iterable,
        desc=desc,
        ncols=50,
        bar_format="{desc:<20}{n_fmt:>5}/{total_fmt:>5} {percentage:3.0f}% [{bar}]"
    )

def extend_yaml():
    import yaml
    def constructor(loader, node):
        values = loader.construct_mapping(node)
        return dict(values)

    yaml.add_constructor('!list', constructor)
    yaml.add_constructor('!obj', constructor)
    yaml.add_constructor('!io', constructor)
    yaml.add_constructor('!str64', lambda loader, node: str(loader.construct_scalar(node)))
    yaml.add_constructor('!str32', lambda loader, node: str(loader.construct_scalar(node)))
    yaml.add_constructor('!str256', lambda loader, node: str(loader.construct_scalar(node)))
    yaml.add_constructor('!vec3', lambda loader, node: list(loader.construct_sequence(node)))
