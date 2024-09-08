
use std::ptr::NonNull;

pub struct ScriptedIter<'a, T, D, K: Clone, S: Fn(&'a D, K) -> Option<T>> {
	index: usize,
	steps: &'a [K],
	items: &'a D,
	script: S
}

impl<'a, T, D, K: Clone, S: Fn(&'a D, K) -> Option<T>> Iterator for ScriptedIter<'a, T, D, K, S> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.steps.len() {
			let item = self.script.call((self.items, self.steps[self.index].clone()))?;
			self.index += 1;
			Some(item)
		} else {
			None
		}
	}
}

impl<'a, T, D, K: Clone, S: Fn(&'a D, K) -> Option<T>> ScriptedIter<'a, T, D, K, S> {
	pub fn new(items: &'a D, steps: &'a [K], script: S) -> Self {
		Self {
			index: 0,
			steps,
			items,
			script,
		}
	}
}


pub struct ScriptedMutIter<'a, T, D: 'a, K: Clone, S: Fn(&'a mut D, K) -> Option<T>> {
	index: usize,
	steps: &'a [K],
	items: NonNull<D>,
	script: S,
}

impl<'a, T, D: 'a, K: Clone, S: Fn(&'a mut D, K) -> Option<T>> Iterator for ScriptedMutIter<'a, T, D, K, S> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.steps.len() {
			unsafe {
				let item = self.script.call((self.items.as_mut(), self.steps[self.index].clone()));
				self.index += 1;
				item
			}
		} else {
			None
		}
	}
}

impl<'a, T, D: 'a, K: Clone, S: Fn(&'a mut D, K) -> Option<T>> ScriptedMutIter<'a, T, D, K, S> {
	pub fn new(items: &'a mut D, steps: &'a Vec<K>, script: S) -> Self {
		Self {
			index: 0,
			steps,
			items: NonNull::from(items),
			script,
		}
	}
}
