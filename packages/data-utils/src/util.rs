use anyhow::bail;

/// Print errors and return Ok if there's no error
pub fn check_errors<T: std::fmt::Display>(errors: &[(usize, T)]) -> anyhow::Result<()> {
    if errors.is_empty() {
        return Ok(());
    }
    let len = errors.len();
    for (chunk_id, e) in errors {
        eprintln!("Error in chunk {}: {}", chunk_id, e);
    }
    bail!("{} errors found", len);
}
