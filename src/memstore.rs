use std::collections::{HashMap, HashSet};

use crate::{
    query::Query,
    store::Store,
    transaction::{Object, Transaction},
    types::{Attribute, Datom, Value, AV, DBID, ID, TX},
};

#[derive(Debug, Default)]
struct MemStore {
    datoms: Vec<Datom>,
    next_tx: u64,
    eavt: HashMap<DBID, HashMap<Attribute, (Value, TX)>>,
}
impl Store for MemStore {
    fn new() -> Self {
        MemStore::default()
    }

    fn rebuild_eavt(&mut self, new_datoms: &Vec<Datom>) {
        for datom in new_datoms {
            let attribute = datom.attribute.clone();
            let value = datom.value.clone();
            let attr_map = self.eavt.entry(datom.id).or_insert_with(HashMap::new);
            attr_map.insert(attribute, (value, datom.tx));
        }
    }

    fn transact(&mut self, transaction: Transaction) {
        let mut new_datoms: Vec<Datom> = transaction
            .elems
            .iter()
            .map(|elem| {
                elem.a_v.iter().map(|av| {
                    let av = av.clone();
                    Datom {
                        id: elem.id.id(),
                        attribute: av.attribute,
                        value: av.value,
                        tx: self.next_tx,
                    }
                })
            })
            .flatten()
            .collect();

        self.rebuild_eavt(&new_datoms);
        self.datoms.append(&mut new_datoms);
        self.next_tx += 1;
    }

    fn resolve_objects(&self, datoms: Vec<&Datom>) -> Vec<Object> {
        let idset: HashSet<DBID> = datoms.iter().map(|datom| datom.id).collect();

        idset
            .iter()
            .map(|id| {
                self.eavt.get(id).map(|m| Object {
                    id: ID::DBID(*id),
                    a_v: m
                        .iter()
                        .map(|(a, (v, _))| AV {
                            attribute: a.clone(),
                            value: v.clone(),
                        })
                        .collect(),
                })
            })
            .filter_map(std::convert::identity)
            .collect()
    }

    fn query_datoms(&self, query: &Query) -> Vec<&Datom> {
        self.datoms
            .iter()
            .filter(|d| query.conjunction.iter().all(|clause| clause.match_datom(d)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    use crate::query::*;

    // Helper to create a Datom
    fn create_datom(id: DBID, attribute: &str, value: Value, tx: TX) -> Datom {
        Datom {
            id,
            attribute: Attribute::String(attribute.to_string()),
            value,
            tx,
        }
    }

    // Helper to create a Transaction
    fn create_transaction(id: ID, av_pairs: Vec<AV>) -> Transaction {
        Transaction {
            elems: vec![Object { id, a_v: av_pairs }],
        }
    }

    #[test]
    fn test_new() {
        let store = MemStore::new();
        assert!(store.datoms.is_empty(), "Datoms should be empty");
        assert!(store.eavt.is_empty(), "EAVT should be empty");
        assert_eq!(store.next_tx, 0, "Next TX should be 0");
    }

    #[test]
    fn test_rebuild_eavt_single_datom() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datoms = vec![create_datom(
            id,
            "name",
            Value::String("Alice".to_string()),
            1,
        )];
        store.rebuild_eavt(&datoms);

        let attr_map = store.eavt.get(&id).expect("ID should exist in EAVT");
        let (value, tx) = attr_map
            .get(&Attribute::String("name".to_string()))
            .expect("Attribute should exist");
        assert_eq!(value, &Value::String("Alice".to_string()));
        assert_eq!(*tx, 1);
    }

    #[test]
    fn test_rebuild_eavt_multiple_datoms() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datoms = vec![
            create_datom(id, "name", Value::String("Alice".to_string()), 1),
            create_datom(id, "age", Value::Integer(30), 1),
        ];
        store.rebuild_eavt(&datoms);

        let attr_map = store.eavt.get(&id).expect("ID should exist in EAVT");
        assert_eq!(attr_map.len(), 2, "Should have two attributes");
        assert_eq!(
            attr_map.get(&Attribute::String("name".to_string())),
            Some(&(Value::String("Alice".to_string()), 1))
        );
        assert_eq!(
            attr_map.get(&Attribute::String("age".to_string())),
            Some(&(Value::Integer(30), 1))
        );
    }

    #[test]
    fn test_rebuild_eavt_empty() {
        let mut store = MemStore::new();
        store.rebuild_eavt(&vec![]);
        assert!(store.eavt.is_empty(), "EAVT should remain empty");
    }

    #[test]
    fn test_transact_single_av() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let transaction = create_transaction(
            ID::DBID(id),
            vec![AV {
                attribute: Attribute::String("name".to_string()),
                value: Value::String("Alice".to_string()),
            }],
        );

        store.transact(transaction);

        assert_eq!(store.datoms.len(), 1, "Should have one datom");
        assert_eq!(store.next_tx, 1, "Next TX should increment to 1");
        let datom = &store.datoms[0];
        assert_eq!(datom.id, id);
        assert_eq!(datom.attribute, Attribute::String("name".to_string()));
        assert_eq!(datom.value, Value::String("Alice".to_string()));
        assert_eq!(datom.tx, 0);

        let attr_map = store.eavt.get(&id).expect("ID should exist in EAVT");
        assert_eq!(
            attr_map.get(&Attribute::String("name".to_string())),
            Some(&(Value::String("Alice".to_string()), 0))
        );
    }

