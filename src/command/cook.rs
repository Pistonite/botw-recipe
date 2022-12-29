use crate::data;
use crate::recipe;
use crate::view;

pub fn cook(cmd: &[&str], converter: &recipe::RecipeConverter, crit_mgr: &mut data::CritMgr) -> Result<(), String> {
    let ids = super::get_material_ids(cmd)?;

    let recipe_id = converter.to_recipe_id(&ids)?;

    let materials = converter.to_materials(recipe_id);
    let data = data::database::get_recipe_by_id(recipe_id)?;
    let crit_hp = crit_mgr.get_recipe_crit_hp(recipe_id, data.hp);

    let view = view::recipe_detail(&materials, data, crit_hp);

    println!("{}", &view);
    Ok(())
}
