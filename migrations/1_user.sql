create table "user"
(   id              text            not null unique,

    email           text            not null unique,
    password        text            not null,

    created_at      datetime            not null default CURRENT_TIMESTAMP,
    updated_at      datetime            not null default CURRENT_TIMESTAMP
);

create index "user_email"       on "user" ("email");
create index "user_password"    on "user" ("password");
