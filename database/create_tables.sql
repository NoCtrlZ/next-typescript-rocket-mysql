USE db

DROP TABLE IF EXISTS temporary_users;
CREATE TABLE users (
    id INT(12) UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) CHARACTER SET utf8,
    mail VARCHAR(80) CHARACTER SET utf8,
    updated_at TIMESTAMP NOT NULL default current_timestamp on update current_timestamp,
    created_at TIMESTAMP NOT NULL default current_timestamp
);