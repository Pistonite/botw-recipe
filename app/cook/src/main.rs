use botw_recipe_cook::{CookingPot, Error, Options};
use clap::Parser;

fn main() {
    if let Err(e) = cli() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn cli() -> Result<(), Error>{
    let options = Options::parse();
    let actors = match options.get_actors() {
        Ok(actors) => actors,
        Err(Error::AmbiguousIngr(input, actors)) => {
            println!("Ambiguous ingredient: {}", input);
            println!("Possible actors: {:#?}", actors);
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    println!("Ingredients are:");
    for actor in &actors {
        println!("  - {:?}", actor);
    }
    let pot = CookingPot::new()?;
    let output = pot.cook_actors(actors)?;
    println!("Cooked: {:#?}", output);

    Ok(())
}
