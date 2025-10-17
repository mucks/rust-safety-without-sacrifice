use std::fmt;

use thiserror::Error;

enum ClassicTxError {
    InvalidFormat,
    InsufficientFunds,
    Unauthorized,
    SpecificTxError(SpecificTxError),
    // Task 1 Hint: Add a new variant here
}

impl fmt::Display for ClassicTxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassicTxError::InvalidFormat => write!(f, "Transaction has an invalid format"),
            ClassicTxError::InsufficientFunds => write!(f, "Insufficient funds for transaction"),
            ClassicTxError::Unauthorized => write!(f, "Transaction is unauthorized"),
            ClassicTxError::SpecificTxError(err) => {
                write!(f, "Specific transaction error: {}", err)
            }
        }
    }
}

impl From<SpecificTxError> for ClassicTxError {
    fn from(err: SpecificTxError) -> Self {
        ClassicTxError::SpecificTxError(err)
    }
}

// This is using the thiserror crate, which is the common way to define errors in modern Rust.
#[derive(Debug, Error, PartialEq)]
enum ModernTxError {
    #[error("Transaction has an invalid format")]
    InvalidFormat,
    #[error("Insufficient funds for transaction")]
    InsufficientFunds,
    #[error("Transaction is unauthorized")]
    Unauthorized,
    #[error("Specific transaction error: {0}")]
    SpecificTxError(#[from] SpecificTxError),
    #[error("Inalid address provided")]
    InvalidAddress,
}

#[derive(Debug, Error, PartialEq)]
struct SpecificTxError {
    details: String,
    currency: String,
}

impl fmt::Display for SpecificTxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.currency, self.details)
    }
}

fn send_specific_tx(tx_details: &str) -> Result<(), SpecificTxError> {
    if tx_details != "btc" {
        return Err(SpecificTxError {
            details: "Only BTC transactions are supported".to_string(),
            currency: "BTC".to_string(),
        });
    }
    // Simulate a specific transaction error
    Err(SpecificTxError {
        details: tx_details.to_string(),
        currency: "USDT".to_string(),
    })
}

fn display_classic_tx_error() -> Result<(), ClassicTxError> {
    println!("Classic Transaction Errors:");
    println!("{}", ClassicTxError::InvalidFormat);
    println!("{}", ClassicTxError::InsufficientFunds);
    println!("{}", ClassicTxError::Unauthorized);
    // function actually returns here because of the ? operator if an error occurs in send_specific_tx
    send_specific_tx("tx details")?;
    Ok(())
}

fn display_modern_tx_error() -> Result<(), ModernTxError> {
    println!("Modern Transaction Errors:");
    println!("{}", ModernTxError::InvalidFormat);
    println!("{}", ModernTxError::InsufficientFunds);
    println!("{}", ModernTxError::Unauthorized);
    // function actually returns here because of the ? operator if an error occurs in send_specific_tx
    send_specific_tx("tx details")?;
    Ok(())
}

fn error_match_showcase(err: &ModernTxError) {
    match err {
        ModernTxError::InvalidFormat => println!("Handle invalid format error"),
        ModernTxError::InsufficientFunds => println!("Handle insufficient funds error"),
        ModernTxError::Unauthorized => println!("Handle unauthorized error"),
        ModernTxError::SpecificTxError(specific_err) => {
            println!("Handle specific transaction error: {}", specific_err)
        }
        ModernTxError::InvalidAddress => println!("Handle invalid address error"),
    }
}

