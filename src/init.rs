use crate::opt_traits::*;

pub struct InitValue<T : OptValue> {
    pub value: T
}

impl<T: OptValue> InitFunc<T> for InitValue<T> {
    fn init(&self) -> T {
        self.value.clone()
    }
}