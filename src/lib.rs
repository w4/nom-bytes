use std::{iter::Enumerate, ops::Deref};

use nom::Needed;

/// A wrapper type for `bytes::Bytes` that implements all the traits `nom` needs
/// to be able to parse the inner value.
#[derive(Debug, Clone)]
pub struct BytesWrapper(bytes::Bytes);

impl Deref for BytesWrapper {
    type Target = bytes::Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<bytes::Bytes> for BytesWrapper {
    fn into(self) -> bytes::Bytes {
        self.0
    }
}

impl From<bytes::Bytes> for BytesWrapper {
    fn from(bytes: bytes::Bytes) -> Self {
        Self(bytes)
    }
}

impl nom::InputTake for BytesWrapper {
    fn take(&self, count: usize) -> Self {
        BytesWrapper::from(self.0.slice(0..count))
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (
            BytesWrapper::from(self.0.slice(count..)),
            BytesWrapper::from(self.0.slice(0..count)),
        )
    }
}

impl nom::FindSubstring<&[u8]> for BytesWrapper {
    fn find_substring(&self, substr: &[u8]) -> Option<usize> {
        (&self.0[..]).find_substring(substr)
    }
}

impl nom::FindSubstring<&str> for BytesWrapper {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        (&self.0[..]).find_substring(substr)
    }
}

impl nom::InputLength for BytesWrapper {
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> nom::InputIter for BytesWrapper {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = bytes::buf::IntoIter<bytes::Bytes>;

    fn iter_elements(&self) -> Self::IterElem {
        self.0.clone().into_iter()
    }

    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        (&self.0[..]).position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        (&self.0[..]).slice_index(count)
    }
}

impl nom::UnspecializedInput for BytesWrapper {}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use nom::bytes::complete::*;

    use crate::BytesWrapper;

    #[test]
    fn it_works() {
        let input = BytesWrapper::from(Bytes::from_static(
            b"this is my cool input, please don't copy from me!",
        ));

        let (rest, v) =
            take_till::<_, _, nom::error::Error<BytesWrapper>>(|v| v == b',')(input).unwrap();

        assert_eq!(&v[..], b"this is my cool input");
        assert_eq!(&rest[..], b", please don't copy from me!");
    }
}
