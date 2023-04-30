create table users (
    id serial primary key,
    name varchar(32) not null,
    active boolean default true not null,
    last_initial char(1),
    UNIQUE(name)
);

insert into users(name, last_initial) values ('Isaac', 'L'), ('Zach', 'L'), ('Jordan', 'Y'), ('Remington', 'D');

SELECt name, active from users;