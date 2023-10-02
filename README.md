# Quake Log Parser

This is a CLI utility tool to parse Quake game log files and generate reports of kills in the games.

## Usage

First, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and setup in
your environment. Then simply clone this repository, and run the application, feeding it a Quake
log file by stdin, with the following command:

```sh
cargo run -- < qgames.log # TODO: verify this command
```

You should get a JSON report of kills in the game printed to stdout.

## Approach

The project focuses on reading the following events from the logs:

- `InitGame`: This event is read to start a new game.
- `ClientUserinfoChanged`: This event is read to identify a new connecting player name and add it
  to the game.
- `Kill`: This event is read to process a game kill.

Every other event is ignored by the parser for simplicity.

## External dependencies used

The [serde](https://docs.rs/serde/latest/serde/) struct serializer/deserializer framework and its
JSON library [serde_json](https://docs.rs/serde_json/latest/serde_json/) were used in this project
to safely serialize the output into JSON format.

The [assert-cmd](https://crates.io/crates/assert_cmd) crate was used as a development dependency,
for CLI end to end tests.
