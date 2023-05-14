create table courses
(
    id   uuid
        primary key,
    name varchar not null
        unique
);