    #[test]
    fn test_transact_multiple_avs() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let transaction = create_transaction(
            ID::DBID(id),
            vec![
                AV {
                    attribute: Attribute::String("name".to_string()),
                    value: Value::String("Alice".to_string()),
                },
                AV {
                    attribute: Attribute::String("age".to_string()),
                    value: Value::Integer(30),
                },
            ],
        );

        store.transact(transaction);

        assert_eq!(store.datoms.len(), 2, "Should have two datoms");
        assert_eq!(store.next_tx, 1, "Next TX should increment to 1");
        let attr_map = store.eavt.get(&id).expect("ID should exist in EAVT");
        assert_eq!(attr_map.len(), 2, "EAVT should have two attributes");
    }

    #[test]
    fn test_transact_empty() {
        let mut store = MemStore::new();
        let transaction = Transaction { elems: vec![] };
        store.transact(transaction);

        assert!(store.datoms.is_empty(), "Datoms should remain empty");
        assert!(store.eavt.is_empty(), "EAVT should remain empty");
        assert_eq!(store.next_tx, 1, "Next TX should still increment");
    }

    #[test]
    fn test_resolve_objects_single_object() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        store.rebuild_eavt(&vec![datom.clone()]);
        let objects = store.resolve_objects(vec![&datom]);

        assert_eq!(objects.len(), 1, "Should resolve one object");
        let obj = &objects[0];
        assert_eq!(obj.id, ID::DBID(id));
        assert_eq!(obj.a_v.len(), 1, "Object should have one AV pair");
        assert_eq!(
            obj.a_v[0],
            AV {
                attribute: Attribute::String("name".to_string()),
                value: Value::String("Alice".to_string()),
            }
        );
    }

    #[test]
    fn test_resolve_objects_empty() {
        let store = MemStore::new();
        let objects = store.resolve_objects(vec![]);
        assert!(
            objects.is_empty(),
            "Should return empty vector for empty input"
        );
    }

    #[test]
    fn test_resolve_objects_no_eavt_match() {
        let store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        let objects = store.resolve_objects(vec![&datom]);
        assert!(
            objects.is_empty(),
            "Should return empty when EAVT has no data"
        );
    }

    #[test]
    fn test_query_datoms_single_clause() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        store.rebuild_eavt(&vec![datom.clone()]);
        store.datoms.push(datom);

        let query = Query {
            conjunction: vec![WhereClause {
                attribute: Attribute::String("name".to_string()),
                value: Some(Value::String("Alice".to_string())),
            }],
        };

        let results = store.query_datoms(&query);
        assert_eq!(results.len(), 1, "Should find one matching datom");
        assert_eq!(results[0].value, Value::String("Alice".to_string()));
    }

    #[test]
    fn test_query_datoms_multiple_clauses() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        store.rebuild_eavt(&vec![datom.clone()]);
        store.datoms.push(datom);

        let query = Query {
            conjunction: vec![
                WhereClause {
                    attribute: Attribute::String("name".to_string()),
                    value: Some(Value::String("Alice".to_string())),
                },
                WhereClause {
                    attribute: Attribute::String("age".to_string()),
                    value: Some(Value::Integer(30)),
                },
            ],
        };

        let results = store.query_datoms(&query);
        assert!(
            results.is_empty(),
            "Should find no datoms when not all clauses match"
        );
    }

    #[test]
    fn test_query_datoms_no_match() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        store.rebuild_eavt(&vec![datom.clone()]);
        store.datoms.push(datom);

        let query = Query {
            conjunction: vec![WhereClause {
                attribute: Attribute::String("name".to_string()),
                value: Some(Value::String("Bob".to_string())),
            }],
        };

        let results = store.query_datoms(&query);
        assert!(results.is_empty(), "Should find no matching datoms");
    }

    #[test]
    fn test_query_datoms_empty_query() {
        let mut store = MemStore::new();
        let id = Uuid::new_v4();
        let datom = create_datom(id, "name", Value::String("Alice".to_string()), 1);
        store.datoms.push(datom);

        let query = Query {
            conjunction: vec![],
        };

        let results = store.query_datoms(&query);
        assert_eq!(results.len(), 1, "Empty query should return all datoms");
    }
}
