table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        priority -> Bool,
        state -> Varchar,
        created -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}
