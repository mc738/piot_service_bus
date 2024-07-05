use rusqlite::Connection;

fn topic_groups_table_sql() -> &'static str {
    "CREATE TABLE IF NOT EXISTS topic_groups
(
    id         TEXT not null
        constraint topic_groups_pk
            primary key,
    name       TEXT not null,
    created_on TEXT not null
)
    "
}

fn topics_table_sql() -> &'static str {
    "CREATE TABLE IF NOT EXISTS topics
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

fn topic_group_links_table_sql() -> &'static str {
    "CREATE TABLE IF NOT EXISTS topic_groups
(
    id         TEXT not null
        constraint topic_groups_pk
            primary key,
    name       TEXT not null,
    created_on TEXT not null
)"
}

fn messages_table_sql() -> &'static str {
    "CREATE TABLE IF NOT EXISTS messages
(
    id                       TEXT    not null
        constraint messages_pk
            primary key,
    topic_id                 TEXT    not null
        constraint messages_topics_id_fk
            references topics,
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

fn message_events_table_sql() -> &'static str {
    "CREATE TABLE IF NOT EXISTS message_events
(
    serial          INTEGER not null
        constraint message_events_pk
            primary key autoincrement,
    message_id      TEXT    not null
        constraint message_events_messages_id_fk
            references messages,
    event_type      TEXT    not null,
    event_timestamp TEXT    not null,
    event_blob      BLOB    not null
)"
}

pub fn initialize(conn: &mut Connection) -> rusqlite::Result<()> {
    let t = conn.transaction()?;
    t.execute(topic_groups_table_sql(), ())?;
    t.execute(topics_table_sql(), ())?;
    t.execute(topic_group_links_table_sql(), ())?;
    t.execute(messages_table_sql(), ())?;
    t.execute(message_events_table_sql(), ())?;
    t.commit()
}