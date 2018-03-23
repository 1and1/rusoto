#![cfg(feature = "cloudfront")]

extern crate rusoto_core;
extern crate rusoto_cloudfront;
extern crate env_logger;

use rusoto_cloudfront::{CloudFront, CloudFrontClient, ListDistributionsRequest};
use rusoto_core::Region;

#[test]
fn should_list_distributions() {
    let _ = env_logger::try_init();
    let client = CloudFrontClient::simple(Region::UsEast1);
    let request = ListDistributionsRequest::default();

    let result = client.list_distributions(&request).sync();
    println!("{:#?}", result);
    assert!(result.is_ok());
}

