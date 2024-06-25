use std::mem;

pub fn return_param<T, F>(f: F) -> T
where
    F: FnOnce(&mut T),
{
    let mut val = unsafe { mem::zeroed() };
    f(&mut val);
    val
}
