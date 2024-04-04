use std::{io::{BufRead, Write}, process::Command};
use crate::{Error, Result};

pub fn get() -> Result<String>{
let monitor = Command::new("hyprctl monitors -j | jq -r '.[0].name'")
        .output().unwrap().stdout;

    let binding = Command::new("hyprctl hyprpaper listactive -j")
        .output().unwrap();
    let output = binding.stdout.as_slice().lines();

    for line in output {
        let line = line?;

        let _ = std::io::stdout().write_all(line.as_bytes());

        let monitor_str = String::from_utf8_lossy(&[monitor[0]]).to_string();

        if line.clone().starts_with(&monitor_str) {
            let path = line.strip_prefix(&(monitor_str.clone() + " ="));

            return Ok(path.unwrap().to_owned());
        }
    }

    return Err(Error::NoImage("Hyprland"))
}