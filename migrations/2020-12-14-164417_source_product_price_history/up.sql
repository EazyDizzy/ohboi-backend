create table source_product_price_history (
    id              serial primary key,
    source_id       int not null,
    product_id      int not null,
    price           numeric not null,
    external_id     varchar not null,

    created_at      timestamp not null,

    foreign key(source_id)
	  references source(id)
	  on delete set null,

    foreign key(product_id)
	  references product(id)
	  on delete set null
);