set -x
cd ./today-rs
cargo build --release
cp ./target/release/today-rs ~/.local/bin/today
