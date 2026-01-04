# uuid-utils

## Overview

I'm tired of following:
- using webapp/REPL to generate UUID
- manually formatting UUID via `echo '<UUID>' | tr -d '-' | tr '[:lower:]' '[:upper:]'`

## Usage

To generate run `uuid-utils gen`.

To format run `uuid-utils fmt <UUID> --compact`.

For detailed information about options look at [usage doc](./docs/usage.md).

## License

[MIT](./LICENSE)
