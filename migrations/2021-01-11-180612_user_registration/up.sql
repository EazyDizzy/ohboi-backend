create TYPE user_registration_type AS ENUM ('google', 'facebook', 'apple');

create table user_registration (
    id                  serial primary key,
    user_id             int not null,
    registration_type   user_registration_type not null,
    email               varchar not null,
    full_name           varchar not null,

     foreign key(user_id)
        	  references users(id)
        	  on delete set null
);