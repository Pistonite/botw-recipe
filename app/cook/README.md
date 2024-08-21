cooking.rs
----------

Cooking simulator for BotW

Originally from [savage13/cooking.rs](https://github.com/savage13/cooking.rs),
modified to be:
- more effect
- use the same data set as the rest of the botw-recipe project


Example
-------

Example from `src/cli.rs` which is the default binary along side the library

```rust
use cooking::Cook;

fn main() {
    let cook = Cook::new();
    let recipe = cook.cook(&["Apple"]);
    println!("{:?}", recipe);

    let recipe = cook.cook(&["Fairy"]);
    println!("{:?}", recipe);

    let items = ["Apple", "Fairy", "Swift Carrot", "Apple"];
    let recipe = cook.cook(&items);
    println!("{:?}", recipe);
}
```

License
-------

BSD 2-Clause
