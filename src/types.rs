use std::hash::Hash;
use uuid::Uuid;

pub type TMPID = Uuid;
pub type DBID = Uuid;
pub type TX = u64;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Value {
    String(String),
    Integer(i64),
    Boolean(bool),
    EntityRef(ID),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    DBID(DBID),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ID {
    DBID(DBID),
    TMPID(TMPID),
}

impl ID {
    pub fn id(&self) -> Uuid {
        match self {
            ID::DBID(db_id) => *db_id,
            ID::TMPID(tmp_id) => *tmp_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Datom {
    pub id: DBID,
    pub attribute: Attribute,
    pub value: Value,
    pub tx: TX,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AV {
    pub attribute: Attribute,
    pub value: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_resolution() {
        let uuid = Uuid::new_v4();
        assert_eq!(ID::DBID(uuid).id(), uuid);
        assert_eq!(ID::TMPID(uuid).id(), uuid);
    }
}
