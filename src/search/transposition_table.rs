use std::collections::HashMap;

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<Option<TranspositionTableEntry>>,
    table_size: usize,
}

impl TranspositionTable {
    pub fn new(table_size: usize) -> Self {
        TranspositionTable {
            table: vec![None; table_size],
            table_size,
        }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspositionTableEntry> {
        let index = (hash as usize) & (self.table_size - 1);
        self.table[index].as_ref().and_then(|entry| {
            if entry.hash == hash {
                Some(entry)
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, hash: u64, entry: TranspositionTableEntry) {
        let index = (hash as usize) & (self.table_size - 1);
        // Replace if:
        // - The slot is empty, or
        // - The stored entry is from a shallower search, or
        // - You want to use another replacement strategy.
        if self.table[index].is_none() || self.table[index].as_ref().unwrap().depth <= entry.depth {
            self.table[index] = Some(entry);
        }
    }

    pub fn len(&mut self) -> usize {
        self.table.len()
    }

    pub fn clear(&mut self){
        self.table.iter_mut().for_each(|e| *e = None);
    }
}

#[derive(Clone)]
pub struct TranspositionTableEntry {
    pub depth: u8,
    pub score: i32,
    pub best_move: Option<cozy_chess::Move>,
    pub entry_type: TranspositionTableEntryType,
    pub pv: Vec<cozy_chess::Move>,
    pub hash: u64,
}

#[derive(Clone)]
pub enum TranspositionTableEntryType {
    Exact,
    LowerBound,
    UpperBound,
}
