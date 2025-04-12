use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    pub attribute: Attribute,
    pub value: Option<Value>,
}

impl WhereClause {
    pub fn match_datom(&self, datom: &Datom) -> bool {
        if self.attribute == datom.attribute {
            if let Some(ref val) = self.value {
                *val == datom.value
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Query {
    pub conjunction: Vec<WhereClause>,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    // Helper function to create a sample Datom
    fn create_datom(attribute: Attribute, value: Value) -> Datom {
        Datom {
            id: Uuid::new_v4(),
            attribute,
            value,
            tx: 1,
        }
    }

    #[test]
    fn test_where_clause_match_datom_same_attribute_and_value() {
        let attribute = Attribute::String("name".to_string());
        let value = Value::String("Alice".to_string());
        let clause = WhereClause {
            attribute: attribute.clone(),
            value: Some(value.clone()),
        };
        let datom = create_datom(attribute, value);

        assert!(
            clause.match_datom(&datom),
            "Should match when attribute and value are the same"
        );
    }

    #[test]
    fn test_where_clause_match_datom_same_attribute_different_value() {
        let attribute = Attribute::String("name".to_string());
        let clause = WhereClause {
            attribute: attribute.clone(),
            value: Some(Value::String("Alice".to_string())),
        };
        let datom = create_datom(attribute, Value::String("Bob".to_string()));

        assert!(
            !clause.match_datom(&datom),
            "Should not match when values differ"
        );
    }

    #[test]
    fn test_where_clause_match_datom_different_attribute() {
        let clause = WhereClause {
            attribute: Attribute::String("name".to_string()),
            value: Some(Value::String("Alice".to_string())),
        };
        let datom = create_datom(
            Attribute::String("age".to_string()),
            Value::String("Alice".to_string()),
        );

        assert!(
            !clause.match_datom(&datom),
            "Should not match when attributes differ"
        );
    }

    #[test]
    fn test_where_clause_match_datom_no_value() {
        let attribute = Attribute::String("name".to_string());
        let clause = WhereClause {
            attribute: attribute.clone(),
            value: None,
        };
        let datom = create_datom(attribute, Value::String("Alice".to_string()));

        assert!(
            !clause.match_datom(&datom),
            "Should not match when clause has no value"
        );
    }

    #[test]
    fn test_query_empty_conjonction() {
        let query = Query {
            conjunction: Vec::new(),
        };

        assert!(
            query.conjunction.is_empty(),
            "Query should have an empty conjonction vector"
        );
    }

    #[test]
    fn test_query_with_multiple_clauses() {
        let clause1 = WhereClause {
            attribute: Attribute::String("name".to_string()),
            value: Some(Value::String("Alice".to_string())),
        };
        let clause2 = WhereClause {
            attribute: Attribute::String("age".to_string()),
            value: Some(Value::Integer(30)),
        };
        let query = Query {
            conjunction: vec![clause1.clone(), clause2.clone()],
        };

        assert_eq!(query.conjunction.len(), 2, "Query should have two clauses");
        assert_eq!(query.conjunction[0], clause1, "First clause should match");
        assert_eq!(query.conjunction[1], clause2, "Second clause should match");
    }
}
