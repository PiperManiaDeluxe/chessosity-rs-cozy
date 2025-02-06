pub fn is_threefold(current_hash: u64, hash_history: &[u64]) -> bool {
    hash_history.iter().filter(|&&h| h == current_hash).count() >= 3
}