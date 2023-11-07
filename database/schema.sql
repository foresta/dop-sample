CREATE TABLE books (
    id bigint unsigned NOT NULL AUTO_INCREMENT,
    api_id varchar(255) NOT NULL,
    status ENUM('WISH', 'BOUGHT', 'READ') NOT NULL,
    created_at datetime NOT NULL,
    updated_at datetime NOT NULL,
    PRIMARY KEY(`id`),
    INDEX idx_status(`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
