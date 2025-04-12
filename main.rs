use std::collections::{HashMap, HashSet};

fn main() {
    let mut store = MemStore::new();

    let id = Uuid::new_v4();

    store.transact(Transaction {
        elems: vec![
            Object {
                id: ID::TMPID(id),
                a_v: vec![
                    AV {
                        attribute: Attribute::String("name".to_owned()),
                        value: Value::String("Alice".to_string()),
                    },
                    AV {
                        attribute: Attribute::String("age".to_owned()),
                        value: Value::Integer(8),
                    },
                ],
            },
            Object {
                id: ID::TMPID(Uuid::new_v4()),
                a_v: vec![AV {
                    attribute: Attribute::String("age".to_owned()),
                    value: Value::Integer(25),
                }],
            },
        ],
    });

    store.transact(Transaction {
        elems: vec![Object {
            id: ID::TMPID(Uuid::new_v4()),
            a_v: vec![
                AV {
                    attribute: Attribute::String("name".to_owned()),
                    value: Value::String("Bob".to_string()),
                },
                AV {
                    attribute: Attribute::String("age".to_owned()),
                    value: Value::Integer(30),
                },
                AV {
                    attribute: Attribute::String("is_active".to_owned()),
                    value: Value::Boolean(true),
                },
            ],
        }],
    });

    // Print the resulting datoms
    println!("{:?}", store);

    let results1 = store.query_datoms(&Query {
        conjonction: vec![WhereClause {
            attribute: Attribute::String("name".to_owned()),
            value: Some(Value::String("Alice".to_string())),
        }],
    });

    println!("{:?}", store.resolve_objects(results1));
}
