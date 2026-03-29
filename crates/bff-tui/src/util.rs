use std::iter::{Fuse, FusedIterator};

pub struct IntersperseEvery<I: Iterator>
where
    I::Item: Clone,
{
    started: bool,
    n: usize,
    count: usize,
    separator: I::Item,
    iter: Fuse<I>,
}

impl<I: Iterator> IntersperseEvery<I>
where
    I::Item: Clone,
{
    pub fn new(iter: I, separator: I::Item, n: usize) -> Self {
        Self {
            started: false,
            count: 0,
            n,
            separator,
            iter: iter.fuse(),
        }
    }
}

impl<I> FusedIterator for IntersperseEvery<I>
where
    I: FusedIterator,
    I::Item: Clone,
{
}

impl<I> Iterator for IntersperseEvery<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            if self.count == self.n {
                self.count = 0;
                Some(self.separator.clone())
            } else {
                self.count += 1;
                self.iter.next()
            }
        } else {
            self.started = true;
            self.count += 1;
            self.iter.next()
        }
    }
}

pub trait IntersperseEveryExt: Sized
where
    Self: Iterator,
    Self::Item: Clone,
{
    fn intersperse_every(self, delim: Self::Item, n: usize) -> IntersperseEvery<Self>;
}

impl<Iter> IntersperseEveryExt for Iter
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    fn intersperse_every(self, delim: Self::Item, n: usize) -> IntersperseEvery<Self> {
        IntersperseEvery::new(self, delim, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_larger_than_seq_size() {
        let res = [1, 1, 1]
            .iter()
            .intersperse_every(&2, 5)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(res, vec![1, 1, 1]);
    }

    #[test]
    fn n_smaller_than_seq_size() {
        let res = [1, 1, 1, 1, 1]
            .iter()
            .intersperse_every(&2, 3)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(res, vec![1, 1, 1, 2, 1, 1]);
    }

    #[test]
    fn n_smaller_than_seq_size_multi() {
        let res = [1, 1, 1, 1, 1, 1, 1]
            .iter()
            .intersperse_every(&2, 3)
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(res, vec![1, 1, 1, 2, 1, 1, 1, 2, 1]);
    }
}
