# Pandora

Pandora is high performance minecraft server implementation written in Rust

## Getting Started

Follow the steps below to get started with Pandora.

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/chloefrfr/Pandora.git
cd pandora
```

2. Build the project:

```bash
cargo build --release
```

3. Run the server

```bash
./target/release/pandora
```

### Configuring Log Levels

Pandora uses a flexable logging system to control the verbosity of logs output to the terminal. By default, the log level is set to `Info`, which means that all logs will be displayed.

### Available Log Levels

- `Error`: Only logs error messages.
- `Warn`: Logs warnings and errors.
- `Info`: Logs informational messages, warnings, and errors.
- `Debug`: Logs debug information, informational messages, warnings, and errors.

#### Setting Log Levels

You can set the log level by passing the `--log-level` argument to the server.

```bash
./target/release/pandora --log-level=Debug
```
