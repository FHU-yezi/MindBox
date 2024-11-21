-- date: 2024-11-21
-- description: 初始化

CREATE TABLE minds (
    id SERIAL CONSTRAINT pk_minds_id PRIMARY KEY,
    publish_time TIMESTAMP NOT NULL,
    content TEXT NOT NULL
);