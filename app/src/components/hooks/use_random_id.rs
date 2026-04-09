use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn generate_hash() -> u64 {
    let mut hasher = DefaultHasher::new();
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    counter.hash(&mut hasher);
    hasher.finish()
}

pub fn use_random_id_for(element: &str) -> String {
    format!("{}_rustui_{}", element, generate_hash())
}
