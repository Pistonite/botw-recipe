use crate::data;
use crate::recipe;

enum Modifier {
    AttackUp = 1 << 0,
    DurabilityUp = 1 << 1,
    CriticalHit = 1 << 2,
    LongThrow = 1 << 3,
    Multishot = 1 << 4,
    Zoom = 1 << 5,
    QuickShot = 1 << 6,
    SurfUp = 1 << 7,
    GuardUp = 1 << 8,
}

pub fn material(id: usize, name: &str) -> String {
    format!("{} = {}", id, name)
}

pub fn recipe_detail(materials: &[String], data: data::RecipeData, crit_hp: u8) -> String {
    let mut buf = String::new();
    buf.push_str("- Recipe:\n");
    for material in materials {
        buf.push_str(&format!("  - {}\n", material));
    }
    buf.push_str(&format!("  Hp: {}\n", data.hp));
    buf.push_str(&format!("  HpCrit: {}\n", crit_hp));
    buf.push_str("  Modifiers:\n");

    for modifier in modifier_list(data.price) {
        buf.push_str(&format!("  - {}\n", modifier));
    }

    buf
}

pub fn modifier_list(modifiers: u16) -> Vec<&'static str> {
    let mut out = vec![];
    if modifiers & Modifier::AttackUp as u16 != 0 {
        out.push("Attack Up");
    }
    if modifiers & Modifier::DurabilityUp as u16 != 0 {
        out.push("Durability Up");
    }
    if modifiers & Modifier::CriticalHit as u16 != 0 {
        out.push("Critical Hit");
    }
    if modifiers & Modifier::LongThrow as u16 != 0 {
        out.push("Long Throw");
    }
    if modifiers & Modifier::Multishot as u16 != 0 {
        out.push("Multishot");
    }
    if modifiers & Modifier::Zoom as u16 != 0 {
        out.push("Zoom");
    }
    if modifiers & Modifier::QuickShot as u16 != 0 {
        out.push("Quick Shot");
    }
    if modifiers & Modifier::SurfUp as u16 != 0 {
        out.push("Surf Up");
    }
    if modifiers & Modifier::GuardUp as u16 != 0 {
        out.push("Guard Up");
    }

    out
}

pub fn modifier_str(modifiers: u16) -> String {
    let mut out = String::new();
    if modifiers & Modifier::AttackUp as u16 != 0 {
        out.push('A');
    }
    if modifiers & Modifier::DurabilityUp as u16 != 0 {
        out.push('D');
    }
    if modifiers & Modifier::CriticalHit as u16 != 0 {
        out.push('C');
    }
    if modifiers & Modifier::LongThrow as u16 != 0 {
        out.push('L');
    }
    if modifiers & Modifier::Multishot as u16 != 0 {
        out.push('M');
    }
    if modifiers & Modifier::Zoom as u16 != 0 {
        out.push('Z');
    }
    if modifiers & Modifier::QuickShot as u16 != 0 {
        out.push('Q');
    }
    if modifiers & Modifier::SurfUp as u16 != 0 {
        out.push('S');
    }
    if modifiers & Modifier::GuardUp as u16 != 0 {
        out.push('G');
    }

    out
}

pub fn modifier_flag(short: &str) -> u16 {
    let mut flag: u16 = 0;

    for char in short.chars() {
        let char = char.to_ascii_uppercase();
        match char {
            'A' => flag |= Modifier::AttackUp as u16,
            'D' => flag |= Modifier::DurabilityUp as u16,
            'C' => flag |= Modifier::CriticalHit as u16,
            'L' => flag |= Modifier::LongThrow as u16,
            'M' => flag |= Modifier::Multishot as u16,
            'Z' => flag |= Modifier::Zoom as u16,
            'Q' => flag |= Modifier::QuickShot as u16,
            'S' => flag |= Modifier::SurfUp as u16,
            'G' => flag |= Modifier::GuardUp as u16,
            _ => (),
        }
    }

    flag
}

/// View short hand of the query
pub fn query(query: &data::Query) -> String {
    format!(
        "[+:{}][-:{}][h:{}-{}][c:{}][e:{}]",
        modifier_str(query.include_modifiers),
        modifier_str(query.exclude_modifiers),
        query.minhp,
        query.maxhp,
        if query.crit { "on" } else { "off" },
        query.exclude_materials.len()
    )
}

/// View detail of query
pub fn query_detail(query: &data::Query, converter: &recipe::RecipeConverter) -> String {
    let mut list = vec![
        query_minhp_detail(query),
        query_maxhp_detail(query),
        query_crit_detail(query),
        query_modifier_detail(query),
        query_excluded_materials(query, converter),
    ];

    if query.minhp == 0 && query.maxhp == 120 {
        list.push(String::from(
            "WARNING: The range of hp values include all recipes. The result might be large.",
        ));
    }

    if query.include_modifiers == 0 && query.exclude_modifiers == 0 {
        list.push(String::from(
            "WARNING: You have not set a filter on the modifiers. The result might be large.",
        ));
    }

    list.join("\n")
}

/// View detail of minhp
pub fn query_minhp_detail(query: &data::Query) -> String {
    format!("Search will only include hp >= {}", query.minhp)
}

/// View detail of maxhp
pub fn query_maxhp_detail(query: &data::Query) -> String {
    format!("Search will only include hp <= {}", query.maxhp)
}

/// View detail of maxhp
pub fn query_crit_detail(query: &data::Query) -> String {
    if query.crit {
        String::from("Search will include rng heart crit")
    } else {
        String::from("Search will NOT include rng heart crit")
    }
}

/// View detail of maxhp
pub fn query_modifier_detail(query: &data::Query) -> String {
    let mut buf = String::new();
    if query.include_modifiers != 0 {
        buf.push_str("Search will only include recipes that have all of:\n");

        for modifier in modifier_list(query.include_modifiers) {
            buf.push_str(&format!("- {}\n", modifier));
        }
    }
    if query.exclude_modifiers != 0 {
        buf.push_str("Search will NOT include recipes that have any of:\n");

        for modifier in modifier_list(query.exclude_modifiers) {
            buf.push_str(&format!("- {}\n", modifier));
        }
    }

    buf
}

/// View detail of excluded materials
pub fn query_excluded_materials(
    query: &data::Query,
    converter: &recipe::RecipeConverter,
) -> String {
    let mut buf = String::new();
    if query.exclude_materials.is_empty() {
        return String::from("Search will include recipes with any material");
    }

    buf.push_str("Search will exclude recipes that contain any of:\n");
    for material_id in &query.exclude_materials {
        buf.push_str(&format!(
            "- {}\n",
            material(material_id, &converter.get_material_name(material_id))
        ));
    }

    buf
}
