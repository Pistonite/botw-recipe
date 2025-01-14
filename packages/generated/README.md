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

### Feature Flags
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
