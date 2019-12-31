USE db;

CREATE USER 'shin'@'localhost' IDENTIFIED WITH mysql_native_password BY '0523';

GRANT ALL PRIVILEGES ON *.* TO 'shin'@'localhost';
