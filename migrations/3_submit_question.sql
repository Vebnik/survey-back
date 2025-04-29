create table "submit_question"
(   id                  text            not null unique,

    submit_answer_id    text            not null,

    question            text            not null,
    after_six_months    text            not null,
    before_surgery      text            not null
);

create index "submit_question_submit_answer_id"     on "submit_question" ("submit_answer_id");
create index "submit_question_question"             on "submit_question" ("question");
create index "submit_question_after_six_months"     on "submit_question" ("after_six_months");
create index "submit_question_before_surgery"       on "submit_question" ("before_surgery");
