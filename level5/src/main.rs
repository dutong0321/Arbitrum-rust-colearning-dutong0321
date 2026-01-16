use ethers::{
    contract::{Contract},
    providers::{Provider, Http, ProviderExt, Middleware},
    types::{Address, U256, U64},
    abi::Abi,
};
use std::str::FromStr;
use std::sync::Arc;

const ERC20_ABI: &str = r#"[
    {
        "constant": true,
        "inputs": [],
        "name": "name",
        "outputs": [
            {
                "name": "",
                "type": "string"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "symbol",
        "outputs": [
            {
                "name": "",
                "type": "string"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "decimals",
        "outputs": [
            {
                "name": "",
                "type": "uint8"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "totalSupply",
        "outputs": [
            {
                "name": "",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }
]"#;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    //let contract_address = Address::from_str("0xc275B23C035a9d4EC8867b47f55427E0bDCe14cB")?;
    //let contract_address = Address::from_str("0xda71c3f9bd2f9513ac1a38f68e139bb4a475aa9d")?;
    let contract_address = Address::from_str("0xc275B23C035a9d4EC8867b47f55427E0bDCe14cB")?;
    
    println!("Connecting to Arbitrum Sepolia testnet...");
    let provider = Arc::new(Provider::<Http>::try_connect(rpc_url).await?);
    println!("Connected successfully!");
    
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);
    
    println!("\nParsing ABI...");
    let abi: Abi = serde_json::from_str(ERC20_ABI)?;
    println!("ABI parsed successfully!");
    
    println!("\nCreating contract instance...");
    let contract = Contract::new(contract_address, abi, provider);
    println!("Contract instance created successfully!");
    
    println!("\nCalling name() method...");
    let name: String = contract.method::<_, String>("name", ())?.call().await?;
    println!("Contract name: {}", name);
    
    println!("\nCalling symbol() method...");
    let symbol: String = contract.method::<_, String>("symbol", ())?.call().await?;
    println!("Contract symbol: {}", symbol);
    
    println!("\nCalling decimals() method...");
    let decimals: u8 = contract.method::<_, u8>("decimals", ())?.call().await?;
    println!("Contract decimals: {}", decimals);
    
    println!("\nCalling totalSupply() method...");
    let total_supply: U256 = contract.method::<_, U256>("totalSupply", ())?.call().await?;
    println!("Contract total supply: {}", total_supply);
    
    println!("\nContract interaction completed successfully!");
    
    Ok(())
}