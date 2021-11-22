#![deny(
    //  missing_docs, // not compatible with big_array
      trivial_casts,
      trivial_numeric_casts,
      unsafe_code,
      unused_import_braces,
      unused_qualifications,
      warnings
  )]

mod utils;

use std::{
    io::{self, Read,BufRead},
    str,
    fs::File
};
use serde::{Deserialize, Deserializer, de::Error};
use lazy_static::lazy_static;
use anyhow::{Context, Result, anyhow};
use web3::{
    contract::{Contract, Options},
    types::{H160, H256},
    ethabi::Token,
    transports::Http   
};

use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use utils::*;

const CONFIG_PATH: &str = "./config.json";

#[derive(Deserialize, Debug, PartialEq)]
struct Config{
    pub web3_transport: String,

    #[serde(deserialize_with = "to_address")]
    pub contract_address: H160,

    #[serde(deserialize_with = "to_address")]
    pub wallet_address: H160,

}

fn to_address<'de, D>(deserializer: D) -> Result<H160, D::Error>
    where D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut bytes = [0u8; 20];

    hex::decode_to_slice(s, &mut bytes).map_err(D::Error::custom)?;
    Ok(H160(bytes))
}

fn read_line() -> Result<String> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line =  iterator.next().ok_or("no input").map_err(anyhow::Error::msg)??;
    Ok(line)
}

fn setup_config(path: &str) -> Result<Config> {
  
    let mut file = File::open(path)?;

    let mut buff = String::new(); 
    file.read_to_string(&mut buff)?;

    let config: Config = serde_json::from_str(&buff)
        .context("config JSON was not well-formatted")?;

    Ok(config)
}

lazy_static! {
    static ref CONFIG: Config = match setup_config(CONFIG_PATH){
        Ok(config) => config,
        Err(err) => {
            panic!("\n{:?}", err);
        }
    };   
}

async fn print_gas_and_balance(web3: &web3::Web3<Http>, tx_hash: H256, wallet_address: H160) -> Result<()>{
    println!("Transaction hash: {:?}",  tx_hash);
    match web3.eth().transaction_receipt(tx_hash).await?{
        Some(receipt) => {
            let gas_used = receipt.gas_used.ok_or("no gas used!!!").map_err(anyhow::Error::msg)?;
            println!("Gas used: {:?}",  gas_used);
            let balance = web3.eth().balance(wallet_address, None).await?;
        
            println!("Account Balance: {}",  balance);
        },
        None =>  return Err(anyhow!("Could not find transaction receipt, but the transaction comletes")),
    };
    Ok(())
}


#[actix_rt::main]
async fn main() -> Result<()> {

    // let cid_b58 = "QmTJd6JnxTGrLgJqLfnhMHXaytrSaBHvos4ECeVTvqwHdi";
 
    let web3_transport = CONFIG.web3_transport.clone();
    let contract_address = CONFIG.contract_address;
    let wallet_address = CONFIG.wallet_address;

    let client = IpfsClient::default();

    #[allow(unused_assignments)]
    let mut cid = String::new();

    loop{
        println!("\n\n\n\n\n");
        println!("------Please insert a file path or q to exit--------");
        println!("\n\n");
        println!("\n\n");

        let file_path = read_line()?;

        match File::open(file_path.clone()).context(format!("Could not find file in {}", file_path)){
            Ok(file) => {
                        //Add to IPFS
                cid = match client.add(file).await {
                    Ok(file) => {
                        println!("File uploaded successfully to IPFS, cid is: {:?}", file.hash); 
                        file.hash
                    },
                    Err(e) =>  {
                        println!("Error uploading the file: {}", e);
                        continue;
                    },
                };

                break;
            },
            Err(_)=> {
                println!("Could not find file in {}", file_path);
                continue;
            }
        }
    }


    //Connection to Ethereum
    let transport = web3::transports::Http::new(&web3_transport)?;
    let web3 = web3::Web3::new(transport);

    //Get contract api from the contract address and the Application Binary Interface (contract.abi)
    let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("contract.abi"))?;

    let smart_cont_str_cid: String = contract.query("getCIDAsString", (), None, Options::default(), None).await?;
    let smart_cont_struct_cid: Token = contract.query("getCIDAsStruct", (), None, Options::default(), None).await?;

    if smart_cont_str_cid == cid {
        println!("This cid is already stored in the smart contract as string, don't waste gas!!!"); 
    }

    if multi_hash_token_to_cid(smart_cont_struct_cid)? == cid{
        println!("This cid is already stored in the smart contract as multihash struct, don't waste gas!!!"); 
    }

    loop{
        println!("\n\n\n\n\n");
        println!("------Please insert a file path--------");
        println!("------1 to store as a string--------");
        println!("------2 to store as a multihash struct--------");
        println!("------q to exit--------");
        println!("\n\n");
    
        let choice = read_line()?;
        println!("\n\n");
    
        match  choice.as_ref() {
            "1" => {
                let tx_hash = contract.call("storeCIDAsString", cid.clone(), wallet_address, Options::default()).await?;
    
                print_gas_and_balance(&web3, tx_hash, wallet_address).await?;
                println!("Quiting");
                break;
            },
            "2" => {
                let multi_hash = IPFSMultihash::new(&cid)?;
                let inputs = (multi_hash.digest, multi_hash.hash_code, multi_hash.size);
                let tx_hash = contract.call("storeCIDAsStruct", inputs, wallet_address, Options::default()).await?;
    
                print_gas_and_balance(&web3, tx_hash, wallet_address).await?;
                println!("Quiting");
                break;
            },
            "q"=> return Ok(()),
            _=> println!("invalid key")
    
        }
    }

    Ok(())
}



#[cfg(test)]
mod tests {
    
    use super::*;
    use hex_literal::hex;


    #[test]
    fn test_deserialize_config() {
        let data = r#"{
            "web3_transport": "http://127.0.0.1:7545",
            "contract_address": "d5f9b7c42d683Dd33A8E4E3318f599A678303ba3",
            "wallet_address": "56ddE94C59FE43dc889dF94E841441364ab66b9B"
        }"#;

        let expected =  Config{
            web3_transport:  "http://127.0.0.1:7545".to_owned(),
            contract_address: H160(hex!("d5f9b7c42d683Dd33A8E4E3318f599A678303ba3")),
            wallet_address: H160(hex!("56ddE94C59FE43dc889dF94E841441364ab66b9B"))
        };
        let result: Config = serde_json::from_str(&data).unwrap();

        assert_eq!(expected, result);
    }
}
