# Quake Log Parser

This is a CLI tool to parse and summarize Quake games from a log file.

## Usage

First, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and setup in
your environment. Then simply clone this repository, and run the application, feeding it a Quake
log file from stdin. Example:

```sh
cargo run -- < tests/qgames.log
```

You should get a JSON report of kills in the games printed to stdout.

## Approach

The project focuses on reading the following events from the logs:

- `InitGame`: This event is read to start a new game.
- `ClientUserinfoChanged`: This event is read to identify a new connecting player name and add it
  to the game summary.
- `Kill`: This event is read to process a game kill.

Every other event is ignored by the parser for simplicity.

## External dependencies used

The [serde](https://docs.rs/serde/latest/serde/) struct serializer/deserializer framework and its
JSON library [serde_json](https://docs.rs/serde_json/latest/serde_json/) were used in this project
to safely serialize the output into JSON format.

The [assert-cmd](https://crates.io/crates/assert_cmd) crate was used as a development dependency,
for CLI end to end tests.

## Tests

You can run the application's unit and end to end tests with the following command:

```sh
cargo test
```

## License

[MIT](./LICENSE.md)
