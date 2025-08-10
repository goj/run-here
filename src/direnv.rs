use std::{collections::HashMap, env};

use anyhow::Result;

pub fn apply_direnv() -> Result<()> {
    let output = std::process::Command::new("direnv")
        .arg("export")
        .arg("json")
        .output()?
        .stdout;
    if output.is_empty() {
        return Ok(());
    }
    let env_vars: HashMap<String, String> = serde_json::from_slice(&output)?;
    for (key, value) in env_vars.iter() {
        env::set_var(key, value);
    }
    Ok(())
}
