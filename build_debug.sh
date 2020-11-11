export PATH="$HOME/tools/osxcross/target/bin:$PATH"
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=x86_64-pc-windows-gnu
cargo build --target=x86_64-apple-darwin
