use botw_recipe_data::{Actor, RecipeInputs};
use cooking::Cook;

fn main() {
    let inputs = RecipeInputs::from_actors(&[
        Actor::try_from("fleet-lotus seeds").unwrap(),
        Actor::try_from("fleet-lotus seeds").unwrap(),
        Actor::try_from("fleet-lotus seeds").unwrap(),
        Actor::try_from("fleet-lotus seeds").unwrap(),
        Actor::try_from("shard of farosh's horn").unwrap(),
    ]);
    let mut ingr = Vec::new();
    for group in inputs.iter() {
        let actor = group.first_actor();
        if actor != Actor::None {
            ingr.push(actor.name());
        }
    }
    println!("{:?}", ingr);
    let mut cook = Cook::new().unwrap();
    cook.set_verbose(true);
    let recipe = cook.cook(&ingr).unwrap();
    println!("{:#?}", recipe);
}
