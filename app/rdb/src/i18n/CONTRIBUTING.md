# Contributing to Localization

**NOTE**: I only plan to support languages that are supported by the game.

If you would like to improve or add missing translations, please read this first.

Preferably, you should have a GitHub account and contribute by opening a Pull Request with the updated translation.

## Status
The following languages need owner for localization:

- Spanish | es-ES
- Italian | it-IT
- Japanese | ja-JP
- Korean | ko-KR
- Dutch | nl-NL
- Russian | ru-RU 
- Traditional Chinese | zh-TW

## Guidelines
Generally, please follow these steps to contribute:
### 1. See what is already done
You can find the localization files in `app/rdb/src/i18n/locales/`. Look for the language you want to contribute to.
I already generated some entries from the game's data.

### 2. Items
The items are auto-generated. Please check if the translations are correct. **If not, do not edit them**, open an issue instead.

### 3. Modifiers
The modifiers are mostly done from the game's data. However, there are a few things that need cleaning up. Please look at the `#FIXME` comments.

### 4. UI
The UI texts need to be written from scratch. Stubs were copied from `en-US.yaml` and you need to change them. For the UI languages, it does not have to exactly word-for-word. You should make it look natural in the UI context.

You can test the UI translation by following the steps below

## Setting up localization side-loading
Side-loading makes it easier to test WIP localizations as you can see the results in the app.

1. Fork and clone the repository. Find the language files in `app/rdb/src/i18n/locales/`.
2. Once you started the application once, there will be a `config.yaml` file next to the executable. Open it and change:
    ```yaml
    localization_override: null
    ```
    To
    ```yaml
    localization_override: "path/to/your/app/rdb/src/i18n/locales/xx-XX.yaml"
    ```
    Replace the path above with the **absolute path** (or relative to the config file) to the language file you want to work on.
3. Restart the application. The app should now use the side-loaded language file. If not, there might be a syntax error in the file.
4. When you change the file, you can change to another language in the language picker. That will cause the app to reload the language file.

## Optimizing Item Search
Currently, the item search is a simple fuzzy search using the item name. If your language as a preferred search method, please let me know, and I will see if I can implement it. (Or implement it yourself and open a PR)

You can see the Pinyin search I implemented for Simplified Chinese as an example.

## Enabling the language
Lastly, the language needs to be uncommented in `app/rdb/src/i18n/locales/locales.ts` to show in the language picker.
