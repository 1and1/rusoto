#![cfg(feature = "sdb")]
extern crate rusoto_core;
extern crate rusoto_sdb;
extern crate env_logger;

use rusoto_sdb::{SimpleDb, SimpleDbClient, ListDomainsRequest, CreateDomainRequest, DeleteDomainRequest};
use rusoto_core::Region;

// See https://github.com/rusoto/rusoto/issues/978 for details on why these tests are ignored.

#[test]
#[ignore]
fn should_list_domains() {
    let _ = env_logger::try_init();
    let client = SimpleDbClient::simple(Region::UsEast1);
    let request = ListDomainsRequest::default();

    let result = client.list_domains(&request).sync().unwrap();
    println!("{:#?}", result);
}

 #[test]
 #[ignore]
 fn roundtrip_test() {
     let _ = env_logger::try_init();
     let client = SimpleDbClient::simple(Region::UsEast1);
     let test_domain = "rusoto_domain".to_string();

     // Create domain
     let create_domain_req = CreateDomainRequest {
         domain_name: test_domain.clone(),
     };
     let create_result = client.create_domain(&create_domain_req).sync();
     println!("create domain result: {:#?}", create_result);
     assert!(create_result.is_ok());

     // see domain in list (may take up to 10 seconds)
     let request = ListDomainsRequest::default();
     let list_domains_result = client.list_domains(&request).sync();
     println!("list domains result: {:#?}", list_domains_result);
     assert!(list_domains_result.is_ok());

     // put attributes

     // get attributes

     // delete attributes

     // delete domain
     let delete_req = DeleteDomainRequest {
         domain_name: test_domain,
     };
     let delete_result = client.delete_domain(&delete_req).sync();
     println!("delete domain result: {:#?}", delete_result);
     assert!(delete_result.is_ok());
 }