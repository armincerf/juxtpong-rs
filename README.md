# JUXTPONG

## Local Dev Instructions

### Install Rust

1. **Open Terminal** and run:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Update PATH**: Add to `.zshrc` or `.bash_profile`:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

### Install Trunk

1. **Install Trunk**:
   ```bash
   cargo install trunk
   ```

2. **Add WebAssembly Target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Run JUXTPONG

1. **Clone and Enter Directory**:
   ```bash
   git clone https://github.com/armincerf/juxtpong-rs.git && cd juxtpong-rs
   ```

2. **Run with Trunk**:
   ```bash
   trunk serve
   ```

Visit [http://localhost:8080](http://localhost:8080) to play JUXTPONG!