create table exchange_rate (
    id              serial primary key,
    currency        currency_enum not null UNIQUE,
    rate            numeric not null,

    updated_at      timestamp not null DEFAULT now()
);

SELECT diesel_manage_updated_at('exchange_rate');