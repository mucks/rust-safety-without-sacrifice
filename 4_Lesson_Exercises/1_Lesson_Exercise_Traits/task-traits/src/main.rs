use anyhow::Result;
use std::fmt::Debug;

trait Coin: Debug {
    fn denom_pow(&self) -> u8;
    fn symbol(&self) -> String;
    fn send(&self, to: &str, amount: u64) -> Result<()>;

    fn denom(&self) -> u64 {
        10u64.pow(self.denom_pow() as u32)
    }
}

trait SupportsStaking: Coin {
    fn stake(&self, amount: u64) -> Result<()>;
    fn staking_provider(&self) -> String;
}

#[derive(Debug, Clone, Default)]
struct Bitcoin {
    utxo_count: u32,
    node_url: String,
    network: String,
}

impl Coin for Bitcoin {
    fn denom_pow(&self) -> u8 {
        9
    }

    fn symbol(&self) -> String {
        "BTC".to_string()
    }

    fn send(&self, to: &str, amount: u64) -> Result<()> {
        println!(
            "Bitcoin Transaction: Sending {} {} to {}",
            amount as f64 / self.denom() as f64,
            self.symbol(),
            to
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct Ethereum {
    node_url: String,
    network: String,
    is_archive: bool,
}

impl Coin for Ethereum {
    fn denom_pow(&self) -> u8 {
        18
    }

    fn symbol(&self) -> String {
        "ETH".to_string()
    }

    fn send(&self, to: &str, amount: u64) -> Result<()> {
        println!(
            "Ethereum Transaction: Sending {} {} to {}",
            amount as f64 / self.denom() as f64,
            self.symbol(),
            to
        );
        Ok(())
    }
}

impl SupportsStaking for Ethereum {
    fn stake(&self, amount: u64) -> Result<()> {
        println!(
            "Staking {} {} on Ethereum network",
            amount as f64 / self.denom() as f64,
            self.symbol()
        );
        Ok(())
    }

    fn staking_provider(&self) -> String {
        "Figment".to_string()
    }
}

#[derive(Debug, Clone, Default)]
struct Solana {
    uses_firedancer: bool,
    node_url: String,
    network: String,
    is_valid: bool,
}

impl Coin for Solana {
    fn denom_pow(&self) -> u8 {
        9
    }

    fn symbol(&self) -> String {
        "SOL".to_string()
    }

    fn send(&self, to: &str, amount: u64) -> Result<()> {
        println!(
            "Solana Transaction: Sending {} {} to {}",
            amount as f64 / self.denom() as f64,
            self.symbol(),
            to
        );
        Ok(())
    }
}

impl SupportsStaking for Solana {
    fn stake(&self, amount: u64) -> Result<()> {
        println!(
            "Staking {} {} on Solana network",
            amount as f64 / self.denom() as f64,
            self.symbol()
        );
        Ok(())
    }

    fn staking_provider(&self) -> String {
        "Okx".to_string()
    }
}

fn static_log_send<C: Coin>(coin: &C, to: &str, amount: u64) -> Result<()> {
    println!("Static: Logging send operation for {}", coin.symbol());
    coin.send(to, amount)
}

fn dynamic_log_send(coin: &dyn Coin, to: &str, amount: u64) -> Result<()> {
    println!("Dynamic: Logging send operation for {}", coin.symbol());
    coin.send(to, amount)
}

trait SuperCoin: Coin {
    fn staking_logic(&self) -> Option<Box<dyn SupportsStaking>> {
        None
    }

    fn info(&self) {
        println!("SuperCoin Info: {}", self.symbol());
        println!("Supports Staking: {}", self.staking_logic().is_some());
        if let Some(staking) = self.staking_logic() {
            println!("Staking Provider: {}", staking.staking_provider());
        }
    }
}

impl<T: Coin> SuperCoin for T {}

fn main() -> anyhow::Result<()> {
    let coin_btc = Bitcoin::default();
    let coin_eth = Ethereum::default();
    let coin_sol = Solana::default();

    static_log_send(&coin_btc, "alice_btc_address", 150_000_000)?;
    static_log_send(&coin_eth, "alice_eth_address", 3_000_000_000_000_000_000)?;

    dynamic_log_send(&coin_btc, "bob_btc_address", 200_000_000)?;
    dynamic_log_send(&coin_eth, "bob_eth_address", 5_000_000_000_000_000_000)?;

    let coins: Vec<Box<dyn Coin>> = vec![
        Box::new(coin_btc.clone()),
        Box::new(coin_eth.clone()),
        Box::new(coin_sol.clone()),
    ];

    for coin in coins.iter() {
        println!("Coin Symbol: {}", coin.symbol());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Task 1: Add a get_balance method to the Coin trait and implement it for each coin. Also add
    // unit tests to verify the balances. You can also add a struct field to each coin to hold the balance.
    // And then also subtract the balance when doing a send operation. Might habe to change the method signatures
    // to use &mut self instead of &self.
    #[test]
    fn test_task1() -> Result<()> {
        let coins: Vec<Box<dyn Coin>> = vec![
            Box::new(Bitcoin::default()),
            Box::new(Ethereum::default()),
            Box::new(Solana::default()),
        ];

        let send_amounts = vec![1000, 2000, 3000];

        for (coin, amount) in coins.iter().zip(send_amounts.iter()) {
            coin.send("test_address", *amount)?;
        }

        let assertions = vec![
            ("BTC".to_string(), 1234),
            ("ETH".to_string(), 500),
            ("SOL".to_string(), 255),
        ];

        for ((symbol, balance), coin) in assertions.iter().zip(coins.iter()) {
            assert_eq!(&coin.symbol(), symbol);
            // TODO: Uncomment after implementing get_balance method
            // assert_eq!(coin.get_balance(), *balance);
        }

        Ok(())
    }

    // Task 2: Add a new trait SupportsSmartContracts with a method deploy_smart_contract.
    // With an argument code: &str & fees: u64. fees is the amount to pay for deploying the contract.
    // It should also be subtracted from the balance of the coin.
    // Also add a method that returns the smart contract language used by the coin.
    // Implement this trait for Ethereum and Solana. Add unit tests to verify the functionality.
    // Also add it to the SuperCoin trait similar on how it is done for SupportsStaking.
    #[test]
    fn test_task2() -> Result<()> {
        let eth = Ethereum::default();
        let sol = Solana::default();

        // TODO: uncomment below after implementing SupportsSmartContracts trait

        // let supports_smart_contracts: Vec<&dyn SupportsSmartContracts> = vec![&eth, &sol];

        // let code_amounts = vec![("MyEthereumContract", 1000), ("MySolanaContract", 2000)];

        // for (coin, (code, fees)) in supports_smart_contracts.iter().zip(code_amounts.iter()) {
        //     coin.deploy_smart_contract(code, *fees)?;
        // }

        // let assertions = vec![("ETH".to_string(), 233), ("SOL".to_string(), 884)];

        // for ((symbol, balance), coin) in assertions.iter().zip(supports_smart_contracts.iter()) {
        //     assert_eq!(&coin.symbol(), symbol);
        //     assert_eq!(coin.get_balance(), *balance);
        // }

        let super_coins: Vec<Box<dyn SuperCoin>> = vec![Box::new(eth), Box::new(sol)];

        for coin in super_coins.iter() {
            coin.info();

            // TODO: uncomment after implementation
            // if let Some(smart_contract) = coin.smart_contract_logic() {
            //     println!(
            //         "Smart Contract Language: {}",
            //         smart_contract.smart_contract_language()
            //     );
            // }
        }

        Ok(())
    }
}
