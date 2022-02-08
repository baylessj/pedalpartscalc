table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    available_parts (id) {
        id -> Int8,
        owner_id -> Int8,
        part_name -> Text,
        quantity -> Int4,
        kind -> Part_kind,
    }
}
