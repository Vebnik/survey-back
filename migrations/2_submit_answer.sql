create table "submit_answer"
(   id              text            not null unique,

    date            text            not null,
    questions       text            not null,
    satisfied       text            not null,

    created_at      datetime        not null default CURRENT_TIMESTAMP,
    updated_at      datetime        not null default CURRENT_TIMESTAMP
);

create index "submit_answer_date"       on "submit_answer" ("date");
create index "submit_answer_satisfied"    on "submit_answer" ("satisfied");
