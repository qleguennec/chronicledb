use crate::transaction::{Object, Transaction};
use crate::{query::Query, types::*};

pub trait Store {
    fn new() -> Self;
    fn transact(&mut self, transaction: Transaction);
    fn query_datoms(&self, query: &Query) -> Vec<&Datom>;
    fn rebuild_eavt(&mut self, new_datoms: &Vec<Datom>);
    fn rebuild_avet(&mut self, new_datoms: &Vec<Datom>);
    fn resolve_objects(&self, datoms: Vec<&Datom>) -> Vec<Object>;
}
