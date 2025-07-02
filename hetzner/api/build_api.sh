cd ~/holy-bible-api/api
git checkout master
git pull
cargo build --release
cp target/release/api /usr/local/bin/api
sudo systemctl restart api
sudo systemctl status api
journalctl -u api -f