"""
Generate the item localization
"""

import sys
import yaml
from build import script_home, project_file, output_file

HEADER = """
# Automatically generated.
# DO NOT EDIT. See packages/generated/README.md for more information.
"""

LOCALES = [
    "en-US",
    "ja-JP",
    "de-DE",
    "es-ES",
    "it-IT",
    "fr-FR",
    "ru-RU",
    "zh-CN",
    "zh-TW",
    "ko-KR",
    "nl-NL",
]

def main():
    locale_lines = {}
    for locale in LOCALES:
        locale_lines[locale] = []

    with open(project_file("data", "seed-actors.yaml"), "r", encoding="utf-8") as f:
        actors: list[str] = yaml.safe_load(f)

    progress = spp.printer(len(actors), "Generate item localization")
    for (i, actor) in enumerate(actors):
        progress.print(i, actor)
        with open(output_file("Actor", f"{actor}.yaml"), "r", encoding="utf-8") as f:
            data = yaml.safe_load(f)

        localization = data["localization"]
        if not localization:
            for locale in LOCALES:
                locale_lines[locale].append(f"actor.{actor}: {actor}")
        else:
            for locale in LOCALES:
                locale_lines[locale].append(f"actor.{actor}: {localization[locale]["name"]["text"]}")

    progress.done()

    for locale in LOCALES:
        with open(project_file("src", "ts", "i18n", f"{locale}.yaml"), "w", encoding="utf-8") as f:
            f.write(HEADER + "\n")
            f.write("\n".join(locale_lines[locale]))

sys.path.append(script_home("src"))
import spp
if __name__ == "__main__":
    main()
