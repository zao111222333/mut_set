use crate::{Item, MutSet};
use core::{hash::BuildHasher, iter::Chain};

/// A lazy iterator producing elements in the difference of `HashSet`s.
///
/// This `struct` is created by the [`difference`] method on [`HashSet`].
/// See its documentation for more.
///
/// [`difference`]: HashSet::difference
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([4, 2, 3, 4]);
///
/// let mut difference = a.difference(&b);
/// ```
pub struct Difference<'a, T, S, I>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
    I: Iterator<Item = &'a T>,
{
    // iterator of the first set
    iter: I,
    // the second set
    other: &'a MutSet<T, S>,
}

/// A lazy iterator producing elements in the symmetric difference of `HashSet`s.
///
/// This `struct` is created by the [`symmetric_difference`] method on
/// [`HashSet`]. See its documentation for more.
///
/// [`symmetric_difference`]: HashSet::symmetric_difference
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([4, 2, 3, 4]);
///
/// let mut intersection = a.symmetric_difference(&b);
/// ```
pub struct SymmetricDifference<'a, T: 'a, S: 'a, I>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
    I: Iterator<Item = &'a T>,
{
    iter: Chain<Difference<'a, T, S, I>, Difference<'a, T, S, I>>,
}

/// A lazy iterator producing elements in the union of `HashSet`s.
///
/// This `struct` is created by the [`union`] method on [`HashSet`].
/// See its documentation for more.
///
/// [`union`]: HashSet::union
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([4, 2, 3, 4]);
///
/// let mut union_iter = a.union(&b);
/// ```
pub struct Union<'a, T: 'a, S: 'a, I>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
    I: Iterator<Item = &'a T>,
{
    iter: Chain<I, Difference<'a, T, S, I>>,
}
