use rusqlite::Connection;

pub(crate) fn topics_table_sql() -> &'static str {
    "create table main.topics if not exists
(
    id         TEXT    not null
        constraint topics_pk
            primary key,
    name       TEXT    not null
        constraint topics_pk_2
            unique,
    created_on TEXT    not null,
    serial     integer not null
        constraint topics_pk_3
            unique
);"
}

pub(crate) fn messages_table_sql() -> &'static str {
    "create table main.messages if not exists
(
    id                       TEXT    not null
        constraint messages_pk
            primary key,
    topic_id                 TEXT    not null
        constraint messages_topics_id_fk
            references main.topics,
    queued_on                TEXT    not null,
    updated_on               TEXT    not null,
    data_blob                BLOB    not null,
    hash                     TEXT    not null,
    handling_on              TEXT,
    handling_acknowledged_on TEXT,
    dead_lettered_on         TEXT,
    serial                   integer not null
);"
}

pub fn initialize(mut conn: Connection) -> rusqlite::Result<()> {
    let t  = conn.transaction()?;
    conn.execute(topics_table_sql(), ())?;
    conn.execute(messages_table_sql(), ())?;
    t.commit()
}