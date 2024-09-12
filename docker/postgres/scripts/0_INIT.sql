CREATE DATABASE shorekeeper_db;
CREATE USER shorekeeper_user WITH encrypted password 'shorekeeper_pass';
GRANT ALL PRIVILEGES ON DATABASE shorekeeper_db to shorekeeper_user;