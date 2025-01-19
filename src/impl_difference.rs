use indexmap::map::Values;

use crate::{Item, MutSet};
use core::{fmt, hash::BuildHasher, iter::Chain};
use std::iter::FusedIterator;

/// A lazy iterator producing elements in the intersection of `HashSet`s.
///
/// This `struct` is created by the [`intersection`] method on [`HashSet`].
/// See its documentation for more.
///
/// [`intersection`]: HashSet::intersection
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([4, 2, 3, 4]);
///
/// let mut intersection = a.intersection(&b);
/// ```
#[must_use = "this returns the intersection as an iterator, \
              without modifying either input set"]
pub struct Intersection<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    // iterator of the first set
    iter: Values<'a, u64, T>,
    // the second set
    other: &'a MutSet<T, S>,
}
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
pub struct Difference<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    // iterator of the first set
    iter: Values<'a, u64, T>,
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
pub struct SymmetricDifference<'a, T: 'a, S: 'a>
where
    T: Item,
    S: BuildHasher,
{
    iter: Chain<Difference<'a, T, S>, Difference<'a, T, S>>,
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
pub struct Union<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    iter: Chain<Values<'a, u64, T>, Difference<'a, T, S>>,
}

impl<'a, T, S> Iterator for Difference<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        loop {
            let elt = self.iter.next()?;
            if !self.other.inner.contains_key(&self.other.id(elt)) {
                return Some(elt);
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, elt| {
            if self.other.inner.contains_key(&self.other.id(elt)) {
                acc
            } else {
                f(acc, elt)
            }
        })
    }
}
impl<'a, T, S> Clone for Intersection<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    #[inline]
    fn clone(&self) -> Self {
        Intersection { iter: self.iter.clone(), ..*self }
    }
}
impl<'a, T, S> Clone for Difference<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    #[inline]
    fn clone(&self) -> Self {
        Difference { iter: self.iter.clone(), ..*self }
    }
}
impl<'a, T, S> Clone for SymmetricDifference<'a, T, S>
where
    T: Item,
    S: BuildHasher,
{
    #[inline]
    fn clone(&self) -> Self {
        SymmetricDifference { iter: self.iter.clone() }
    }
}
impl<'a, T, S> Clone for Union<'a, T, S>
where
    T: 'a + Item,
    S: 'a + BuildHasher,
{
    #[inline]
    fn clone(&self) -> Self {
        Union { iter: self.iter.clone() }
    }
}
impl<'a, T, S> FusedIterator for Intersection<'a, T, S>
where
    T: Item,
    S: BuildHasher,
{
}
impl<'a, T, S> Iterator for Intersection<'a, T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        loop {
            let elt = self.iter.next()?;
            if self.other.inner.contains_key(&self.other.id(elt)) {
                return Some(elt);
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, elt| {
            if self.other.inner.contains_key(&self.other.id(elt)) {
                f(acc, elt)
            } else {
                acc
            }
        })
    }
}
impl<'a, T, S> Iterator for SymmetricDifference<'a, T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<'a, T, S> Iterator for Union<'a, T, S>
where
    T: Item,
    S: BuildHasher,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<'a, T, S> fmt::Debug for Intersection<'a, T, S>
where
    T: 'a + Item + fmt::Debug,
    S: 'a + BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T, S> MutSet<T, S>
where
    T: Item,
    S: BuildHasher,
{
    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([4, 2, 3, 4]);
    ///
    /// // Can be seen as `a - b`.
    /// for x in a.difference(&b) {
    ///     println!("{x}"); // Print 1
    /// }
    ///
    /// let diff: HashSet<_> = a.difference(&b).collect();
    /// assert_eq!(diff, [1].iter().collect());
    ///
    /// // Note that difference is not symmetric,
    /// // and `b - a` means something else:
    /// let diff: HashSet<_> = b.difference(&a).collect();
    /// assert_eq!(diff, [4].iter().collect());
    /// ```
    #[inline]
    pub fn difference<'a>(&'a self, other: &'a MutSet<T, S>) -> Difference<'a, T, S> {
        Difference { iter: self.iter(), other }
    }
    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([4, 2, 3, 4]);
    ///
    /// // Print 1, 4 in arbitrary order.
    /// for x in a.symmetric_difference(&b) {
    ///     println!("{x}");
    /// }
    ///
    /// let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
    /// let diff2: HashSet<_> = b.symmetric_difference(&a).collect();
    ///
    /// assert_eq!(diff1, diff2);
    /// assert_eq!(diff1, [1, 4].iter().collect());
    /// ```
    #[inline]
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a MutSet<T, S>,
    ) -> SymmetricDifference<'a, T, S> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }
    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`.
    ///
    /// When an equal element is present in `self` and `other`
    /// then the resulting `Intersection` may yield references to
    /// one or the other. This can be relevant if `T` contains fields which
    /// are not compared by its `Eq` implementation, and may hold different
    /// value between the two equal copies of `T` in the two sets.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([4, 2, 3, 4]);
    ///
    /// // Print 2, 3 in arbitrary order.
    /// for x in a.intersection(&b) {
    ///     println!("{x}");
    /// }
    ///
    /// let intersection: HashSet<_> = a.intersection(&b).collect();
    /// assert_eq!(intersection, [2, 3].iter().collect());
    /// ```
    #[inline]
    pub fn intersection<'a>(&'a self, other: &'a MutSet<T, S>) -> Intersection<'a, T, S> {
        if self.len() <= other.len() {
            Intersection { iter: self.iter(), other }
        } else {
            Intersection { iter: other.iter(), other: self }
        }
    }
    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([4, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order.
    /// for x in a.union(&b) {
    ///     println!("{x}");
    /// }
    ///
    /// let union: HashSet<_> = a.union(&b).collect();
    /// assert_eq!(union, [1, 2, 3, 4].iter().collect());
    /// ```
    #[inline]
    pub fn union<'a>(&'a self, other: &'a MutSet<T, S>) -> Union<'a, T, S> {
        if self.len() >= other.len() {
            Union { iter: self.iter().chain(other.difference(self)) }
        } else {
            Union { iter: other.iter().chain(self.difference(other)) }
        }
    }
}
