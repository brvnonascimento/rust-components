-- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
--
-- For our user table, quoting the table name is recommended by IntelliJ's tooling because `user` is a keyword,
-- though Postgres seems to handle it fine in most contexts either way.
create table "user"
(
    -- Having the table name as part of the primary key column makes it nicer to write joins, e.g.:
    --
    -- select * from "user"
    -- inner join article using (user_id)
    --
    -- as opposed to `inner join article on article.user_id = user.id`, and makes it easier to keep track of primary
    -- keys as opposed to having all PK columns named "id"
    user_id       uuid primary key                                default uuid_generate_v1mc(),
    
    name          text                                   not null,

    -- If you want to be really pedantic you can add a trigger that enforces this column will never change,
    -- but that seems like overkill for something that's relatively easy to enforce in code-review.
    created_at    timestamptz                            not null default now(),

    updated_at    timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('"user"');