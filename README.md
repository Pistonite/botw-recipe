# ![logo](app/rdb/src-tauri/icons/32x32.png) botw-recipe

![Build Badge](https://img.shields.io/github/check-runs/Pistonite/botw-recipe/main)
![License Badge](https://img.shields.io/github/license/Pistonite/botw-recipe)
![Issue Badge](https://img.shields.io/github/issues/Pistonite/botw-recipe)
![Downloads Badge](https://img.shields.io/github/downloads/Pistonite/botw-recipe/total)
![Recipe Badge](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/Pistonite/botw-recipe/main/dump/emulate/badge.json)

The fast, accurate and complete Breath of the Wild Recipe Database for Weapon Modifier Corruption (WMC).

Previous tools for searching WMC recipes use brute-force method to iterate through all possible combinations and check if each recipe match the modifiers. This project pre-computes ALL possible recipes using the fastest BOTW cooking simulator and stores them in a compact, binary format for extremely fast search

Please see the [releases](https://github.com/Pistonite/botw-recipe/releases) for pre-built downloads for common systems. See below for build instructions

## Language Support
I am planning to support the languages supported by the game. 
If you would like to contribute, please see [CONTRIBUTING.md](app/rdb/src/i18n/CONTRIBUTING.md)

## Build
You need:
- Python
- Rust toolchain and compilers for your platform
- Node.js
- [Task](https://taskfile.dev/#/installation) (Optional)

Install dependencies
```
task rdb:install
```
Generating the database (takes several minutes)
```
task dump:dump -- -C
```
At this point, you can run the app using development mode
```
task rdb:dev -- --release
```

Without `task`, the commands are
```
cd dump/emulate
cargo run --release --bin rdump -- -C
cd ../../app/rdb
npm install
npx tauri dev --release --features devtools
```

To build the standalone binary (Output is in `/target/release`)
```
task rdb:build
```