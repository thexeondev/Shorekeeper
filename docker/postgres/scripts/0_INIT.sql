CREATE DATABASE camellya_db;
CREATE USER camellya_user WITH encrypted password 'camellya_pass';
GRANT ALL PRIVILEGES ON DATABASE camellya_db to camellya_user;