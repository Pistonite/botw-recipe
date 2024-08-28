const YAML = require("yaml");
const path = require("path");
const fs = require("fs");

const DIR = path.join(__dirname, "../src/i18n/locales");

const defaultLocale = YAML.parse(fs.readFileSync(path.join(DIR, "en-US.yaml"), "utf8"));

const errors = [];

for (const file of fs.readdirSync(DIR)) {
  if (file === "en-US.yaml"){
    continue;
  }

  const locale = YAML.parse(fs.readFileSync(path.join(DIR, file), "utf8"));
  for (const key in defaultLocale) {
    if (!locale[key]) {
        errors.push(`Key ${key} is missing in ${file}`);
    }
  }
}

if (errors.length > 0) {
  console.error("Errors found:");
  for (const error of errors) {
    console.error(error);
  }
  process.exit(1);
}
console.log("All locales are verified");