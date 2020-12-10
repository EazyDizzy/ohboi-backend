create table category (
    id              serial primary key,
    title           varchar not null,
    parent_id       int not null,

    foreign key(parent_id)
    	  references category(id)
    	  on delete set null
);