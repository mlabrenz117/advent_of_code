use std::{
    collections::{hash_map::Keys, HashMap},
    hash::Hash,
};

pub trait HashMapExtension<K, V> {
    fn intersection<'a>(&'a self, other: &'a HashMap<K, V>) -> Intersection<'a, K, V>;
}

impl<K, V> HashMapExtension<K, V> for HashMap<K, V> {
    #[inline]
    fn intersection<'a>(&'a self, other: &'a HashMap<K, V>) -> Intersection<'a, K, V> {
        Intersection {
            iter: self.keys(),
            other,
        }
    }
}

pub struct Intersection<'a, K, V> {
    iter: Keys<'a, K, V>,
    other: &'a HashMap<K, V>,
}

impl<'a, K, V> Iterator for Intersection<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let elt = self.iter.next()?;
            if self.other.contains_key(elt) {
                return Some(elt);
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}
