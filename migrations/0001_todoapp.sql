create table todoapp (
    id varchar not null,
    time numeric not null,
    data varchar not null
);

create unique index id_inx on todoapp (id);
