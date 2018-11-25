table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
    }
}

table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        priority -> Bool,
        state -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        project_id -> Nullable<Int4>,
    }
}

joinable!(todos -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    projects,
    todos,
);
