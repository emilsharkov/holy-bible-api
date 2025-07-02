sudo systemctl daemon-reload
sudo systemctl enable api
sudo systemctl start api
sudo systemctl status api
journalctl -u api -f