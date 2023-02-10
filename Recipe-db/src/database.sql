
drop table if exists yummyrecipe_c5;


create table yummyrecipe_c5
(
    recipe_id serial primary key,
    user_id INT not null,
    recipe_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* Load seed data for testing */
insert into yummyrecipe_c5
    (recipe_id,user_id, recipe_name,posted_time)
values(1, 1, 'Tomyam seafood pasta', '2022-2-16 12:20:00');
insert into yummyrecipe_c5
    (recipe_id, user_id, recipe_name,posted_time)
values(2, 1, 'Chilli crab pasta', '2022-2-16 13:22:00');

grant all privileges on table yummyrecipe_c5 to annt;