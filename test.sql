-- Create a table called users
create table users (
    id serial primary key,
    name varchar( 32) not null,
    _active boolean default true not null,
    last_initial char(1),
    UNIQUE(name)
);

-- Insert some people into the table
insert into users(name, last_initial) values ('Isaac''s', 'L'), ('Zachary'
, 'L'), ('Jordan', 'Y'), ('\tRemington', 'D');

-- Select users based on criteria
SELECt name, _active from users where name like '%a%' and last_initial = 'L'or age >= 10;

/*
 That is all!
*/

select * from users; -- Finally, all users