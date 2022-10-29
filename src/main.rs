use secp256k1::ffi::KeyPair;
use wallet_lib::create_txn_object;

pub mod wallet_lib;

const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

use tokio;
use web3::types::{Address};

#[tokio::main]
async fn main() -> Result<()> {
    let keypair = wallet_lib::create_keypair();

    let web3 = wallet_lib::establish_web3_connection(URL)?;
    let to_adrs = Address::from_str("0x08302CF8648A961c607e3e7Bd7B73c3230c2A6c5")?;
    let tx_object = wallet_lib::create_txn_object(to_adrs, keypair.0 )?;
    let result= wallet_lib::sign_and_send(web3.clone(),tx_object,keypairs.0).await;

    println!("result {:?}",result);

    Ok(())
}
