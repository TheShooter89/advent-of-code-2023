use super::schema::Schema;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Engine {
    schema: Schema,
}
