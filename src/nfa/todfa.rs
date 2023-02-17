use crate::{
  dfa::{self, DFAStatusTransGraph, Edge},
  NodeOffset,
  TargetNodeOffset,
};
use std::collections::{hash_map::Entry, HashMap};

use super::NFAStatusTransGraph;



// type TableKey = HashSet<TargetNodeOffset>;
type NonUniqueKey = Vec<TargetNodeOffset>;
type TableValue = HashMap<u8, NonUniqueKey>;
type Table = HashMap<NonUniqueKey, TableValue>;


fn get_key_from_value(i: &TableValue) -> Vec<NonUniqueKey> {
  i.values().map(|x| x.to_vec()).collect()
}

fn get_sorted_set(i: impl Iterator<Item = TargetNodeOffset>) -> NonUniqueKey {
  let mut r = i.collect::<Vec<_>>();
  r.sort();
  r
}

impl NFAStatusTransGraph {
  pub fn to_dfa(&self) -> DFAStatusTransGraph {
    let mut value: Table = Table::new();
    // get start condition(key list)
    let mut work_list: Vec<NonUniqueKey> = (0..self.0.len())
      .map(|offset| vec![TargetNodeOffset::Next(offset)])
      // .map(|x| self.get_nexts(x))
      // .flat_map(|x| get_key_from_value(&x))
      .collect::<Vec<_>>();
    work_list.sort();
    // graph propagate and find fixed point
    loop {
      let new_value: Table = work_list
        .iter()
        .map(|k| (k.clone(), self.get_target_nexts(k)))
        .collect();
      let mut new_work_list: Vec<NonUniqueKey> =
        new_value.values().flat_map(get_key_from_value).collect();
      new_work_list.sort();
      value = new_value;
      if new_work_list == work_list {
        break;
      } else {
        work_list = new_work_list;
      }
    }
    let key_map: HashMap<NonUniqueKey, usize> = value
      .keys()
      .enumerate()
      .map(|(offset, o)| (o.clone(), offset))
      .collect();
    let map: Vec<dfa::Node> = value
      .into_iter()
      .map(|(k, v)| {
        v.into_iter()
          .map(|(mat, next)| Edge(mat, key_map[&k], TargetNodeOffset::Next(key_map[&next]))) // fixme Edge(mat, prev, next!!!!!)
          .collect()
      })
      .collect();
    DFAStatusTransGraph::new(map)
  }

  // map sum product
  fn get_target_nexts(&self, i: &NonUniqueKey) -> TableValue {
    i.iter()
      .flat_map(|x| x.to_node())
      .flat_map(|i| self.get_nexts(i))
      .collect()
  }

  /// table
  /// 1 -> 'a' -> 1, 2
  /// 1 -> 'b' -> 2
  /// 2 -> 'b' -> 4
  /// table.get_nexts(1) => {'a': {1, 2}, 'b': {2}}
  fn get_nexts(&self, i: NodeOffset) -> TableValue {
    let mut ret_table: HashMap<u8, Vec<TargetNodeOffset>> = HashMap::new();
    for (k, v) in self.0.iter().filter(|x| x.1 == i).map(|x| (x.0, &x.2)) {
      if let Entry::Vacant(e) = ret_table.entry(k) {
        e.insert(v.clone());
      } else {
        ret_table.get_mut(&k).unwrap().extend(v.iter());
      }
    }
    ret_table
      .into_iter()
      .map(|(k, v)| (k, get_sorted_set(v.into_iter())))
      .collect()
  }
}
