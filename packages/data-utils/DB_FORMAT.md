# Recipe data format and dumping

## Group ID
Items (actors) are grouped into ingredient groups. Each actor in a group
behaves the same when cooked. For example, the 3 dragon horns, or any key item.
This significantly reduces the number of recipes needed to be stored.

See [group.rs](../generated/src/group.rs) for the groups and their ID.
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
This is not possible to obtain in game. However, it does have a special
handler in the game to output a dubious food with that sells for 1 rupee

See [Mnr](../generated/src/multichoose/mod.rs) for the implementation
to convert between recipe inputs and recipe ID.

Having recipe ID like this allows us to efficiently pack the database.
Essentially, we don't need to store the input, just the output, and the
input can be inferred from the position of the output in the entire list.

## Database Formats
There will be 3 sets of database:
- **Raw DB**: This is the largest database in file size and stores the raw output
  of recipes. Each record is 24 bytes
- **Crit DB**: This stores the critical success information of each recipe. Each record
  is 2 bits
- **Compact DB**: This only stores the information relevant to WMC - HP and (important bits of ) price.
  Each record is 2 bytes. 

The Raw DB and Crit DB are used to verify the correctness of the cooking simulator during development,
and the Compact DB is used by the searcher app. If additional information of the recipe is needed while searching,
it is computed on the fly with the cooking simulator.

### Raw DB Record
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

Each record is 24 bytes, and the total database should be around 40GB.
Each field is stored in little endian.

### Crit DB Record
Each record in the crit DB is only 2 bits:
- `0b00` - `NoRng`. The recipe always outputs the same HP
- `0b01` - `Normal`. The recipe follows normal Crit RNG.
  The Raw DB stores the base, non-crit HP, and crit HP is either +12 for non-hearty food
  and +4 for hearty food
- `0b11` - `Monster`. The recipe contains monster extract AND follows monster RNG.
  The Raw DB stores the base, non-changed HP. Crit HP is either +12 or setting the HP to 1 for non-hearty food,
  and either +4 or setting HP to 4 for hearty food

Each record is stored in the lower 2 bits of a byte. This makes the size 4x
as large. However, size is not a concern since Crit DB is not shipped in the
searcher app.

### Compact DB Record
Each record in the compact DB is 16 bits:
- Lower 9 bits is the lower 9 bits of the price, or the modifier bitflags
- Upper 7 bits is the HP (0-127). Since the max of HP is 120, the special
  value of 121 is used to indicate the recipe has `Monster` RNG. The real
  HP is computed on the fly with the simulator. If the recipe does not have
  HP crit rng (`NoRng`), then this field is the HP after crit is applied

## Chunking
To allow efficient parallel processing, the database will be divided into smaller
sequencial chunks. The chunk size is selected so that there aren't too many
chunks, and each chunk efficiently uses disk space.

See [meta.rs](../wmcdb/src/meta.rs) for the parameters used for chunking

## Indexing
The Compact DB ships with one extra file - The index. It is essentially pre-computed
metadata for the chunk, so certain chunks can be skipped entirely when searching.

The index will contain the following metadata for each chunk:
- The minimum hp of all recipes
- The maximum hp without crit_rng_hp of all recipes
- The maximum hp with crit_rng_hp of all recipes
- A bit mask for whether a modifier is enabled by any record
- A bit mask for whether a modifier is enabled by all records
- A boolean for whether all recipes contain at least one material only holdable with PE
- A (hp, price, crit_rng_hp) tuple if all records are the same.
- SHA-256 hash of the chunk
