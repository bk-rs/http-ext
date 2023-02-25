use bytes::{Bytes, BytesMut};
use futures_util::{Stream, StreamExt as _};

use crate::Error;

pub async fn bytes_stream_to_bytes(
    mut stream: impl Stream<Item = Result<Bytes, Error>> + Unpin,
) -> Result<Bytes, Error> {
    let mut bytes_mut = BytesMut::new();
    while let Some(bytes) = stream.next().await {
        let bytes = bytes?;
        bytes_mut.extend_from_slice(&bytes[..]);
    }
    Ok(bytes_mut.freeze())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bytes_stream_to_bytes() {
        let stream =
            futures_util::stream::once(async { Ok(Bytes::copy_from_slice(b"foo")) }).boxed();
        assert_eq!(
            bytes_stream_to_bytes(stream).await.unwrap(),
            Bytes::copy_from_slice(b"foo")
        );
    }
}
