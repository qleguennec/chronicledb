use super::types::{AV, ID};

#[derive(Debug)]
pub struct Object {
    pub id: ID,
    pub a_v: Vec<AV>,
}

pub struct Transaction {
    pub elems: Vec<Object>,
}

#[cfg(test)]
mod tests {
    use super::super::types::{Attribute, Value};
    use super::*;

    #[test]
    fn test_object_creation() {
        let obj = Object {
            id: ID::TMPID(uuid::Uuid::new_v4()),
            a_v: vec![AV {
                attribute: Attribute::String("name".to_owned()),
                value: Value::String("Alice".to_string()),
            }],
        };
        assert_eq!(obj.a_v.len(), 1);
    }
}
