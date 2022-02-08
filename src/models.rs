use crate::sql_types::PartKind;

#[derive(Queryable)]
pub struct AvailablePart {
    pub id: i32,
    pub owner_id: i32, // the id of the user who owns this part
    // TODO: make this an enum? can probably include all kinds of parts that would be needed
    pub kind: PartKind,
    pub part_name: String,
    pub quantity: i32,
}

// #[derive(Insertable)]
// #[table_name = "available_parts"]
// pub struct NewAvailablePart<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
