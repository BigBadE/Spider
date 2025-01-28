use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub(crate) struct Spider {
    pub dependencies: HashMap<u64, HashSet<u64>>
}