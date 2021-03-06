use megam_api::api::Api;
use megam_api::util::csars::Csar;
use rustc_serialize::json;
use megam_api::api::Options;

//#[test]
fn create() {
    // create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

//example yaml string
    a.desc = "
tosca_definitions_version: tosca_simple_yaml_1_0
 
description: Template for firing up an Ubuntu instance
node_templates:
 
 Ubuntu:
  type: tosca.torpedo.ubuntu
  properties:
    # omitted here for sake of brevity
  requirements:
    - host: meg1146028290067267584
    - domain: megam.io
".to_string();   

     match a.create(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

//#[test]
fn list() {
		// create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

   	 match a.list(json::encode(&options).unwrap()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

#[test]
fn push() {
		// create hashmap for api settings
    let options = Options {
    Email: "b@a.com".to_string(),
    Apikey: "GGUdgT-sjLXn9EYRkS8Q7g==".to_string(),
    Host: "http://localhost:9000".to_string(),
    Version: "/v2".to_string(),
    };

     println!("{:?}", json::encode(&options).unwrap());

    let mut a = Csar::new();		

   	 match a.push(json::encode(&options).unwrap(), "CSR1218494963256524800".to_string()) {
        Ok(n) => println!("result: Is OK: {:?}", n),
        Err(FailOne) => println!("result: Failed One: {:?}", FailOne),
    }
}

