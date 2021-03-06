USE db

DROP TABLE IF EXISTS temporary_users;
CREATE TABLE temporary_users (
    id INT(12) UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    mail VARCHAR(50) CHARACTER SET utf8,
    password VARCHAR(64) CHARACTER SET utf8,
    salt VARCHAR(16) CHARACTER SET utf8,
    cypher VARCHAR(64) CHARACTER SET utf8,
    updated_at TIMESTAMP NOT NULL default current_timestamp on update current_timestamp,
    created_at TIMESTAMP NOT NULL default current_timestamp
);

DROP TABLE IF EXISTS verified_users;
CREATE TABLE verified_users (
    id INT(12) UNSIGNED NOT NULL PRIMARY KEY AUTO_INCREMENT,
    mail VARCHAR(50) CHARACTER SET utf8,
    password VARCHAR(80) CHARACTER SET utf8,
    updated_at TIMESTAMP NOT NULL default current_timestamp on update current_timestamp,
    created_at TIMESTAMP NOT NULL default current_timestamp
);

