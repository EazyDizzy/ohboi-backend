create table exchange_rate (
    id              serial primary key,
    currency        currency_enum not null,
    rate            numeric not null,

    updated_at      timestamp not null
);