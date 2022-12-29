use crate::recipe;
use crate::view;

pub fn inspect(cmd: &[&str], converter: &recipe::RecipeConverter) {
    let materials = converter.get_matching_materials(cmd);

    for material in &materials {
        let text = view::material(material, &converter.get_material_name(material));
        println!("{}", text)
    }
}
