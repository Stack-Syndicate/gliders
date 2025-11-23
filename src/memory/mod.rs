use std::{fmt::Debug, ops::{Deref, DerefMut}};
use bumpalo::Bump;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub static GLOBAL_ARENA: Lazy<Mutex<Bump>> = Lazy::new(|| Mutex::new(Bump::new()));

pub struct P<T> {
	ptr: *mut T
}
impl<T> P<T> {
	fn new(data: T) -> Self {
		let arena = GLOBAL_ARENA.lock();
		let ptr = arena.alloc(data);
		Self { ptr }
	}
	fn get(&self) -> &T {
		unsafe { &*self.ptr }
	}
	fn get_mut(&self) -> &mut T {
		unsafe { &mut *self.ptr }
	}
}
impl<T> Deref for P<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}
impl<T> DerefMut for P<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.get_mut()
	}
}
impl<T: Debug> Debug for P<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.get().fmt(f)
	}
}
impl<T> Clone for P<T> {
	fn clone(&self) -> Self {
		P { ptr: self.ptr }
	}
}

#[cfg(test)]
mod memory_test {
    use crate::memory::P;

	#[test]
	fn p_test() {
		let mut x = P::new(10);
		*x += 5;
		assert!(*x == 15)
	}
}