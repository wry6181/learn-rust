create table task (
    id varchar not null,
    time int not null,
    data varchar not null
);

create unique index id_inx on task (id);
