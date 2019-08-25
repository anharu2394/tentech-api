use futures::{stream::Stream, Future};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_s3::S3Client;

pub fn initial_s3_client() -> S3Client {
    let mut provider = DefaultCredentialsProvider::new().unwrap();

    S3Client::new(Region::ApNortheast1)
}
