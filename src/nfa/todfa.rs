use std::collections::{HashSet, HashMap};
use crate::{dfa::DFAStatusTransGraph, NodeOffset, TargetNodeOffset};

use super::{NFAStatusTransGraph, Node};



type TableKey = HashSet<TargetNodeOffset>;
type NonUniqueKey = Vec<TargetNodeOffset>;
type TableValue = HashMap<u8, TableKey>;
type Table = HashMap<NonUniqueKey, TableValue>;


fn get_key_from_value(i: &TableValue) -> Vec<NonUniqueKey> {
  i.values().map(|x | x.iter().cloned().collect::<NonUniqueKey>()).collect()
}

impl NFAStatusTransGraph {
  fn to_dfa(&self) -> DFAStatusTransGraph {
    let mut value: Table = Table::new();
    let mut work_list: Vec<NonUniqueKey> = (0..self.0.len())
      .into_iter()
      .map(|x| self.get_nexts(x))
      .flat_map(|x| get_key_from_value(&x))
      .map(|x | x.iter().cloned().collect::<NonUniqueKey>())
      .collect::<Vec<_>>();
    work_list.sort();
    // 很经典的流分析（图遍历）样板代码
    loop {
      let new_value: Table = work_list.iter()
        .map(|k| (k.clone(), self.get_target_nexts(&k))).collect();
      let mut new_work_list: Vec<NonUniqueKey> = new_value.values().flat_map(get_key_from_value).collect();
      new_work_list.sort();
      if new_work_list == work_list {
        break;
      } else {
        work_list = new_work_list;
        value = new_value;
      }
    }
    println!("debug value: {:?}", value);
    todo!()
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
    for (k, v) in self.0
      .iter()
      .filter(|x| x.1 == i)
      .map(|x| (x.0, &x.2)) {
        if ret_table.contains_key(&k) {
          ret_table.get_mut(&k).unwrap().extend(v.into_iter());
        } else {
          ret_table.insert(k, v.clone());
        }
      }
    return ret_table.into_iter().map(|(k, v)| (k, v.into_iter().collect::<TableKey>())).collect();
  }
}
