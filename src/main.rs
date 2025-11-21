use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What we care about from the JSON output of `tailscale status --json`.
#[derive(Deserialize)]
struct Status {
    /// Tailscale calls its list of connected devices "peers", under a "Peer" json key.
    #[serde(rename = "Peer")]
    peers: Option<HashMap<String, Peer>>,
}

/// What we care about from each tailscale peer (host) in the JSON output.
#[derive(Deserialize)]
struct Peer {
    #[serde(rename = "HostName")]
    host_name: String,
    #[serde(rename = "Online")]
    online: Option<bool>,
}

fn tailscale_executable() -> Result<PathBuf> {
    let mut executable = PathBuf::from("tailscale");
    if tailscale_works(&executable) {
        return Ok(executable);
    }
    if cfg!(target_os = "macos") {
        executable = PathBuf::from("/Applications/Tailscale.app/Contents/MacOS/Tailscale");
        if tailscale_works(&executable) {
            return Ok(executable);
        }
    }
    Err(anyhow::anyhow!("Tailscale executable not found"))
}

fn tailscale_works(path: &Path) -> bool {
    let mut command = Command::new(path);
    command.arg("--version");
    let Ok(output) = command.output() else {
        return false;
    };
    output.status.success()
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Running without any arguments lists all online hosts.
    let mut prefix = "";
    // Running with an argument filters hosts by the given prefix.
    if args.len() >= 2 {
        prefix = &args[1];
    }

    let executable = tailscale_executable()?;
    let output = Command::new(&executable)
        .arg("status")
        .arg("--json")
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Tailscale command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let status: Status = serde_json::from_slice(&output.stdout)?;

    let mut host_names: Vec<String> = status
        .peers
        .unwrap_or_default()
        .values()
        .filter(|p| p.online.unwrap_or(false))
        .filter(|p| p.host_name.starts_with(prefix))
        .map(|p| p.host_name.clone())
        .collect();

    host_names.sort();

    // If no arguments were provided, or only a prefix was provided, print all online hosts.
    if args.len() <= 2 {
        for host_name in host_names {
            println!("{}", host_name);
        }
        return Ok(());
    }

    // A command was provided, so execute it for each matching host.
    let command_args = &args[2..];

    for host_name in host_names {
        println!("{}", format!("--- {} ---", host_name).blue());
        let output = Command::new("ssh")
            .arg("-tt")
            .arg(&host_name)
            .args(command_args)
            .output()?;

        let have_stderr = output.stderr.len() > 0;
        if have_stderr {
            println!("{}", "stdout:".green());
        }
        print!("{}", String::from_utf8_lossy(&output.stdout));
        if have_stderr {
            eprintln!("\n{}", "stderr:".red());
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(())
}
