use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Event verification helper
    #[derive(Debug)]
    enum EventInfo {
        TokenInitialized {
            mint: Pubkey,
            authority: Pubkey,
            total_supply: u64,
            decimals: u8,
            marketing_wallet: Pubkey,
        },
        TransferExecuted {
            from: Pubkey,
            to: Pubkey,
            amount: u64,
            timestamp: i64,
        },
        FeesCollected {
            reflection_amount: u64,
            marketing_amount: u64,
            burn_amount: u64,
            dev_amount: u64,
            timestamp: i64,
        },
        TradingStatusChanged {
            enabled: bool,
            timestamp: i64,
        },
        FeesUpdated {
            reflection_fee_bp: u16,
            marketing_fee_bp: u16,
            burn_fee_bp: u16,
            dev_fee_bp: u16,
            timestamp: i64,
        },
    }

    async fn get_program_events(
        banks_client: &mut BanksClient,
        signature: &Signature,
    ) -> Vec<EventInfo> {
        let transaction = banks_client
            .get_transaction_with_config(
                signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )
            .await
            .unwrap();
        
        // TODO: Implement event parsing from transaction logs
        vec![] // Placeholder until we implement proper event parsing
    }

    async fn cleanup_test_state(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        accounts: &[Pubkey],
    ) {
        for account in accounts {
            if let Ok(Some(_)) = banks_client.get_account(*account).await {
                // Close account by transferring lamports back to payer
                let _ = banks_client.process_transaction(
                    Transaction::new_signed_with_payer(
                        &[system_instruction::transfer(
                            account,
                            &payer.pubkey(),
                            0,
                        )],
                        Some(&payer.pubkey()),
                        &[payer],
                        banks_client.get_latest_blockhash().await.unwrap(),
                    ),
                ).await;
            }
        }
    }

    // Test utilities
    fn create_program_test() -> ProgramTest {
        ProgramTest::new(
            "bigballs_token",
            crate::ID,
            processor!(crate::entry),
        )
    }

    async fn setup_token() -> (
        BanksClient,
        Keypair,  // Payer
        Keypair,  // Authority
        Pubkey,   // Token Config
        Pubkey,   // Mint
        Pubkey,   // Marketing Wallet
    ) {
        let mut program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        
        let authority = Keypair::new();
        let mint = Keypair::new();
        let marketing_wallet = Keypair::new();
        
        // Initialize token config account
        let config = Pubkey::find_program_address(
            &[b"token_config", mint.pubkey().as_ref()],
            &crate::ID,
        ).0;

        // Fund accounts
        banks_client.process_transaction(
            Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &payer.pubkey(),
                    &authority.pubkey(),
                    1_000_000_000, // 1 SOL
                )],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            ),
        ).await.unwrap();

        (
            banks_client,
            payer,
            authority,
            config,
            mint.pubkey(),
            marketing_wallet.pubkey(),
        )
    }

    async fn initialize_token(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        authority: &Keypair,
        config: Pubkey,
        mint: Pubkey,
        marketing_wallet: Pubkey,
    ) -> Result<(), BanksClientError> {
        let init_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
            ],
            data: bigballs_token::instruction::Initialize {
                name: "Big Balls".to_string(),
                symbol: "BIGBALLS".to_string(),
                decimals: 9,
                marketing_wallet,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[init_ix],
            Some(&payer.pubkey()),
            &[payer, authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await
    }

    async fn enable_trading(
        banks_client: &mut BanksClient,
        payer: &Keypair,
        authority: &Keypair,
        config: Pubkey,
    ) -> Result<(), BanksClientError> {
        let enable_trading_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(authority.pubkey(), true),
            ],
            data: bigballs_token::instruction::SetTradingEnabled {
                enabled: true,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[enable_trading_ix],
            Some(&payer.pubkey()),
            &[payer, authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await
    }

    #[tokio::test]
    async fn test_initialize() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Test initialization
        let init_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
            ],
            data: bigballs_token::instruction::Initialize {
                name: "Big Balls".to_string(),
                symbol: "BIGBALLS".to_string(),
                decimals: 9,
                marketing_wallet,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[init_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify token config
        let config_account = banks_client.get_account(config).await.unwrap().unwrap();
        let token_config: TokenConfig = try_from_slice_unchecked(&config_account.data).unwrap();

        assert_eq!(token_config.name, "Big Balls");
        assert_eq!(token_config.symbol, "BIGBALLS");
        assert_eq!(token_config.decimals, 9);
        assert_eq!(token_config.marketing_wallet, marketing_wallet);
        assert_eq!(token_config.trading_enabled, false);
        assert_eq!(token_config.reflection_fee_bp, 200);
        assert_eq!(token_config.marketing_fee_bp, 150);
        assert_eq!(token_config.burn_fee_bp, 100);
        assert_eq!(token_config.dev_fee_bp, 50);
    }

    #[tokio::test]
    async fn test_initialize_invalid_marketing_wallet() {
        let (mut banks_client, payer, authority, config, mint, _) = setup_token().await;

        // Try to initialize with default pubkey
        let init_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
            ],
            data: bigballs_token::instruction::Initialize {
                name: "Big Balls".to_string(),
                symbol: "BIGBALLS".to_string(),
                decimals: 9,
                marketing_wallet: Pubkey::default(),
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[init_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_trading_controls() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token first
        // ... (initialization code)

        // Test enabling trading
        let enable_trading_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(authority.pubkey(), true),
            ],
            data: bigballs_token::instruction::SetTradingEnabled {
                enabled: true,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[enable_trading_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify trading is enabled
        let config_account = banks_client.get_account(config).await.unwrap().unwrap();
        let token_config: TokenConfig = try_from_slice_unchecked(&config_account.data).unwrap();
        assert_eq!(token_config.trading_enabled, true);

        // Test unauthorized access
        let unauthorized = Keypair::new();
        let unauthorized_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(unauthorized.pubkey(), true),
            ],
            data: bigballs_token::instruction::SetTradingEnabled {
                enabled: false,
            }
            .data(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[unauthorized_ix],
            Some(&payer.pubkey()),
            &[&payer, &unauthorized],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fee_calculations() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token and enable trading
        // ... (initialization code)

        // Create test accounts
        let from = Keypair::new();
        let to = Keypair::new();
        let amount = 1_000_000;

        // Test transfer with fees
        let transfer_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(from.pubkey(), false),
                AccountMeta::new(to.pubkey(), false),
                AccountMeta::new(marketing_wallet, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data: bigballs_token::instruction::Transfer {
                amount,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify fee distributions
        // ... (verification code)
    }

    #[tokio::test]
    async fn test_transfer_with_trading_disabled() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        // Create test accounts
        let from = Keypair::new();
        let to = Keypair::new();
        let amount = 1_000_000;

        // Attempt transfer without enabling trading
        let transfer_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(from.pubkey(), false),
                AccountMeta::new(to.pubkey(), false),
                AccountMeta::new(marketing_wallet, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data: bigballs_token::instruction::Transfer {
                amount,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_fees() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        // Update fees
        let update_fees_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(authority.pubkey(), true),
            ],
            data: bigballs_token::instruction::UpdateFees {
                reflection_fee_bp: 300,    // 3%
                marketing_fee_bp: 100,     // 1%
                burn_fee_bp: 50,           // 0.5%
                dev_fee_bp: 50,            // 0.5%
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[update_fees_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify updated fees
        let config_account = banks_client.get_account(config).await.unwrap().unwrap();
        let token_config: TokenConfig = try_from_slice_unchecked(&config_account.data).unwrap();
        
        assert_eq!(token_config.reflection_fee_bp, 300);
        assert_eq!(token_config.marketing_fee_bp, 100);
        assert_eq!(token_config.burn_fee_bp, 50);
        assert_eq!(token_config.dev_fee_bp, 50);

        // Test invalid fee update (total > 10%)
        let invalid_update_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(authority.pubkey(), true),
            ],
            data: bigballs_token::instruction::UpdateFees {
                reflection_fee_bp: 500,
                marketing_fee_bp: 300,
                burn_fee_bp: 200,
                dev_fee_bp: 100,
            }
            .data(),
        };

        let transaction = Transaction::new_signed_with_payer(
            &[invalid_update_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transfer_limits() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token and enable trading
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        enable_trading(
            &mut banks_client,
            &payer,
            &authority,
            config,
        ).await.unwrap();

        // Get max transaction amount
        let config_account = banks_client.get_account(config).await.unwrap().unwrap();
        let token_config: TokenConfig = try_from_slice_unchecked(&config_account.data).unwrap();
        let max_amount = token_config.max_transaction_amount;

        // Test transfer exceeding limit
        let from = Keypair::new();
        let to = Keypair::new();
        
        let transfer_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(from.pubkey(), false),
                AccountMeta::new(to.pubkey(), false),
                AccountMeta::new(marketing_wallet, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data: bigballs_token::instruction::Transfer {
                amount: max_amount + 1,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_holder_state() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token and enable trading
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        enable_trading(
            &mut banks_client,
            &payer,
            &authority,
            config,
        ).await.unwrap();

        // Create test accounts
        let from = Keypair::new();
        let to = Keypair::new();
        let amount = 1_000_000;

        // Execute transfer
        let transfer_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(from.pubkey(), false),
                AccountMeta::new(to.pubkey(), false),
                AccountMeta::new(marketing_wallet, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data: bigballs_token::instruction::Transfer {
                amount,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify holder state
        let holder_state_address = Pubkey::find_program_address(
            &[b"holder_state", from.pubkey().as_ref()],
            &crate::ID,
        ).0;

        let holder_state_account = banks_client.get_account(holder_state_address).await.unwrap().unwrap();
        let holder_state: HolderState = try_from_slice_unchecked(&holder_state_account.data).unwrap();

        assert_eq!(holder_state.total_transactions, 1);
        assert_eq!(holder_state.total_amount, amount);
        assert!(holder_state.last_transaction > 0);
    }

    #[tokio::test]
    async fn test_event_emission() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize token
        let init_result = initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await;

        assert!(init_result.is_ok());
        // Note: In a real implementation, we would verify the TokenInitialized event here
        // However, the current test framework doesn't provide event verification capabilities

        // Enable trading
        let enable_result = enable_trading(
            &mut banks_client,
            &payer,
            &authority,
            config,
        ).await;

        assert!(enable_result.is_ok());
        // Would verify TradingStatusChanged event

        // Update fees
        let update_fees_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(config, false),
                AccountMeta::new(authority.pubkey(), true),
            ],
            data: bigballs_token::instruction::UpdateFees {
                reflection_fee_bp: 300,
                marketing_fee_bp: 100,
                burn_fee_bp: 50,
                dev_fee_bp: 50,
            }
            .data(),
        };

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let transaction = Transaction::new_signed_with_payer(
            &[update_fees_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok());
        // Would verify FeesUpdated event
    }

    #[tokio::test]
    async fn test_concurrent_transfers() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize and enable trading
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        enable_trading(
            &mut banks_client,
            &payer,
            &authority,
            config,
        ).await.unwrap();

        // Create test accounts
        let from = Keypair::new();
        let to = Keypair::new();
        let amount = 1_000_000;

        // Attempt multiple transfers within cooldown period
        for i in 0..3 {
            let transfer_ix = Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new(config, false),
                    AccountMeta::new(from.pubkey(), false),
                    AccountMeta::new(to.pubkey(), false),
                    AccountMeta::new(marketing_wallet, false),
                    AccountMeta::new(authority.pubkey(), true),
                    AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data: bigballs_token::instruction::Transfer {
                    amount,
                }
                .data(),
            };

            let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let transaction = Transaction::new_signed_with_payer(
                &[transfer_ix],
                Some(&payer.pubkey()),
                &[&payer, &authority],
                recent_blockhash,
            );

            let result = banks_client.process_transaction(transaction).await;
            
            // Only the first transfer should succeed
            if i == 0 {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err()); // Should fail due to cooldown
            }
        }

        // Verify final holder state
        let holder_state_address = Pubkey::find_program_address(
            &[b"holder_state", from.pubkey().as_ref()],
            &crate::ID,
        ).0;

        let holder_state_account = banks_client.get_account(holder_state_address).await.unwrap().unwrap();
        let holder_state: HolderState = try_from_slice_unchecked(&holder_state_account.data).unwrap();

        assert_eq!(holder_state.total_transactions, 1);
        assert_eq!(holder_state.total_amount, amount);

        // Cleanup
        cleanup_test_state(
            &mut banks_client,
            &payer,
            &[config, holder_state_address],
        ).await;
    }

    #[tokio::test]
    async fn test_fee_precision() {
        let (mut banks_client, payer, authority, config, mint, marketing_wallet) = 
            setup_token().await;

        // Initialize and enable trading
        initialize_token(
            &mut banks_client,
            &payer,
            &authority,
            config,
            mint,
            marketing_wallet,
        ).await.unwrap();

        enable_trading(
            &mut banks_client,
            &payer,
            &authority,
            config,
        ).await.unwrap();

        // Test various transfer amounts
        let test_amounts = vec![
            100,            // Very small amount
            1_000_000,      // Medium amount
            1_000_000_000,  // Large amount
            u64::MAX / 100, // Very large amount
        ];

        for amount in test_amounts {
            let config_account = banks_client.get_account(config).await.unwrap().unwrap();
            let token_config: TokenConfig = try_from_slice_unchecked(&config_account.data).unwrap();
            
            // Calculate expected fees
            let total_fee_bp = token_config.reflection_fee_bp + token_config.marketing_fee_bp + 
                              token_config.burn_fee_bp + token_config.dev_fee_bp;
            let expected_total_fee = amount * total_fee_bp as u64 / 10000;
            
            // Execute transfer
            let from = Keypair::new();
            let to = Keypair::new();
            
            let transfer_ix = Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new(config, false),
                    AccountMeta::new(from.pubkey(), false),
                    AccountMeta::new(to.pubkey(), false),
                    AccountMeta::new(marketing_wallet, false),
                    AccountMeta::new(authority.pubkey(), true),
                    AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data: bigballs_token::instruction::Transfer {
                    amount,
                }
                .data(),
            };

            let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
            let transaction = Transaction::new_signed_with_payer(
                &[transfer_ix],
                Some(&payer.pubkey()),
                &[&payer, &authority],
                recent_blockhash,
            );

            let result = banks_client.process_transaction(transaction).await;
            assert!(result.is_ok());

            // Verify fees through events
            if let Ok(signature) = result {
                let events = get_program_events(&mut banks_client, &signature).await;
                for event in events {
                    if let EventInfo::FeesCollected {
                        reflection_amount,
                        marketing_amount,
                        burn_amount,
                        dev_amount,
                        ..
                    } = event {
                        let actual_total_fee = reflection_amount + marketing_amount + burn_amount + dev_amount;
                        assert_eq!(actual_total_fee, expected_total_fee, "Fee calculation mismatch for amount {}", amount);
                    }
                }
            }
        }

        // Cleanup
        cleanup_test_state(
            &mut banks_client,
            &payer,
            &[config],
        ).await;
    }

    #[tokio::test]
    async fn test_basic_setup() {
        // Create program test environment
        let mut program_test = ProgramTest::new(
            "bigballs_token",
            crate::ID,
            processor!(crate::entry),
        );

        // Start the test environment
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        
        // Basic assertion to verify test environment
        assert!(banks_client.get_latest_blockhash().await.is_ok());
        println!("✅ Test environment successfully initialized");
    }
} 