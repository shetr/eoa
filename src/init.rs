use crate::opt_traits::*;

pub struct InitValue<T : OptData> {
    pub value: T
}

impl<T: OptData> InitFunc<T> for InitValue<T> {
    fn init(&self) -> T {
        self.value.clone()
    }
}