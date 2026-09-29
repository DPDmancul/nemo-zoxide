# Nemo Zoxide

Use Zoxide to change Nemo location

<img src="https://gitlab.com/-/project/86643873/uploads/a6d45c276e4e65cc408af2a8e3baedca/nemo-zoxide.png" alt="nemo-zoxide" width="825" height="574" />

## Usage

1. Press `Ctrl+J` (for **j**ump)
2. Write your query in the Zoxide interactive dialog
3. Use arrows to select the desired location
4. Press `Enter` to change location, or `Esc` to cancel

## Installation

```bash
cargo build --release
sudo install -Dm755 target/release/libnemo_zoxide.so /lib/nemo/extensions-3.0
```

## Development

```bash
cargo build
NEMO_EXTENSION_DIR=./target/debug nemo
```
