use super::env::Environment;
use std::process::Command;
pub struct Mamba {}

impl Environment for Mamba {
    fn create(&self) {
        Command::new("touch")
            .args(["env.yml"])
            .spawn()
            .expect("Failed to create a mamba env");
        Command::new("mamba")
            .args(["create", "-p", ".env"])
            .spawn()
            .expect("Failed to create a mamba env");
        return;
    }

    fn update(&self) {
        Command::new("mamba")
            .args(["env", "update", "-f", "env.yml", "-p", ".env"])
            .spawn()
            .expect("Failed to create a mamba env");
        return;
    }
}