fn main() -> anyhow::Result<()> {
    if let Err(e) = display_classic_tx_error() {
        println!("Error occurred: {}", e);
    }
    if let Err(e) = display_modern_tx_error() {
        println!("Error occurred: {}", e);
    }

    error_match_showcase(&ModernTxError::InsufficientFunds);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Task 1: Extend both error enums with a new variant that wraps a new custom error struct.
    // The error should be a struct containing a number amount and a currency.
    // Implement Display for the struct to show the amount and currency.
    // Also extend the ModernTxError enum using thiserror's #[from] attribute to automatically convert from the struct to the enum variant.
    // Hint Look at the SpecificTxError struct and how it's integrated into both error enums.
    fn task_1(classic_err: ClassicTxError, modern_err: ModernTxError) {
        let classic_display = format!("{}", classic_err);
        assert_eq!(classic_display, "btc: 10");
        let modern_display = format!("{}", modern_err);
        assert_eq!(modern_display, "eth: 20");
    }

    fn task_1_failing_fn_classic() -> Result<(), ClassicTxError> {
        // return your custom error struct wrapped in the ClassicTxError enum here
        todo!();
    }

    fn task_1_failing_fn_modern() -> Result<(), ModernTxError> {
        // return your custom error struct wrapped in the ModernTxError enum here
        todo!();
    }

    #[test]
    fn test_task_1() -> anyhow::Result<()> {
        let classic_err = task_1_failing_fn_classic().unwrap_err();
        let modern_err = task_1_failing_fn_modern().unwrap_err();
        task_1(classic_err, modern_err);
        Ok(())
    }

    // Create your own Error enum named WalletError similar to ModernTxError and add 3 variants to it.
    // Look at the test function below for guidance on what variants to add.
    // Add your Error enum to to ModernTxError as a new variant using the #[from] attribute to automatically convert from WalletError to ModernTxError.
    fn task2(err: ModernTxError, assert_msg: &str) -> anyhow::Result<()> {
        let display = format!("{}", err);
        assert_eq!(display, assert_msg);
        Ok(())
    }

    #[test]
    fn test_task2() -> anyhow::Result<()> {
        let wallet_err_1 = todo!();
        let wallet_err_2 = todo!();
        let wallet_err_3 = todo!();

        task2(wallet_err_1, "Wallet is locked")?;
        task2(wallet_err_2, "Wallet not found")?;
        task2(
            wallet_err_3,
            "Insufficient balance in wallet, expected: 30, actual: 10",
        )?;

        Ok(())
    }

    #[derive(Debug, Clone, PartialEq)]
    struct WalletBalance {
        currency: String,
        balance: u32,
    }

    fn process_transaction(
        mut wallet_balances: Vec<WalletBalance>,
        currency: &str,
        amount: f64,
        address: &str,
    ) -> Result<Vec<WalletBalance>, ModernTxError> {
        // add your logic here, returning appropriate errors, e.g. if insufficient balance, return WalletError::InsufficientBalance
        // also update the wallet_balances vector to reflect the transaction if successful
        Ok(wallet_balances)
    }

    // Task 3:
    #[test]
    fn test_task3() -> anyhow::Result<()> {
        let mut wallet_balances = vec![
            WalletBalance {
                currency: "btc".to_string(),
                balance: 15,
            },
            WalletBalance {
                currency: "eth".to_string(),
                balance: 10,
            },
            WalletBalance {
                currency: "sol".to_string(),
                balance: 5,
            },
        ];
        let mut f = std::fs::File::open("test_data.txt")?;
        let mut contents = String::new();
        use std::io::Read;
        f.read_to_string(&mut contents)?;

        let assertions: Vec<Result<Vec<WalletBalance>, ModernTxError>> = vec![
            // ---- BTC ----
            // 1) send btc 10 -> OK : btc 15 -> 5
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 5,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 10,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 5,
                },
            ]),
            // 2) send btc 5 -> OK : btc 5 -> 0
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 10,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 5,
                },
            ]),
            // 3) send btc 1 -> ❌ insufficient (btc is 0)
            Err(ModernTxError::InsufficientFunds),
            // 4) send btc 2 -> ❌ insufficient
            Err(ModernTxError::InsufficientFunds),
            // 5) send btc 1 to invalid checksum addr -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // 6) send btc 7 to addr with 'O' -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // ---- ETH ----
            // 7) send eth 5 -> OK : eth 10 -> 5
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 5,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 5,
                },
            ]),
            // 8) send eth 2 -> OK : eth 5 -> 3
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 3,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 5,
                },
            ]),
            // 9) send eth 3 -> OK : eth 3 -> 0
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 5,
                },
            ]),
            // 10) invalid length -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // 11) missing 0x -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // 12) invalid hex chars -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // ---- SOL ----
            // 13) send sol 2 -> OK : sol 5 -> 3
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 3,
                },
            ]),
            // 14) send sol 1 -> OK : sol 3 -> 2
            Ok(vec![
                WalletBalance {
                    currency: "btc".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "eth".into(),
                    balance: 0,
                },
                WalletBalance {
                    currency: "sol".into(),
                    balance: 2,
                },
            ]),
            // 15) send sol 4 -> ❌ insufficient funds (sol is 2)
            Err(ModernTxError::InsufficientFunds),
            // 16) invalid base58 -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // 17) too short -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
            // 18) empty -> ❌ invalid address
            Err(ModernTxError::InvalidAddress),
        ];

        for (i, line) in contents.lines().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let currency = parts[1];
            let amount: f64 = parts[2].parse()?;
            let address = parts[5];

            println!(
                "Processing transaction: send {} {} to address {}",
                amount, currency, address
            );

            let res = process_transaction(wallet_balances.clone(), currency, amount, address);

            if let Ok(updated_balances) = &res {
                wallet_balances = updated_balances.clone();
            }

            assert_eq!(res, assertions[i]);
        }
        Ok(())
    }
}
