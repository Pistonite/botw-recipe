# botw-recipe-generated

Static code generated from botw-research-scripts.

To regenerate/update:
- Install `pyyaml`
    ```bash
    pip install pyyaml
    ```
- Make sure packages/research-scripts is installed and built
    ```bash
    cd packages/research-scripts
    task install build
    ```
- Run the python script(s) in this directory to regenerate the parts you need
    ```bash
    python build.py
    python build_multichoose.py
    ```

## Rust Crate
If you are looking to integrate the cooking simulator, you are
likely looking for [`botw-recipe`](../core-lib) instead of this crate.

However if you do need to use this, or if
you need extra features not enabled, you can add it directly to your project
```bash
cargo add botw-recipe-sys --git https://github.com/Pistonite/botw-recipe --features full
```

Use `--features full` if compile speed or binary size is not a concern for you.
Otherwise, you might want to only include the features you need

### Feature Sets
The feature are grouped into sets. Enabling a feature set will
enable all feature flags in the set:

- `full`: enable everything
  - `cook-item-full`: enable `CookItem` enum and all its features
  - `actor-full`: enable `Actor` enum and all its features
  - `actor-wmc-group`: enable `Group` enum
  - `tag-full`: enable `Tag` enum and all its features. Note only cooking-related tags are included
  - `recipe`: enable `Recipe` struct and recipe data
  - `actor-data`: enable actor data (parameters, tags, etc)
  - `multichoose`: enable Multichoose utilities for WMC recipe database related function

#### Enum features
The `CookItem`, `Actor` and `Tag` enums have the following features
(* can be either `cook-item`, `actor` or `tag`)
- `*-enum-map`: Enable `EnumMap` implementation (enum as key in a map)
- `*-enum-set`: Enable `EnumSetType` implementation (enum as key in as bitset)
- `*-to-actor`/`tag-to-str`: Enable converting the enum to string representation
- `*-from-actor`/`tag-from-str`: Enable converting string to enum
- `*-serde`: Enable serializing/deserializing the enum as string.
  - `*-serde-serialize` and `*-serde-deserialize` can be used to control serialize/deserialize separately
- `*-english`: Enable english names for `CookItem` and `Actor`

#### Addtional features

- `prompt-entanglement`: Enable functions on `Actor` and `Group` to check if 
  a material requires Prompt Entanglement to hold

The feature sets of the generated data are broken down
by type and by feature. The type and feature are combined
to form a feature flag, for example `cook-item`(type) + `-to-actor`
(feature) = `cook-item-to-actor`

The types are:
- `cook-item`: The `CookItem` enum, for output actors that the recipe data contains
  - Note this does not include `Item_Cook_` actors that are removed (i.e. not in the recipe data)
- `actor`: The `Actor` enum, for selected input actors that can be put into the cooking pot
  - Currently this includes
    - Materials, except for monster extract
    - Holdable food
    - `dyecolor_*` actors
    - `Obj_Photo_*` actors
    - `Obj_DRStone_Get` (Sheika slate)
- `group`: The `Group` enum, grouping the `actors` into equivalence classes
  - `botw-research-scripts` computes all recipe groups, but the uncookable ones are excluded:
    - Any Weapon/Bow/Shield/Arrow/Armor by profile
    - Cook outputs (`Item_Cook_*`)
    - Duplicated material actors (For example, Stamina Bass x5), `Item_Enemy_Put_57`
    - Different arrow bundles (such as `Obj_AncientArrow_B_01`)
    - Itemized jewerly armor (`Obj_Head_02X` where `X` is `4-9`)
    - `Obj_Cushion`
     

The features are:
- `-enum-map`: Enable using the enum as key for the `enum-map` crate
- `-enum-set`: Enable using the enum as key for the `enumset` crate
- `-to-actor`: Enable converting the enum to actor name strings
- `-from-actor`: Enable converting from string actor name to the enum
- `-serde-serialize-actor`: Enable `serde::Serialize` implementation for the enum that serializes to the actor name string
- `-serde-serialize-value`: Enable `serde::Serialize` implementation for the enum that serializes to the integer representation
  - Conflicts with `-serde-serialize-actor`
  - Note the value might not be stable across version updates
- `-serde-deserialize-actor`: Enable `serde::Deserialize` implementation to deserialize string actor name to the enum
- `-serde-deserialize-value`: Enable `serde::Deserialize` implementation to deserialize integer representation to the enum
  - Note the value might not be stable across version updates
- `-english`: Enable getting the english name of the actor
