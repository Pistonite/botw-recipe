# Recipe data format and dumping

## Group ID
Items (actors) are grouped into ingredient groups. Each actor in a group
behaves the same when cooked. For example, the 3 dragon horns, or any key item.
This significantly reduces the number of recipes needed to be stored.

See `/research/output/ids.yaml` for the groups and their ID.
The group with ID = 0 corresponds to the `None` group, which indicates
the absence of an ingredient.

## Recipe ID
Given the groups `N`, and 5 is the max number of ingredients in a recipe,
the totel number of recipe is:
```
binomial(N+5-1, 5) = binomial(N+4, 5) // or multichoose(N, 5)
```
All possible recipe inputs can be generated with an algorithm. This means
each recipe has a deterministic position in the output of the algorithm.
We use that as the ID of the recipe.

The recipe ID = 0 corresponds to choosing 5 `None` ingredients. 
This is not a valid recipe, just a placeholder.

see `/app/data/src/lib.rs` for an efficient algorithm to convert between
recipe inputs and recipe ID.

Having recipe ID like this allows us to effciently pack the database.
Essentially, we don't need to store the input, just the output, and the
input can be inferred from the position of the output in the entire list.

## Raw record format
The raw data record for each recipe will have this following format:
```c
struct CookData {
    // Number of quarter-hearts, usually 0-120
    int32_t health_recover;
    // Effect seconds, usually 0-1800
    int32_t effect_duration;
    // Price when selling
    int32_t sell_price;
    // Effect ID but as an float. -1 is None
    float32_t effect_id;
    // Effect level, 0f-3f for most effects, hearty is the number of yellow hearts
    float32_t effect_level;
    // Chance of critical success, usually 0-100
    int32_t crit_chance;
}
```
Note that the layout is the same as `uking::ui::PouchItem::CookData` in
the BOTW game, with an extra `crit_chance` in the end.

The record for recipe ID = 0 should be 24 bytes of `0x00`.

Each record is 24 bytes, and the total database should be around 40GB

## Chunking
To allow efficient processing, the database will be divided into smaller
sequencial chunks. The chunk size is selected so that there aren't too many
chunks, and each chunk efficiently uses disk space.

The selected chunk size is `409600` records. There will be around 4000 chunks.

## Storage format
Each chunk will be stored as a binary file. Each record will be stored as 
a binary blob of 24 bytes, using C-layout and little endian for the fields.

## Dumping
The database will be dumped by 2 means: emulated and console.

The emulated dumper uses https://github.com/savage13/cooking.rs with
an adapter to convert the output to the raw record format above.
This dumper is implemented in `/dump/emulate`

The real dumper uses a mod to call the `cook()` function in the game,
and write the result to the console's SD card which can be retrieved later.
Having smaller chunks here helps to prevent data corruption/crashes.
This dumper is implemented in `/dump/console`

## Validation
The data will be cross-validated. The dumps from 2 methods must exactly match.
