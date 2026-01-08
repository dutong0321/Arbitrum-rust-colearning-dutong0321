//! Example of creating an HTTP provider using the `connect_http` method on the `ProviderBuilder`.
 
use alloy::providers::{Provider, ProviderBuilder}; 
use alloy::primitives::Address;
use std::error::Error;
use alloy::sol;
 
// Generate the contract bindings for the ERC20 interface.
sol! { 
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)] 
   contract HelloWeb3 { 
        function hello_web3() pure public returns(string memory);
   } 
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?;
 
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");
    println!("Hello web3!");
 
    // Instantiate the contract instance.
    let weth: Address = "0x3f1f78ed98cd180794f1346f5bd379d5ec47de90".parse()?;
    let contract = HelloWeb3::new(weth, provider); 
 
    // Fetch the balance of WETH for a given address.
    let message = contract.hello_web3().call().await?;

    println!("合约返回: {message}");

    Ok(())
}