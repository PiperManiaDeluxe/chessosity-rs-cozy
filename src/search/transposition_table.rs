use std::collections::HashMap;

pub struct TranspositionTable {
    table: HashMap<u64, TranspositionTableEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspositionTableEntry> {
        self.table.get(&hash)
    }

    pub fn insert(&mut self, hash: u64, entry: TranspositionTableEntry) {
        self.table.insert(hash, entry);
    }
}

#[derive(Clone)]
pub struct TranspositionTableEntry {
    pub depth: u8,
    pub score: i32,
    pub best_move: Option<cozy_chess::Move>,
    pub entry_type: TranspositionTableEntryType,
    pub pv: Vec<cozy_chess::Move>,
}

#[derive(Clone)]
pub enum TranspositionTableEntryType {
    Exact,
    LowerBound,
    UpperBound,
}
