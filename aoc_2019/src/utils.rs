use std::{
    borrow::BorrowMut,
    collections::HashMap,
    hash::{BuildHasher, Hash},
};

pub trait HashMapExtension<'a, K: 'a, V, S, T, I>
where
    I: Iterator<Item = &'a K>,
    T: BorrowMut<I>,
{
    fn intersection(self, other: &'a HashMap<K, V, S>) -> Intersection<'a, K, V, S, T, I>;
}

impl<'a, K: 'a, V, S, I, T> HashMapExtension<'a, K, V, S, T, I> for T
where
    T: BorrowMut<I>,
    I: Iterator<Item = &'a K>,
{
    #[inline]
    fn intersection(self, other: &'a HashMap<K, V, S>) -> Intersection<'a, K, V, S, T, I> {
        Intersection {
            iter: self,
            other,
            pd: std::marker::PhantomData,
        }
    }
}

pub struct Intersection<'a, K: 'a, V, S, T, I>
where
    I: Iterator<Item = &'a K>,
    T: BorrowMut<I>,
{
    iter: T,
    other: &'a HashMap<K, V, S>,
    pd: std::marker::PhantomData<I>,
}

impl<'a, K: 'a, V, S, T, I> Iterator for Intersection<'a, K, V, S, T, I>
where
    S: BuildHasher,
    K: Eq + Hash,
    T: BorrowMut<I>,
    I: Iterator<Item = &'a K>,
{
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let elt = self.iter.borrow_mut().next()?;
            if self.other.contains_key(elt) {
                return Some(elt);
            }
        }
    }
}
