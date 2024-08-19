use botw_recipe_data::{Actor, RecipeId, RecipeInputs};
use cooking::{Cook, Recipe};

fn main() {
    botw_recipe_data::init();
    // let recipe = RecipeId::new(700).unwrap();
    // println!("rid = {:?}", recipe);
    let recipe_inputs: RecipeInputs = RecipeInputs::from_actors(&[
        Actor::try_from("shard of farosh's horn").unwrap(),
        Actor::try_from("fleet-lotus seeds").unwrap()
    ]);
    println!("rin = {:?}", recipe_inputs);
    let mut ingr = Vec::new();

    for group in recipe_inputs.iter() {
        let actor = group.first_actor();
            if actor != Actor::None {
            ingr.push(actor.name());
        }
    }

    println!("names = {:?}", ingr);
    let cook = Cook::new();
    let result = cook.cook(&ingr);
    println!("result = {:#?}", result);
}

fn process_cooking_result(result: Recipe) {
}
