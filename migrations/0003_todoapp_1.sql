create table tasks (
    id varchar not null,
    time int not null,
    name varchar not null,
    data varchar not null
);

create unique index id_indx on tasks (id);
