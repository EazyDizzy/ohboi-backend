CREATE TYPE characteristic_visualisation_type AS ENUM ('range', 'multi_selector', 'single_selector', 'bool');
CREATE TYPE characteristic_value_type AS ENUM ('float', 'int', 'string', 'enum','bool');

CREATE TABLE characteristic (
    id                  smallint primary key,
    slug                varchar not null UNIQUE,
    enabled             bool not null,
    visualisation_type  characteristic_visualisation_type not null,
    value_type          characteristic_value_type not null
);


CREATE TABLE category_characteristic (
    id                  serial primary key,
    category_id         int not null,
    characteristic_id   smallint not null,

    foreign key(characteristic_id)
                      references characteristic(id)
                      on delete cascade,
    foreign key(category_id)
                  references category(id)
                  on delete cascade,

    UNIQUE (characteristic_id, category_id)
);

CREATE TABLE product_characteristic (
    product_id          int not null,
    characteristic_id   smallint not null,
    value_id            int not null,

    foreign key(product_id)
            	  references product(id)
            	  on delete cascade,

   PRIMARY KEY (product_id, characteristic_id, value_id)
);

CREATE TABLE product_characteristic_string_value (
    id          serial primary key,
    value       varchar not null UNIQUE
);
CREATE TABLE product_characteristic_enum_value (
    id          serial primary key,
    value       varchar not null UNIQUE
);
CREATE TABLE product_characteristic_float_value (
    id          serial primary key,
    value       numeric not null UNIQUE
);