use std::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

/// Creates a stream that yields a single item.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::once(7);
///
/// assert_eq!(s.next().await, Some(7));
/// assert_eq!(s.next().await, None);
/// #
/// # })
/// ```
pub fn once<T>(t: T) -> Once<T> {
    Once { value: Some(t) }
}

pin_project! {
    /// A stream that yields a single item.
    ///
    /// This stream is constructed by the [`once`] function.
    ///
    /// [`once`]: fn.once.html
    #[derive(Debug)]
    pub struct Once<T> {
        value: Option<T>,
    }
}

impl<T: Unpin> Stream for Once<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<T>> {
        Poll::Ready(self.project().value.take())
    }
}
