use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, U256};
use alloy::signers::{local::LocalSigner};
use alloy::consensus::TxLegacy;
use alloy::consensus::transaction::RlpEcdsaEncodableTx;
use alloy::network::TxSignerSync;
use alloy::primitives::TxKind;
use dotenv::dotenv;
use std::error::Error;
use std::env;
use std::str::FromStr;
use k256::ecdsa::SigningKey;

/// 获取Arbitrum测试网的Gas价格
async fn get_gas_price() -> Result<u128, Box<dyn Error>> {
    // 设置Arbitrum Sepolia测试网的RPC URL
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?;
    
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let gas_price = provider.get_gas_price().await?;
    
    Ok(gas_price)
}

/// 计算预估转账Gas费
fn calculate_transaction_fee(gas_price: u128, gas_limit: Option<u64>) -> u128 {
    let gas_limit = gas_limit.unwrap_or(21000);
    gas_price * gas_limit as u128
}

/// 从环境变量加载私钥并创建签名者
fn load_signer() -> Result<LocalSigner<SigningKey>, Box<dyn Error>> {
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY").map_err(|_| "PRIVATE_KEY environment variable not set")?;
    // 确保私钥格式正确，移除0x前缀如果存在
    let private_key = private_key.trim_start_matches("0x");
    
    // 验证私钥长度
    if private_key.len() != 64 {
        return Err("Private key must be 64 hexadecimal characters long".into());
    }
    
    // 验证私钥只包含十六进制字符
    if !private_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Private key must only contain hexadecimal characters".into());
    }
    
    let signer = LocalSigner::from_str(private_key)?;
    Ok(signer)
}

/// 执行ETH转账
async fn transfer_eth(
    signer: &LocalSigner<SigningKey>,
    to_address: Address,
    amount_wei: U256,
) -> Result<String, Box<dyn Error>> {
    // 设置Arbitrum Sepolia测试网的RPC URL
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    
    // 获取发送地址
    let from_address = signer.address();
    println!("From address: {}", from_address);
    
    // 检查发送地址余额
    let balance = provider.get_balance(from_address).await?;
    println!("Current balance: {} wei", balance);
    if balance < amount_wei {
        return Err("Insufficient balance".into());
    }
    
    // 获取Gas价格
    let gas_price = get_gas_price().await?;
    println!("Current gas price: {} wei", gas_price);
    
    // 计算Gas费
    let gas_limit = 25000u64; // 增加 gas limit 以避免 "intrinsic gas too low" 错误
    let estimated_fee = calculate_transaction_fee(gas_price, Some(gas_limit));
    println!("Estimated transaction fee: {} wei", estimated_fee);
    
    // 检查总花费（转账金额 + Gas费）
    let total_cost = amount_wei + U256::from(estimated_fee);
    if balance < total_cost {
        return Err("Insufficient balance for transfer + gas fee".into());
    }
    
    // 获取当前nonce
    let nonce = provider.get_transaction_count(from_address).await?;
    println!("Current nonce: {}", nonce);
    
    // 获取链ID
    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);
    
    // 构建传统交易
    let mut tx = TxLegacy {
        to: TxKind::Call(to_address),
        value: amount_wei,
        gas_limit: gas_limit,
        gas_price: gas_price * 2,
        nonce: nonce,
        input: Default::default(),
        chain_id: Some(chain_id),
        ..Default::default()
    };
    
    // 签名交易
    let signature = signer.sign_transaction_sync(&mut tx)?;
    
    // 编码签名后的交易
    let mut signed_tx = Vec::new();
    tx.rlp_encode_signed(&signature, &mut signed_tx);
    
    // 发送交易
    let tx_hash = send_raw_transaction(&provider, &signed_tx).await?;
    println!("Transaction sent! Hash: {}", tx_hash);
    
    Ok(tx_hash)
}



/// 发送原始交易
async fn send_raw_transaction<P: Provider>(provider: &P, signed_tx: &[u8]) -> Result<String, Box<dyn Error>> {
    let pending_tx = provider.send_raw_transaction(signed_tx).await?;
    let tx_hash = pending_tx.tx_hash();
    Ok(tx_hash.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum测试网ETH转账脚本 ===");
    
    // 从环境变量加载私钥
    let signer = load_signer()?;
    let from_address = signer.address();
    println!("From address: {}", from_address);
    
    // 从环境变量加载目标地址
    let to_address_str = env::var("TO_ADDRESS").map_err(|_| "TO_ADDRESS environment variable not set")?;
    let to_address: Address = Address::from_str(&to_address_str)?;
    println!("To address: {}", to_address);
    
    // 从环境变量加载转账金额（ETH）
    let amount_eth = env::var("AMOUNT_ETH").map_err(|_| "AMOUNT_ETH environment variable not set")?;
    // 正确计算 amount_wei
    let amount_wei = {
        let parts: Vec<&str> = amount_eth.split('.').collect();
        match parts.len() {
            1 => {
                // 没有小数点，直接添加 18 个零
                U256::from_str(&format!("{}000000000000000000", parts[0]))?
            },
            2 => {
                // 有小数点，处理整数和小数部分
                let integer_part = parts[0];
                let mut fractional_part = parts[1].to_string();
                // 确保小数部分不超过 18 位
                if fractional_part.len() > 18 {
                    fractional_part = fractional_part[..18].to_string();
                } else if fractional_part.len() < 18 {
                    // 不足 18 位，补零
                    fractional_part = format!("{:0<18}", fractional_part);
                }
                U256::from_str(&format!("{}{}", integer_part, fractional_part))?
            },
            _ => {
                return Err("Invalid ETH amount format".into());
            }
        }
    };
    println!("Transfer amount: {} ETH", amount_eth);
    
    // 执行转账
    let tx_hash = transfer_eth(&signer, to_address, amount_wei).await?;
    println!("\n=== 转账完成 ===");
    println!("交易哈希: {}", tx_hash);
    println!("请在Arbitrum测试网浏览器中查询交易状态:");
    println!("https://sepolia.arbiscan.io/tx/{}", tx_hash);
    
    Ok(())
}