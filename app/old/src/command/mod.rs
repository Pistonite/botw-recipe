use crate::recipe;

mod cook;
pub use cook::cook;

mod inspect;
pub use inspect::inspect;

mod exclude;
pub use exclude::exclude_add;
pub use exclude::exclude_remove;

pub fn get_u32_arg(cmd: &[&str], arg_name: &str) -> Result<u32, String> {
    if cmd.len() < 2 {
        return Err(format!(
            "Error: command needs 1 non-negative integer argument: {}",
            arg_name
        ));
    }

    let arg = cmd[1];
    arg.parse()
        .map_err(|_| format!("Error: {} must be non-negative integer: {}", arg_name, arg))
}

pub fn get_arg(cmd: &[&str], arg_name: &str) -> Result<String, String> {
    if cmd.len() < 2 {
        return Err(format!("Error: command needs 1 argument: {}", arg_name));
    }

    Ok(String::from(cmd[1]))
}

pub fn get_arg_pair(
    cmd: &[&str],
    arg_name1: &str,
    arg_name2: &str,
) -> Result<(String, String), String> {
    if cmd.len() < 3 {
        return Err(format!(
            "Error: command needs 2 arguments: {} and {}",
            arg_name1, arg_name2
        ));
    }

    Ok((String::from(cmd[1]), String::from(cmd[2])))
}

pub fn get_material_ids(cmd: &[&str]) -> Result<Vec<usize>, String> {
    let mut ids = vec![];

    for cmd_part in cmd {
        let id: usize = match cmd_part.parse() {
            Ok(id) => id,
            Err(_) => {
                return Err(format!("{} is not a valid integer", cmd_part));
            }
        };
        if !recipe::is_valid_item(id) {
            return Err(format!("{} is not a valid item id", cmd_part));
        }
        ids.push(id);
    }

    Ok(ids)
}
