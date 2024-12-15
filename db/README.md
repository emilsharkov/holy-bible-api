# Database
## Login to Database
### PROD: 
```psql -h terraform-20241206002335676200000001.cru2wuokeaez.us-east-1.rds.amazonaws.com -U postgres -d postgres -p 5432```
### Locally: 
```psql -h localhost -U postgres -d postgres -p 5432```

## Create Tables
Type ```\i path/init_db.sql``` using forward slashes for the absolute path

## To Drop All Tables
Type ```\i path/wipe_db_clean.sql``` using forward slashes for the absolute path