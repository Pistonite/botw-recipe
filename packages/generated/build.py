"""
Generate the code!
"""

HEADER = """
//! Automatically generated.
//!
//! DO NOT EDIT. See packages/generated/README.md for more information.

"""

import os
import yaml
import shutil
import subprocess

COMMON_ENUM_DERIVES = [
    "Copy", "Clone", "PartialEq", "Eq", "PartialOrd", "Ord", "Hash"
]

def actor_name_and_name_debug_display_impl(enum_name):
    return """
        #[cfg(all(feature = "english-names", feature="actor-names"))]
        impl std::fmt::Debug for """+enum_name+""" {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(self.actor_name())
                    .field(&self.name())
                    .finish()
            }
        }
        #[cfg(all(not(feature = "english-names"), feature="actor-names"))]
        impl std::fmt::Debug for """+enum_name+""" {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.actor_name().fmt(f)
            }
        }
        #[cfg(feature="actor-names")]
        impl std::fmt::Display for """+enum_name+""" {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.actor_name().fmt(f)
            }
        }
    """

def generate_cook_item():
    with open(output_file("recipe-meta.yaml"), "r", encoding="utf-8") as f:
        recipe_meta = yaml.safe_load(f)
    output_actors = recipe_meta["output_actors"]
    actor_and_english_name = [(actor, get_actor_english_name(actor)) for actor in sorted(output_actors)]
    enum_lines = []
    english_name_lines = []
    actor_name_lines = []

    for (actor, english) in actor_and_english_name:
        # not including the {{effect}} prefix, to minimize binary size
        if not english.startswith("{{effect}}"):
            raise ValueError(f"Invalid English name for {actor}: {english}, should start with {{{{effect}}}}")
        english = english[10:]
        enum_lines.append(f"    /// {english}")
        enum_lines.append(f"    {actor},")
        english_name_lines.append(f"        Self::{actor} => \"{english}\",")
        actor_name_lines.append(f"        Self::{actor} => \"{actor}\",")

    lines = [
        "/// Cooked Item (Output of cooking pot)",
        f"#[derive({",".join(["enum_map::Enum"] + COMMON_ENUM_DERIVES)})]"
        "#[allow(non_camel_case_types)]",
        "#[repr(u8)]",
        "pub enum CookItem {",
    ] + enum_lines + [
        "}",
        "impl CookItem {",
        "/// Get the English name of the cook item actor",
        "#[cfg(feature = \"english-names\")]",
        "pub const fn name(&self) -> &'static str {",
        "match self {",
    ] + english_name_lines + [
        "}}",
        "/// Get the actor name of the cook item",
        "#[cfg(feature = \"actor-names\")]",
        "pub const fn actor_name(&self) -> &'static str {",
        "match self {",
    ] + actor_name_lines + [
        "}}}",
        actor_name_and_name_debug_display_impl("CookItem")
    ] 

    write_rust_source(src_file("cook_item.rs"), lines)


def write_rust_source(path: str, lines: list[str]):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(HEADER)
        for line in lines:
            f.write(line)
            f.write("\n")
    rustfmt = shutil.which("rustfmt")
    if rustfmt:
        subprocess.run([rustfmt, path], check=True)


def src_file(*args):
    return os.path.join(os.path.dirname(__file__), "src", *args)

def get_actor_english_name(actor: str) -> str:
    actor_path = output_file("Actor", f"{actor}.yaml")
    with open(actor_path, "r", encoding="utf-8") as f:
        data = yaml.safe_load(f)
    localization = data["localization"]
    if not localization:
        return ""
    return localization["en-US"]["name"]["text"]

def output_file(*args):
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), "research-scripts", "output", *args)

if __name__ == "__main__":
    generate_cook_item()
