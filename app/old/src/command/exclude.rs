use bit_set::BitSet;

use crate::recipe;
use crate::view;

pub fn exclude_add(
    cmd: &[&str],
    converter: &recipe::RecipeConverter,
    excluded_materials: &mut BitSet,
) -> Result<(), String> {
    parse_materials(cmd, converter, "The following materials are now excluded:").map(|new_ids| {
        excluded_materials.union_with(&new_ids);
    })
}

pub fn exclude_remove(
    cmd: &[&str],
    converter: &recipe::RecipeConverter,
    excluded_materials: &mut BitSet,
) -> Result<(), String> {
    parse_materials(
        cmd,
        converter,
        "The following materials are no longer excluded:",
    )
    .map(|new_ids| {
        excluded_materials.difference_with(&new_ids);
    })
}

fn parse_materials(
    cmd: &[&str],
    converter: &recipe::RecipeConverter,
    info: &str,
) -> Result<BitSet, String> {
    let ids = super::get_material_ids(cmd)?;

    println!("{}", info);
    let mut new_ids = BitSet::new();
    for id in ids {
        new_ids.insert(id);
        let text = view::material(id, &converter.get_material_name(id));
        println!("- {}", text)
    }
    Ok(new_ids)
}
