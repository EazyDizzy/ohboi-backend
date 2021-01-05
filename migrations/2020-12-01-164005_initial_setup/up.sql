create table category (
    id              serial primary key,
    slug            varchar not null,
    parent_id       int,

    foreign key(parent_id)
    	  references category(id)
    	  on delete set null
);

create table source (
    id              serial primary key,
    site_name       varchar not null,
    logo            varchar not null,
    enabled         bool not null,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null
);
SELECT diesel_manage_updated_at('source');

create table product (
    id              serial primary key,
    title           varchar not null,
    description     text,
    lowest_price    numeric not null,
    images          varchar[],
    category        int not null,
    enabled         bool not null,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null,

     foreign key(category)
        	  references category(id)
        	  on delete cascade
);
CREATE UNIQUE INDEX idx_product_title
ON product(title);
SELECT diesel_manage_updated_at('product');

create table source_product (
    id              serial primary key,
    source_id       int not null,
    product_id      int not null,
    external_id     varchar not null,
    price           numeric not null,
    enabled         bool not null,

    updated_at      timestamp not null,

    foreign key(source_id)
	  references source(id)
	  on delete cascade,

    foreign key(product_id)
	  references product(id)
	  on delete cascade
);
SELECT diesel_manage_updated_at('source_product');
CREATE UNIQUE INDEX idx_source_product
ON source_product(source_id, product_id, external_id);

create table users (
    id              serial primary key,
    username       varchar not null,

    created_at      timestamp not null default now(),
    updated_at      timestamp not null
);
SELECT diesel_manage_updated_at('users');