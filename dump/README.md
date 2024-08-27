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
the total number of recipe is:
```
binomial(N+5-1, 5) = binomial(N+4, 5) // or multichoose(N, 5)
```
All possible recipe inputs can be generated with an algorithm. This means
each recipe has a deterministic position in the output of the algorithm.
We use that as the ID of the recipe.

The recipe ID = 0 corresponds to choosing 5 `None` ingredients. 
This is not a valid recipe, just a placeholder.

See `/app/data/src/recipe.rs` for an efficient algorithm to convert between
recipe inputs and recipe ID.

Having recipe ID like this allows us to efficiently pack the database.
Essentially, we don't need to store the input, just the output, and the
input can be inferred from the position of the output in the entire list.

## Raw record format
The raw data record for each recipe will have this following format:
```c
struct CookData {
    // Number of quarter-hearts (or yellow quarter-hearts), usually 0-120
    int32_t health_recover;
    // Effect seconds, usually 0-1800
    int32_t effect_duration;
    // Price when selling, usually > 0
    int32_t sell_price;
    // Effect ID but as an float. -1 is None, but some may have 0.0 as None
    float32_t effect_id;
    // Effect level:
    // - potion effect: integer level, usually 0-3
    // - hearty effect: number of yellow quarter-hearts
    // - stamina effect: number of wheels * 1000 (3000 max)
    // - endura effect: number of wheels * 5 (technically 15 max, but in-game max is 12)
    float32_t effect_level;
    // Chance of critical success, 0-125, note that the game does not cap
    // this value, but >=100 guarantees critical success
    int32_t crit_chance;
}
```
Note that the layout is the same as `uking::ui::PouchItem::CookData` in
BOTW, with an extra `crit_chance` in the end.

The record for recipe ID = 0 should be 24 bytes of `0x00`.

Each record is 24 bytes, and the total database should be around 40GB

## Chunking
To allow efficient processing, the database will be divided into smaller
sequencial chunks. The chunk size is selected so that there aren't too many
chunks, and each chunk efficiently uses disk space.

## Storage format
Each chunk will be stored as a binary file. Each record will be stored as 
a binary blob of 24 bytes, using C-layout and little endian for the fields.

## Dumping
The database will be dumped by 2 means: emulated and console.

The emulated dumper is adapted from https://github.com/savage13/cooking.rs.
The performance is massively (20x) improved by using generated enum instead of string.
Any inconsistency needs to be fixed to match the console dumper.

This dumper is implemented in `/dump/emulate`.
On my machine, it takes around 2 minutes to dump the entire database.

The console dumper uses a mod to call the `cook()` function in the game,
and write the result to the console's SD card which can be retrieved later.
Having smaller chunks here helps to prevent data corruption/crashes.
This dumper is implemented in `/dump/console`.
On my switch it takes around 1.5 minutes on average to dump a chunk.
It will take around 90 switch-hours to dump the entire database.

## Validation
The data will be cross-validated. The dumps from 2 methods must exactly match, byte-by-byte

## Compacting the Database
The raw database contains extra information that's not needed for WMC. In WMC, the only 2
fields that matter are `health_recover` and `sell_price`. `health_recover` is transmutated
into `modifier_value`, and `sell_price` into `modifier_flags`, which indicates what
modifiers are applied.

Since the range for `health_recover` is `0<=hp<=120`, it takes 7 bits to store it.
For the modifier flag, we can exclude the yellow modifier, which is `0x80000000` and is
only possible with a negative price. This leaves the rest of the 9 flags storable in 9 bits.

With this transformation, every record can be stored in 16 bites, or 2 bytes, compared to 24 bytes.
The upper 7 bits will be `health_recover`, and the lower 9 bits will be the lower 9 bits of `sell_price`.

However, there's one extra piece of information - critical boost. In WMC, we only really
care if it's possible for the recipe to have random boost to `health_recover` by critting,
which is the definition of the `crit_rng_hp` field in the compacted DB.

There are generally 3 cases:
1. It's impossible for the recipe to crit (`crit_chance` = 0)
2. It's guaranteed for the recipe to crit, AND it's guaranteed to be a heart crit, AND the heart crit will increase `health_recover`
3. Either crit_chance is `0 < x < 100`, or it's guaranteed to crit but not guaranteed to be a heart crit

In case 1 and 2, `crit_rng_hp` is false. In case 3, `crit_rng_hp` is true.
In case 2, the `health_recover` stored in the compact DB will be the value after crit.

The `crit_rng_hp` flag will be stored separately from the main record in order to keep
the main record nicely aligned. It will be compacted into 1 bit per record with little bit-endianless
(i.e. 0x01 means the 0-th record has `crit_rng_hp` = true).

The `crit_rng_hp` flag will be calculated purely using the simulator, by brute-forcing all possible
paths the recipe can go through, and comparing if the output hp are different.

## Chunking and Indexing
With the size of the record reduced, we can put more records into a chunk. The new chunk size
will be `3276800` records per chunk.

We don't want to put too many records into the chunks, because it will be less effective to index
the chunks. Indexing essentially pre-computes metadata for the chunk, so certain chunks can be skipped directly
when searching.

The index will contain the following metadata for each chunk:
- The minimum hp of all recipes
- The maximum hp without crit_rng_hp of all recipes
- The maximum hp with crit_rng_hp of all recipes
- A bit mask for whether a modifier is enabled by any record
- A bit mask for whether a modifier is enabled by all records
- A boolean for whether all recipes contain at least one material only holdable with PE
- A (hp, price, crit_rng_hp) tuple if all records are the same.
- SHA-256 hash of the chunk
