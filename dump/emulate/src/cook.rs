use botw_recipe_data::{Actor, RecipeInputs};
use cooking::Cook;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <ingredient>...", args[0]);
        std::process::exit(1);
    }
    let inputs = args[1].split(',')
        .map(|s| Actor::try_from(s.trim()).unwrap())
        .collect::<Vec<_>>();
    let inputs = RecipeInputs::from_actors(&inputs);
    let mut ingr = Vec::new();
    for group in inputs.iter() {
        let actor = group.first_actor();
        if actor != Actor::None {
            ingr.push(actor.name());
        }
    }
    println!("{:?}", ingr);
    #[allow(unused_mut)]
    let mut cook = Cook::new().unwrap();
    cook.set_verbose(true);
    let recipe = cook.cook(&ingr).unwrap();
    let data = rdump_emulate::convert_recipe(&recipe);
    println!("{:#?}", recipe);
    println!("{:#?}", data);
}
