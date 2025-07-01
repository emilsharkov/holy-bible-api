sudo apt update
sudo apt upgrade -y
sudo apt install redis-server -y
sudo systemctl start redis-server
sudo systemctl enable redis-server
sudo systemctl status redis-server
# requirepass foobared
sudo nano /etc/redis/redis.conf
sudo systemctl restart redis-server
# redis-cli -h 127.0.0.1 -p 6379 -a your_secure_password