use std::env;
use std::io::prelude::*;
use std::fs::File;
use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;

#[tokio::main]
pub async fn main() {

    println!("We are going to upload a file to azure");
    let account = env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    println!("using account -> {}", account);
    
    let sas = env::var("SAS").expect("Set env variable SAS first!");
    println!("using sas -> {}", sas);
    
    let container = env::var("CONTAINER").expect("Set env variable CONTAINER first!");
    println!("using container -> {}", container);

    let source = "./test.txt";
    let blob_name = "test.txt";
    let mut buffer = Vec::new();
    let mut f = File::open(source).expect("unable to open the file");
    f.read_to_end(&mut buffer).expect("failed to read file");
    let client = client::with_azure_sas(&account, &sas);

    let res = client
        .put_block_blob()
        .with_container_name(&container)
        .with_blob_name(blob_name)
        .with_body(&buffer[..])
        .finalize().await;

    match res {
        Ok(response) => println!("response:{:?}", response),
        Err(err) => println!("error:{:?}", err),
    };
}
