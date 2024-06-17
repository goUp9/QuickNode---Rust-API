//-----------------------------------------------------getBlockProduction

// use reqwest::header;
// use reqwest::Client;
// use std::error::Error;
// use std::fs::File;
// use std::io::Write;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());

//     let client = Client::new();
//     let json_data = r#"
//     {
//         "jsonrpc": "2.0",
//         "id": 1,
//         "method": "getBlockProduction"
//     }
//     "#;
//     let response = client
//         .post("https://docs-demo.solana-mainnet.quiknode.pro/")
//         .headers(headers)
//         .body(json_data)
//         .send()
//         .await?; 

//     let body = response.text().await?;
//     println!("{}", body);

//     let mut file = File::create("response.txt")?;
//     file.write_all(body.as_bytes())?;

//     Ok(())
// }



//--------------------------------------getBlockTime
/* use reqwest::header;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlockTime",
        "params": [
            94101948
        ]
    }
    "#;
    let response = client
        .post("https://docs-demo.solana-mainnet.quiknode.pro/")
        .headers(headers)
        .body(json_data)
        .send()
        .await?; 

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
*/
/* 
use reqwest::header;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlocks",
        "params": [
            5,
            10
        ]
    }
    "#;
    let response = client
        .post("https://docs-demo.solana-mainnet.quiknode.pro/")
        .headers(headers)
        .body(json_data)
        .send()
        .await?; 

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
*/

//---------------------------------------getBlocksWithLimit(Same as getBlock)
/*
use reqwest::header;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlocksWithLimit",
        "params": [
            5,
            3
        ]
    }
    "#;
    let response = client
        .post("https://docs-demo.solana-mainnet.quiknode.pro/")
        .headers(headers)
        .body(json_data)
        .send()
        .await?; 

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}

 */



 //--------------------------------getClusterNodes

 
/*
 use reqwest::header;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getClusterNodes"
    }
    "#;
    let response = client
        .post("https://docs-demo.solana-mainnet.quiknode.pro/")
        .headers(headers)
        .body(json_data)
        .send()
        .await?; 

    let body = response.text().await?;
    println!("{}", body);

    let mut file = File::create("GetClusterNodes.txt")?;
    file.write_all(body.as_bytes())?;

    Ok(())
}
 */


 // --------------------------GetConfirmed




/*
use reqwest::header;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     let mut headers = header::HeaderMap::new();
     headers.insert("Content-Type", "application/json".parse().unwrap());
     
     let client = Client::new();
     let json_data = r#"
     {
     "jsonrpc": "2.0",
     "id": 1,
     "method": "getConfirmedBlock",
     "params": [
        94101948,
        {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false
    }
    ]
}
"#;
let response = client
.post("https://docs-demo.solana-mainnet.quiknode.pro/")
.headers(headers)
.body(json_data)
.send()
.await?; 

let body = response.text().await?;
println!("{}", body);
let mut confirmed_block =  File::create("confirmedb")?;
confirmed_block.write_all(body.as_bytes())?;
Ok(())
}

*/



//------------------------------------getConfirmedSignaturesForAddress2
use reqwest::header;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::new();
    let json_data = r#"
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getConfirmedSignaturesForAddress2",
        "params": [
            "Vote111111111111111111111111111111111111111",
            {
                "limit": 10
            }
        ]
    }
    "#;
    let response = client
        .post("https://docs-demo.solana-mainnet.quiknode.pro/")
        .headers(headers)
        .body(json_data)
        .send()
        .await?; 

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}









































































