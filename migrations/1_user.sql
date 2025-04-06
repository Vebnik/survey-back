create table "user"
(   id              text            not null unique,

    email           text            not null unique,

    created_at      text            not null,
    updated_at      text            not null
);

create index "user_email" on "user" ("email");
