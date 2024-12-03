# Advent of Code 2024

These are my solutions to [Advent of Code 2024](https://adventofcode.com/2024).

## `Makefile` Usage

### Prerequisites

- `cargo`
- A `SESSION_COOKIE` environment variable whose value is `session=<your session cookie>`. You can get your session cookie by pulling it from the Storage tab in your browser's developer tools or by watching the network tab while you navigate the site.

### Create a new day

```bash
make new
```

This runs `cargo new`, downloads the input for the day, and commits everything.
