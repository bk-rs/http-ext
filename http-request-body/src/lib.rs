#[cfg(feature = "hyper-request-body")]
pub use hyper_request_body::{self, Body as HyperRequestBody};
#[cfg(feature = "warp-request-body")]
pub use warp_request_body::{self, Body as WarpRequestBody};

use core::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_util::Stream;
use pin_project_lite::pin_project;

pub mod error;
mod utils;

use error::Error;

//
pin_project! {
    #[project = BodyProj]
    pub enum Body {
        Bytes { inner: Bytes },
        Stream { #[pin] inner: Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send + 'static>> },
    }
}

impl core::fmt::Debug for Body {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bytes { inner } => f.debug_tuple("Bytes").field(&inner).finish(),
            Self::Stream { inner: _ } => write!(f, "Stream"),
        }
    }
}

impl core::fmt::Display for Body {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::Bytes {
            inner: Bytes::default(),
        }
    }
}

//
impl Body {
    pub fn with_bytes_from<T>(v: T) -> Self
    where
        T: Into<Bytes>,
    {
        Self::Bytes { inner: v.into() }
    }

    pub fn with_bytes(bytes: Bytes) -> Self {
        Self::Bytes { inner: bytes }
    }

    pub fn with_stream(stream: impl Stream<Item = Result<Bytes, Error>> + Send + 'static) -> Self {
        Self::Stream {
            inner: Box::pin(stream),
        }
    }

    #[cfg(feature = "hyper-request-body")]
    pub fn from_hyper_body(body: hyper_request_body::HyperBody) -> Self {
        use futures_util::TryStreamExt as _;

        Self::with_stream(body.map_err(|err| Error::Other(err.into())))
    }

    #[cfg(feature = "hyper-request-body")]
    pub fn from_hyper_request_body(body: hyper_request_body::Body) -> Self {
        use futures_util::TryStreamExt as _;
        use hyper_request_body::error::Error as HyperRequestBodyError;

        match body {
            hyper_request_body::Body::HyperBody { inner } => {
                Self::with_stream(inner.map_err(|err| Error::Other(err.into())))
            }
            body => Self::with_stream(body.map_err(|err| match err {
                HyperRequestBodyError::HyperError(err) => Error::Other(err.into()),
                HyperRequestBodyError::Other(err) => Error::Other(err),
            })),
        }
    }

    #[cfg(feature = "warp-request-body")]
    pub fn from_warp_request_body(body: warp_request_body::Body) -> Self {
        use futures_util::TryStreamExt as _;
        use warp_request_body::error::Error as WarpRequestBodyError;

        match body {
            warp_request_body::Body::HyperBody { inner } => {
                Self::with_stream(inner.map_err(|err| Error::Other(err.into())))
            }
            body => Self::with_stream(body.map_err(|err| match err {
                WarpRequestBodyError::HyperError(err) => Error::Other(err.into()),
                WarpRequestBodyError::WarpError(err) => Error::Other(err.into()),
            })),
        }
    }
}

#[cfg(feature = "hyper-request-body")]
impl From<hyper_request_body::HyperBody> for Body {
    fn from(body: hyper_request_body::HyperBody) -> Self {
        Self::from_hyper_body(body)
    }
}

#[cfg(feature = "hyper-request-body")]
impl From<hyper_request_body::Body> for Body {
    fn from(body: hyper_request_body::Body) -> Self {
        Self::from_hyper_request_body(body)
    }
}

#[cfg(feature = "warp-request-body")]
impl From<warp_request_body::Body> for Body {
    fn from(body: warp_request_body::Body) -> Self {
        Self::from_warp_request_body(body)
    }
}

impl Body {
    pub fn require_to_bytes_async(&self) -> bool {
        matches!(self, Self::Stream { inner: _ })
    }

    pub fn to_bytes(self) -> Bytes {
        match self {
            Self::Bytes { inner } => inner,
            Self::Stream { inner: _ } => panic!("Please call require_to_bytes_async first"),
        }
    }

    pub async fn to_bytes_async(self) -> Result<Bytes, Error> {
        match self {
            Self::Bytes { inner } => Ok(inner),
            Self::Stream { inner } => utils::bytes_stream_to_bytes(inner)
                .await
                .map_err(Into::into),
        }
    }
}

//
impl Stream for Body {
    type Item = Result<Bytes, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project() {
            BodyProj::Bytes { inner } => {
                if !inner.is_empty() {
                    let bytes = inner.clone();
                    inner.clear();
                    Poll::Ready(Some(Ok(bytes)))
                } else {
                    Poll::Ready(None)
                }
            }
            BodyProj::Stream { inner } => inner.poll_next(cx).map_err(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt as _;

    use super::*;

    #[tokio::test]
    async fn test_with_bytes() {
        //
        let bytes = Bytes::from_static(b"foo");
        let body = Body::with_bytes(bytes);
        assert!(matches!(body, Body::Bytes { inner: _ }));
        assert!(!body.require_to_bytes_async());
        assert_eq!(body.to_bytes(), Bytes::copy_from_slice(b"foo"));
    }

    #[tokio::test]
    async fn test_with_stream() {
        //
        let stream =
            futures_util::stream::once(async { Ok(Bytes::copy_from_slice(b"foo")) }).boxed();
        let body = Body::with_stream(stream);
        assert!(matches!(body, Body::Stream { inner: _ }));
        assert!(body.require_to_bytes_async());
        assert_eq!(
            body.to_bytes_async().await.unwrap(),
            Bytes::copy_from_slice(b"foo")
        );
    }

    #[cfg(feature = "hyper-request-body")]
    #[tokio::test]
    async fn test_from_hyper_body() {
        //
        let body = hyper_request_body::HyperBody::from("foo");
        let body = Body::from_hyper_body(body);
        assert!(matches!(body, Body::Stream { inner: _ }));
        assert!(body.require_to_bytes_async());
        assert_eq!(
            body.to_bytes_async().await.unwrap(),
            Bytes::copy_from_slice(b"foo")
        );
    }

    #[cfg(feature = "hyper-request-body")]
    #[tokio::test]
    async fn test_from_hyper_request_body() {
        //
        let body = hyper_request_body::Body::HyperBody {
            inner: hyper_request_body::HyperBody::from("foo"),
        };
        let body = Body::from_hyper_request_body(body);
        assert!(matches!(body, Body::Stream { inner: _ }));
        assert!(body.require_to_bytes_async());
        assert_eq!(
            body.to_bytes_async().await.unwrap(),
            Bytes::copy_from_slice(b"foo")
        );
    }

    #[cfg(feature = "warp-request-body")]
    #[tokio::test]
    async fn test_from_warp_request_body() {
        //
        let body = warp_request_body::Body::Bytes {
            inner: Bytes::from_static(b"foo"),
        };
        let body = Body::from_warp_request_body(body);
        assert!(matches!(body, Body::Stream { inner: _ }));
        assert!(body.require_to_bytes_async());
        assert_eq!(
            body.to_bytes_async().await.unwrap(),
            Bytes::copy_from_slice(b"foo")
        );
    }
}
