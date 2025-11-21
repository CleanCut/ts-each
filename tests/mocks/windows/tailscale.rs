use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args[0] is the executable path
    if args.len() >= 3 && args[1] == "status" && args[2] == "--json" {
        println!(
            r#"{{
  "Peer": {{
    "1": {{ "HostName": "app-prod-1", "Online": true }},
    "2": {{ "HostName": "app-prod-2", "Online": true }},
    "3": {{ "HostName": "app-staging-1", "Online": true }}
  }}
}}"#
        );
        return;
    }

    // Pass through to real tailscale
    let status = Command::new(r"C:\Program Files\Tailscale\tailscale.exe")
        .args(&args[1..])
        .status();

    match status {
        Ok(s) => std::process::exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("Failed to execute real tailscale: {}", e);
            std::process::exit(1);
        }
    }
}
