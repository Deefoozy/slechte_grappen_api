CREATE TYPE file_types AS ENUM ('audio', '');
CREATE TYPE score_types AS ENUM ('penalty', 'main');

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS interfaces (
    id BIGSERIAL PRIMARY KEY,
    name varchar(50) NOT NULL,
    key varchar(128) NULL
);

CREATE TABLE IF NOT EXISTS score_boards (
    id BIGSERIAL PRIMARY KEY,
    name varchar(50) NOT NULL,
    point_increment INT NULL,
    penalty_increment INT NULL
);

CREATE TABLE IF NOT EXISTS files (
    id BIGSERIAL PRIMARY KEY,
    interface_id BIGINT NOT NULL,
    type file_types NOT NULL,

    CONSTRAINT files_interfaces_fk FOREIGN KEY (interface_id) REFERENCES interfaces (id)
);

CREATE TABLE IF NOT EXISTS points (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    score_board_id BIGINT NOT NULL,
    val INT NOT NULL,
    type score_types NOT NULL,

    CONSTRAINT points_users_fk FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT points_score_boards_fk FOREIGN KEY (score_board_id) REFERENCES score_boards (id)
);

CREATE TABLE IF NOT EXISTS interface_scoreboards (
    interface_id BIGINT NOT NULL,
    score_board_id BIGINT NOT NULL,

    PRIMARY KEY(interface_id, score_board_id)
);

CREATE TABLE IF NOT EXISTS user_scoreboards (
    user_id BIGINT NOT NULL,
    score_board_id BIGINT NOT NULL,

    PRIMARY KEY(user_id, score_board_id)
);
