-- Your SQL goes here
CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4() NOT NULL primary key,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL
);