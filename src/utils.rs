use std::hash::{hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8]{
    // This is dangerous - do this better
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}
//
//pub fn obj_to_u8<T: Hash>(p: &T) -> [u8]{
//    let mut hasher = DefaultHasher::new();
//    hahser
//}
