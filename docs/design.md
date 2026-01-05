# Design

Here is a brief overview of the design decisions made for uuid-utils.
To avoid reimplementing the bicycle we use the `uuid` crate.

## Formatters

UUID formatters are placed in `./src/formatters.rs`. 
Each format (e.g. canonical, compact or hex) has its own formatter for expandability.

## Generators

UUID generators are placed in `./src/generators.rs`. 
Each generator (e.g. v4, v5 or v6) has its own generator for expandability.

## Subcommands

Each subcommand (`gen`, `fmt`, `info`, `nil`, etc.) places its own implementation in `./src/subcommands/`.
