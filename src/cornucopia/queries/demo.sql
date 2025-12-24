--! demo_insert
insert into demos (col1)
values ('text1');

--! demo_insert_1
insert into demos (col1)
values (:col1);
