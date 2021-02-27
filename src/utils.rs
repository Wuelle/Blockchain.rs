use std::time::SystemTime;
use sha2::{Digest, Sha256};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::fmt::Debug;

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8]{
    // This is dangerous - do this better
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

pub fn get_unix_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before Unix epoch!"),
    }
}

pub fn sha256_digest<T: Hash + Debug>(t: T) -> Vec<u8>{
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    let hash = hasher.finish().to_ne_bytes();

    // Redigest the hash
    Sha256::digest(&hash).to_vec()
}
