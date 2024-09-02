// Generate Pin Yin search keys
const { pinyin } = require("pinyin");
const YAML = require("yaml");
const path = require("path");
const fs = require("fs");

const FILES = ["zh-CN"];
const SEGMENTER = "segmentit";

// the segmenter can handle some but not ones specific to the game
// so we replace the charater with hetronym with one that only has one pronunciation
const heteronymDictionary = {
    "zh-CN": {
        "\u5927": "\u8fbe", // Da (Big)
        "\u7684": "\u5fb7", // De (Of)
        "\u677e\u9732": "\u677e\u8def", // Song lu (Truffle)
        "\u6f5c\u884c": "\u6f5c\u661f", // Qian xing (Silent/Sneaky)
        "\u841d\u535c": "\u841d\u6ce2", // Luo bo (Radish/Carrot)
        "\u5965\u5c14": "\u6fb3\u5c14", // Ao er (Dinraal)
        "\u77f3": "\u4e8b", // Shi (Stone)
        "\u7ea2": "\u8679", // Hong (Red)
        "\u83ab": "\u672b", // Mo (Moblin/Modulga)
        "\u7fc5\u8180": "\u7fc5\u5e2e", // Chi Bang (Wing)
        "\u516b\u722a": "\u516b\u6293", // Ba Zhua (Octo)
        "\u5f39\u7c27": "\u8c2d\u7c27", // Tan Huang (Spring)
        "\u4f20\u52a8\u8f74": "\u8239\u52a8\u8f74", // Chuan Dong Zhou (Shaft)
        "\u5e0c\u5361": "\u5e0c\u5494", // Xi Ka (Sheika)
        "\u7eff\u8272": "\u94dd\u8272", // Lv Se (Green)
    },
};

const DIR = path.join(__dirname, "../src/i18n/locales");

function checkHeteronym(msg, output) {
    for (const word of output) {
        if (Array.isArray(word) && word.length > 1) {
            throw new Error(`Heteronym found for ${msg}: ${word}`);
        }
    }
    return output.map((word) => word[0]);
}

for (const file of FILES) {
    console.log(file);
    const locale = YAML.parse(
        fs.readFileSync(path.join(DIR, `${file}.yaml`), "utf8"),
    );

    const output = {};
    const dictionary = heteronymDictionary[file] || {};
    for (const key in locale) {
        if (!key.startsWith("actor.")) {
            continue;
        }
        let msg = locale[key];
        for (const phrase in dictionary) {
            msg = msg.replaceAll(phrase, dictionary[phrase]);
        }
        // full segmented words without accents
        const fullWords = checkHeteronym(
            msg,
            pinyin(msg, {
                heteronym: true,
                segment: SEGMENTER,
                group: false,
                style: "normal",
            }),
        ).join(" ");
        const initials = checkHeteronym(
            msg,
            pinyin(msg, {
                heteronym: true,
                segment: SEGMENTER,
                group: false,
                style: "first_letter", // note it's not "initials"
            }),
        ).join("");
        output[key + ".full"] = fullWords;
        output[key + ".initials"] = initials;
    }
    const outputString = YAML.stringify(output, null, 2);
    fs.writeFileSync(path.join(DIR, `${file}.pinyin.yaml`), outputString);
}
