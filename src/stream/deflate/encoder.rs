use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use flate2::Compression;
use futures::stream::Stream;
use pin_project::unsafe_project;

/// A deflate encoder, or compressor.
///
/// This structure implements a [`Stream`] interface and will read uncompressed data from an
/// underlying stream and emit a stream of compressed data.
#[unsafe_project(Unpin)]
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "deflate")))]
pub struct DeflateEncoder<S: Stream<Item = Result<Bytes>>> {
    #[pin]
    inner: crate::stream::generic::Encoder<S, crate::codec::DeflateEncoder>,
}

impl<S: Stream<Item = Result<Bytes>>> DeflateEncoder<S> {
    /// Creates a new encoder which will read uncompressed data from the given stream and emit a
    /// compressed stream.
    pub fn new(stream: S, level: Compression) -> Self {
        Self {
            inner: crate::stream::generic::Encoder::new(
                stream,
                crate::codec::DeflateEncoder::new(level),
            ),
        }
    }

    /// Acquires a reference to the underlying stream that this encoder is wrapping.
    pub fn get_ref(&self) -> &S {
        self.inner.get_ref()
    }

    /// Acquires a mutable reference to the underlying stream that this encoder is wrapping.
    ///
    /// Note that care must be taken to avoid tampering with the state of the stream which may
    /// otherwise confuse this encoder.
    pub fn get_mut(&mut self) -> &mut S {
        self.inner.get_mut()
    }

    /// Acquires a pinned mutable reference to the underlying stream that this encoder is wrapping.
    ///
    /// Note that care must be taken to avoid tampering with the state of the stream which may
    /// otherwise confuse this encoder.
    pub fn get_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut S> {
        self.project().inner.get_pin_mut()
    }

    /// Consumes this encoder returning the underlying stream.
    ///
    /// Note that this may discard internal state of this encoder, so care should be taken
    /// to avoid losing resources when this is called.
    pub fn into_inner(self) -> S {
        self.inner.into_inner()
    }
}

impl<S: Stream<Item = Result<Bytes>>> Stream for DeflateEncoder<S> {
    type Item = Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes>>> {
        self.project().inner.poll_next(cx)
    }
}

fn _assert() {
    crate::util::_assert_send::<DeflateEncoder<Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>>>(
    );
    crate::util::_assert_sync::<DeflateEncoder<Pin<Box<dyn Stream<Item = Result<Bytes>> + Sync>>>>(
    );
}
