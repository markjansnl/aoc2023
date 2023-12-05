use std::{collections::BTreeMap, ops::Range, cmp::Ordering};

pub type Mappings<T> = BTreeMap<MappingRange<T>, T>;

#[derive(Debug)]
pub enum MappingRange<K: Ord> {
    Key(Range<K>),
    Get(K),
}


impl<K: Ord> PartialEq for MappingRange<K> {
    fn eq(&self, other: &Self) -> bool {
        use MappingRange::*;
        match (self, other) {
            (Key(range), Get(get)) => range.contains(get),
            (Get(get), Key(range)) => range.contains(get),
            _ => false,
        }
    }
}

impl<K: Ord> Eq for MappingRange<K> {}

impl<K: Ord> PartialOrd for MappingRange<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use MappingRange::*;
        match (self, other) {
            (Key(range1), Key(range2)) => range1.start.partial_cmp(&range2.start),
            (Key(range), Get(get)) => {
                if range.contains(get) {
                    Some(Ordering::Equal)
                } else if *get < range.start {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Get(get), Key(range)) => {
                if range.contains(get) {
                    Some(Ordering::Equal)
                } else if range.start > *get {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Get(get1), Get(get2)) => get1.partial_cmp(get2),
        }
    }
}

impl<K: Ord> Ord for MappingRange<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn test_mapping() -> anyhow::Result<()> {
    let mut mappings = BTreeMap::new();
    mappings.insert(MappingRange::Key(10..20), 100);
    mappings.insert(MappingRange::Key(20..30), 200);
    mappings.insert(MappingRange::Key(5..10), 50);

    let vijfentwintig = mappings.get(&MappingRange::Get(15));
    println!("{vijfentwintig:?}");

    Ok(())
}
