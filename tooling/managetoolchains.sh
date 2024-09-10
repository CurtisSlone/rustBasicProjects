# Install the nightly toolchain
rustup install nightly

# Switch between toolchains
rustup default stable
rustup default nightly

# To use nightly only for a specific project
rustup override set nightly

# Compile with different toolchains
cargo build
