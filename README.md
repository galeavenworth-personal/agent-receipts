# agent-receipts

A tiny, reliable command wrapper CLI that executes arbitrary commands and emits execution receipts as JSON on disk.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/rcpt`.

## Usage

Execute a command and generate a receipt:

```bash
rcpt run <command> [args...]
```

### Options

- `--out <path>` - Output path for the receipt JSON file (default: `receipt.json`)

### Examples

```bash
# Basic command
rcpt run echo "Hello, world!"

# Command with arguments
rcpt run ls -la /tmp

# Custom output path
rcpt run --out my-receipt.json cargo build

# Complex shell command
rcpt run sh -c 'echo "stdout"; echo "stderr" >&2; exit 42'
```

## Receipt Format

The receipt is a JSON file containing:

- `command` - The executed command
- `args` - Command arguments
- `exit_code` - Exit code of the command
- `stdout` - Standard output captured
- `stderr` - Standard error captured
- `start_time` - Execution start time (ISO 8601)
- `end_time` - Execution end time (ISO 8601)
- `duration_ms` - Duration in milliseconds

### Example Receipt

```json
{
  "command": "echo",
  "args": ["Hello, world!"],
  "exit_code": 0,
  "stdout": "Hello, world!\n",
  "stderr": "",
  "start_time": "2026-02-16T01:00:00.000000000Z",
  "end_time": "2026-02-16T01:00:00.001000000Z",
  "duration_ms": 1
}
```

## Features

- ✅ Simple, portable, deterministic
- ✅ Exit code propagation (rcpt exits with same code as wrapped command)
- ✅ Complete stdout/stderr capture
- ✅ Precise timing with millisecond resolution
- ✅ ISO 8601 timestamps
- ✅ Pretty-printed JSON output

## License

See LICENSE file.