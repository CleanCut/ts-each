# `ts-each`

A simple utility that runs a command on multiple remote hosts via [tailscale]/[SSH](https://www.openssh.org/), displaying the output. Supports macOS, Linux, Windows, and any Unix-like OS that has a `tailscale` command-line client in the path.

## Installation

You should already have [Tailscale] installed and working.

You can install the latest release of `ts-each` using Cargo:

```bash
cargo install ts-each
```

Or to build from source, clone the repository and run:

```bash
cargo install --path .
```

## Usage

Without any arguments, `ts-each` lists all available Tailscale SSH hosts:

```bash
ts-each
```

With a single argument, it lists all matching Tailscale SSH hosts that start with the provided prefix:

```bash
ts-each <host-prefix>

# For example, to list all hosts starting with "db-production-", do

ts-each db-production-
```

With multiple arguments, the first argument is the host prefix and the rest of the arguments are the command to execute on each matching host:

```bash
ts-each <host-prefix> <command> [args...]

# For example, to run `uptime` on all hosts starting with "web-", do
ts-each web- uptime
```

## Contribution

All software contributions are assumed to be dual-licensed under MIT/Apache-2.

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [`license/APACHE`](license/APACHE) and [`license/MIT`](license/MIT).

[Tailscale]: https://tailscale.com/
