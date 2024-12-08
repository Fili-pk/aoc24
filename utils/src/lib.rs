use std::{
	mem::{transmute_copy, ManuallyDrop, MaybeUninit},
	thread::scope,
};

pub trait IffExt {
	fn iff(self, cnd: impl FnOnce(&Self) -> bool) -> Option<Self>
	where
		Self: Sized;
}

impl<T> IffExt for T {
	fn iff(self, cnd: impl FnOnce(&Self) -> bool) -> Option<Self> {
		if cnd(&self) {
			Some(self)
		} else {
			None
		}
	}
}

pub trait GetIExt {
	type Out;
	fn geti(&self, idx: i32) -> Option<&Self::Out>;
	fn geti_mut(&mut self, idx: i32) -> Option<&mut Self::Out>;
}

impl<T> GetIExt for [T] {
	type Out = T;
	fn geti(&self, idx: i32) -> Option<&T> {
		if idx < 0 {
			return None;
		}

		self.get(idx as usize)
	}

	fn geti_mut(&mut self, idx: i32) -> Option<&mut T> {
		if idx < 0 {
			return None;
		}

		self.get_mut(idx as usize)
	}
}

pub trait AndMutExt<T> {
	fn and_mut<F>(self, f: F)
	where
		F: FnOnce(&mut T);
}

impl<T> AndMutExt<T> for Option<&mut T> {
	fn and_mut<F>(self, f: F)
	where
		F: FnOnce(&mut T),
	{
		if let Some(x) = self {
			f(x)
		}
	}
}

pub fn tryy<'a, T: Send + 'a>(q: impl FnOnce() -> T + Send + 'a) -> Option<T> {
	scope(|s| s.spawn(q).join().ok())
}

pub struct CProduct<'a, T, const SIZE: usize, const OUT_SIZE: usize> {
	src: &'a [T; SIZE],
	idx: [usize; OUT_SIZE],
	end: bool,
}

pub trait CProductExt<T, const SIZE: usize> {
	fn c_product<const OUT_SIZE: usize>(&self) -> CProduct<T, SIZE, OUT_SIZE>;
}

impl<T: Clone, const SIZE: usize> CProductExt<T, SIZE> for [T; SIZE] {
	fn c_product<const OUT_SIZE: usize>(&self) -> CProduct<T, SIZE, OUT_SIZE> {
		CProduct {
			src: self,
			idx: [0; OUT_SIZE],
			end: SIZE == 0 || OUT_SIZE == 0,
		}
	}
}

impl<'a, T: Clone, const SIZE: usize, const OUT_SIZE: usize> Iterator
	for CProduct<'a, T, SIZE, OUT_SIZE>
{
	type Item = [T; OUT_SIZE];
	fn next(&mut self) -> Option<[T; OUT_SIZE]> {
		if self.end {
			return None;
		}

		let mut res = [const { MaybeUninit::uninit() }; OUT_SIZE];
		let mut i = 0;
		while i < OUT_SIZE {
			res[i] = MaybeUninit::new(self.src[self.idx[i]].clone());
			if self.idx[i] != SIZE - 1 {
				self.idx[i] += 1;
				for i in 0..i {
					self.idx[i] = 0;
				}
				break;
			}

			i += 1;
		}

		if i == OUT_SIZE {
			self.end = true;
			return Some(unsafe { transmute_copy(&ManuallyDrop::new(res)) });
		}

		for i in (i + 1)..OUT_SIZE {
			res[i] = MaybeUninit::new(self.src[self.idx[i]].clone());
		}

		Some(unsafe { transmute_copy(&ManuallyDrop::new(res)) })
	}
}

pub struct FindOverlapping<'a, T> {
	t: &'a [T],
	pattern: &'a [T],
	idx: usize,
}

impl<'a, T: Eq> Iterator for FindOverlapping<'a, T> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.idx + self.pattern.len().max(1) > self.t.len() {
			return None;
		}

		while !self.t[self.idx..].starts_with(self.pattern) {
			self.idx += 1;
			if self.idx + self.pattern.len().max(1) > self.t.len() {
				return None;
			}
		}

		let v = Some(self.idx);
		self.idx += 1;
		v
	}
}

pub trait FindOverlappingExt<T> {
	fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T>;
}

impl<T: Eq> FindOverlappingExt<T> for [T] {
	fn find_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindOverlapping<'a, T> {
		FindOverlapping {
			t: self,
			pattern,
			idx: 0,
		}
	}
}

pub struct FindNotOverlapping<'a, T> {
	t: &'a [T],
	pattern: &'a [T],
	idx: usize,
}

impl<'a, T: Eq> Iterator for FindNotOverlapping<'a, T> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.idx + self.pattern.len().max(1) > self.t.len() {
			return None;
		}

		while !self.t[self.idx..].starts_with(self.pattern) {
			self.idx += 1;
			if self.idx + self.pattern.len().max(1) > self.t.len() {
				return None;
			}
		}

		let v = Some(self.idx);
		self.idx += self.pattern.len();
		v
	}
}

pub trait FindNotOverlappingExt<T> {
	fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T>;
}

impl<T: Eq> FindNotOverlappingExt<T> for [T] {
	fn find_not_overlapping<'a>(&'a self, pattern: &'a [T]) -> FindNotOverlapping<'a, T> {
		FindNotOverlapping {
			t: self,
			pattern,
			idx: 0,
		}
	}
}
