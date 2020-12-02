create table source (
    id              serial primary key,
    site_name       varchar not null,
    id_regex        varchar not null,
    name_selector   varchar not null,
    price_selector  varchar not null,
    logo            varchar not null,
    enabled         bool not null,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null
);

create table product (
    id              serial primary key,
    title           varchar not null,
    description     text not null,
    lowest_price    numeric not null,
    images          varchar[] not null ,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null
);

create table source_product (
    id              serial primary key,
    source_id      int not null,
    product_id      int not null,
    price           numeric not null,

    updated_at      timestamp not null,

    foreign key(source_id)
	  references source(id)
	  on delete set null,

    foreign key(product_id)
	  references product(id)
	  on delete set null
);

create table users (
    id              serial primary key,
    username       varchar not null,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null
);