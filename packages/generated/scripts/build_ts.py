"""
Generate the TypeScript code!
"""

import sys
import subprocess
import shutil
from build import script_home, load_actors_and_groups, project_file

HEADER = """
/**
 * Automatically generated.
 *
 * DO NOT EDIT. See packages/generated/README.md for more information.
 */
"""

def main():
    actors, groups, _ = load_actors_and_groups()
    generate_group(groups)

def generate_group(groups: list[tuple[str, list[str]]]):
    enum_lines = [
        "// \"Empty\" slot in recipe input",
        "None: 0,",
    ]

    progress = spp.printer(len(groups), "Generate TS Group")
    for (i, (group_name, _)) in enumerate(groups):
        progress.print(i, group_name)
        enum_lines.append(f"    {group_name} = {i+1},")
    progress.done()

    lines = [
        "export const Group = {",
    ] + enum_lines + [
        "} as const;",
        "export type Group = typeof Group[keyof typeof Group];",
        "/** Get all the groups as an array */",
        "export function getGroups(): Group[] {",
        # plus one for the none group
        f"return Array.from({{ length: {len(groups) + 1} }}, (_, i) => i as Group);",
        "}",
    ]

    return write_typescript_source(ts_src_file("data", "Group.ts"), lines)

def write_typescript_source(path: str, lines: list[str]) -> str:
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(HEADER)
        for line in lines:
            f.write(line)
            f.write("\n")
    return path

def run_prettier(paths: list[str]):
    print("Running prettier")
    pnpm = shutil.which("pnpm")
    if pnpm:
        subprocess.run([pnpm, "exec", "prettier", "--write"] + paths, check=True)

def ts_src_file(*args):
    return project_file("ts", *args)

if __name__ == "__main__":
    sys.path.append(script_home("src"))
    import spp
    main()
