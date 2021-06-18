create table exchange_rate (
    id              serial primary key,
    currency_code   varchar not null,
    rate            numeric not null,

    updated_at      timestamp not null
);