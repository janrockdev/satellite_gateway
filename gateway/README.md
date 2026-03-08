# GATEWAY

## Install Homebrew
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv bash)"' >> /home/vscode/.bashrc
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv bash)"
```

## Install dependencies
```bash
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv bash)"
mongosh "mongodb+srv://cluster0.gsjcz.mongodb.net/" --apiVersion 1 --username <db_username>
```

## Compile and Run

### 1. Copy and fill in credentials
```bash
cp .env.example .env
```

### 2. If building on the Pi directly:
```bash
cargo build --release
./target/release/gateway
```

### 3. Cross-compile from x86 (faster):
```bash
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```