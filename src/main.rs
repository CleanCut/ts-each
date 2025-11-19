use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::process::Command;

/// What we care about from the JSON output of `tailscale status --json`.
#[derive(Deserialize)]
struct Status {
    /// Tailscale calls its list of connected devices "peers", under a "Peer" json key.
    #[serde(rename = "Peer")]
    peers: HashMap<String, Peer>,
}

/// What we care about from each tailscale peer (host) in the JSON output.
#[derive(Deserialize)]
struct Peer {
    #[serde(rename = "HostName")]
    host_name: String,
    #[serde(rename = "Online")]
    online: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // Running without any arguments lists all online hosts.
    let mut prefix = "";
    // Running with an argument filters hosts by the given prefix.
    if args.len() >= 2 {
        prefix = &args[1];
    }

    let output = Command::new("/Applications/Tailscale.app/Contents/MacOS/Tailscale")
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
        println!("--- {} ---", host_name);
        let output = Command::new("ssh")
            .arg(&host_name)
            .args(command_args)
            .output()?;

        print!("{}", String::from_utf8_lossy(&output.stdout));
        //eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
