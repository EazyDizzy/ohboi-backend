create table category (
    id              serial primary key,
    slug            varchar not null,
    parent_id       int,

    foreign key(parent_id)
    	  references category(id)
    	  on delete set null
);