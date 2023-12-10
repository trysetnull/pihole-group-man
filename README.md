# Pihole Group Management

`pihole-group-man` is a utility to administrate groups associated with a client in PiHole's gravity database.

See the [web](./web) directory to demonstrate how it could be integrated with a webpage to enable/disable internet access of a given client.

## Build and Test

This is a rust project and when building for the PiHole 4B requires an `aarch64` toolchain.
Build the software using cargo:

```shell
cargo build --release --target=aarch64-unknown-linux-gnu 
```

There is an integration test to validate the API which can be run, again using cargo:

```shell
cargo test --test integration_test -- --nocapture
```

## Usage

See the help:

```shell
Simple program to administrate groups of a client

Usage: pihole-group-man [OPTIONS] <COMMAND>

Commands:
  append  Appends client to group
  remove  Removes client from group
  help    Print this message or the help of the given subcommand(s)

Options:
      --database-path <DATABASE_PATH>  Filepath to the pi-hole gravity database [default: /etc/pihole/gravity.db]
  -v, --verbose...                     Make the operation more talkative
  -s, --silent                         Silent mode
  -h, --help                           Print help
  -V, --version                        Print version
```

Should the program return an 0 exit code then the FTL DNS server should be restarted:

```shell
pihole restartdns reload-lists
```
