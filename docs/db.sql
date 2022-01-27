
create table user
(
    id int auto_increment primary key,
    username varchar(20) null,
    password varchar(50) null,
    constraint user_username_uindex unique (username)
);
