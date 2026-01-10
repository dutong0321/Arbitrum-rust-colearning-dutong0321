use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, U256};
use std::error::Error;

/// 查询Arbitrum测试网地址的ETH余额，并转换为可读格式
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 私钥打码
    let address_str = "0xA2ABFAae943CC34817A19b5bDaF71B6784f2B688";
    let balance = get_eth_balance(address_str).await?;
    println!("地址 {} 的ETH余额: {}", address_str, balance);
    Ok(())
}

/// 查询Arbitrum测试网地址的ETH余额
async fn get_eth_balance(address_str: &str) -> Result<String, Box<dyn Error>> {
    // 设置Arbitrum Sepolia测试网的RPC URL
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?;
    
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let address: Address = address_str.parse()?;
    let balance_wei: U256 = provider.get_balance(address).await?;
    
    // 将wei转换为ETH（1 ETH = 10^18 wei）
    // 使用字符串格式化处理，避免浮点数精度问题
    let balance_str = balance_wei.to_string();
    
    // 格式化输出：在第18位前插入小数点
    let formatted_balance = if balance_str.len() > 18 {
        // 余额大于等于1 ETH
        let (integer, fractional) = balance_str.split_at(balance_str.len() - 18);
        format!("{}.{} ETH", integer, fractional)
    } else {
        // 余额小于1 ETH，需要在前面补0
        let padding_zeros = "0".repeat(18 - balance_str.len());
        format!("0.{}{} ETH", padding_zeros, balance_str)
    };
    
    Ok(formatted_balance)
}