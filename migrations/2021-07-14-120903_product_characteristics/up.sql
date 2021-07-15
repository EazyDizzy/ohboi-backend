CREATE TYPE characteristic_visualisation_type AS ENUM ('range', 'multi-selector', 'single-selector', 'bool');
CREATE TYPE characteristic_value_type AS ENUM ('float', 'int', 'string', 'bool');

CREATE TABLE characteristic (
    id                  serial primary key,
    slug                varchar not null,
    enabled             bool not null,
    visualisation_type  characteristic_visualisation_type not null,
    value_type          characteristic_value_type not null
);


CREATE TABLE category_characteristic (
    id                  serial primary key,
    category_id         int not null,
    characteristic_id   int not null,

    foreign key(characteristic_id)
                      references characteristic(id)
                      on delete cascade,
    foreign key(category_id)
                  references category(id)
                  on delete cascade
);

CREATE TABLE product_characteristic (
    id                  serial primary key,
    product_id          int not null,
    characteristic_id   int not null,
    value_id            int not null,

    foreign key(product_id)
            	  references product(id)
            	  on delete cascade,
    foreign key(characteristic_id)
            	  references characteristic(id)
            	  on delete cascade
);

CREATE TABLE product_characteristic_string_value (
    id          serial primary key,
    value       varchar not null
);
CREATE TABLE product_characteristic_float_value (
    id          serial primary key,
    value       float not null
);
CREATE TABLE product_characteristic_int_value (
    id          serial primary key,
    value       int not null
);
CREATE TABLE product_characteristic_enum_value (
    id          serial primary key,
    value       smallint not null
);