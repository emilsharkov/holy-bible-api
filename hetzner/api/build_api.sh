cd ~/holy-bible-api/api
cargo build --release
cp target/release/api /usr/local/bin/api
sudo systemctl restart api
sudo systemctl status api
journalctl -u api -f