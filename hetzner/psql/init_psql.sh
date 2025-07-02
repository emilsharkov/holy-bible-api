sudo apt update
sudo apt upgrade -y
sudo apt install postgresql -y
sudo -u postgres psql
# CREATE ROLE postgres LOGIN SUPERUSER;
# CREATE ROLE postgres WITH LOGIN SUPERUSER PASSWORD '';
# psql -U postgres -h localhost -d postgres
# pg_dump -U postgres -h localhost -d postgres -Fc -v -f backup_file.dump
# pg_restore -U postgres -h localhost -d postgres -v backup_file.dump