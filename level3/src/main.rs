use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::U256;
use std::error::Error;

/// 获取Arbitrum测试网的Gas价格
async fn get_gas_price() -> Result<U256, Box<dyn Error>> {
    // 设置Arbitrum Sepolia测试网的RPC URL
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?;
    
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let gas_price_u128 = provider.get_gas_price().await?;
    let gas_price = U256::from(gas_price_u128);
    
    Ok(gas_price)
}

/// 计算预估转账Gas费
fn calculate_transaction_fee(gas_price: U256, gas_limit: Option<U256>) -> U256 {
    let gas_limit = gas_limit.unwrap_or(U256::from(21000));
    gas_price * gas_limit
}

/// 将wei转换为ETH格式的字符串
fn format_wei_to_eth(wei_value: U256) -> String {
    let wei_str = wei_value.to_string();
    
    // 格式化输出：在第18位前插入小数点
    let formatted_value = if wei_str.len() > 18 {
        let (integer, fractional) = wei_str.split_at(wei_str.len() - 18);
        format!("{}.{} ETH", integer, fractional)
    } else {
        let padding_zeros = "0".repeat(18 - wei_str.len());
        format!("0.{}{} ETH", padding_zeros, wei_str)
    };
    
    formatted_value
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum测试网Gas费预估 ===");
    
    // 获取Gas价格
    let gas_price = get_gas_price().await?;
    println!("Gas价格: {} wei", gas_price);
    
    let base_gas_limit = U256::from(21000);
    println!("Gas限额: {} gas", base_gas_limit);
    
    // 计算预估Gas费
    let estimated_fee = calculate_transaction_fee(gas_price, Some(base_gas_limit));
    println!("\n=== 预估转账Gas费 ===");
    println!("预估Gas费: {} wei", estimated_fee);
    println!("预估Gas费: {}", format_wei_to_eth(estimated_fee));
    
    Ok(())
}