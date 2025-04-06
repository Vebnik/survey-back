create table "user"
(
    id              uuid            not null unique,

    email           text            not null unique,

    created_at      TIMESTAMPTZ     not null,
    updated_at      TIMESTAMPTZ     not null
);

create index "user_email" on "user" ("email");
