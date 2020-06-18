// extension trait
pub trait IteratorExt: Iterator{
    fn do_flatten(self) -> Flatten<Self>
        where
            Self: Sized,
            Self::Item: IntoIterator;
}

impl<T> IteratorExt for T where T: Iterator{

    fn do_flatten(self) -> Flatten<Self> where Self: Sized, Self::Item: IntoIterator {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
    where I: IntoIterator,
          I::Item: IntoIterator {
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator {
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator {
    fn new(iter: O) -> Self {
        Flatten { outer: iter, inner: None }
    }
}

impl<O> Iterator for Flatten<O>
    where O: Iterator,
          O::Item: IntoIterator {
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }

            // self.outer.next().and_then(|inner| inner.into_iter().next())
            let inner_item = self.outer.next()?.into_iter();
            self.inner = Some(inner_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![1])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![1, 2])).count(), 2);
    }

    #[test]
    fn two_onces() {
        assert_eq!(flatten(vec![vec![1], vec![2]]).count(), 2);
    }

    #[test]
    fn two_onces_ext_flatten() {
        assert_eq!(vec![vec![1], vec![2]].into_iter().do_flatten().count(), 2);
    }
}
