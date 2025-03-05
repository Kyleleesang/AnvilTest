use alloy::{
     primitives::U256, providers::{Provider, ProviderBuilder}, 
     sol
};
use alloy_contract::*;


sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Example,
    "./tests/ERC20Example.json"
);

#[tokio::test]
async fn test_deployment() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let contract = ERC20Example::deploy(provider).await?;
    println!("Deployed contract at: {}", contract.address());
    Ok(())
}


#[tokio::test]
async fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let contract = ERC20Example::deploy(&provider).await?;
    let name = contract.name().call().await?._0;
    let symbol = contract.symbol().call().await?._0;
    let decimals = contract.decimals().call().await?._0;
    assert_eq!(name, "ERC20Example");
    assert_eq!(symbol, "XYZ");
    assert!(decimals == 18);
    Ok(())
}
#[tokio::test]
async fn test_transfers() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn an anvil instance (local testnet).

    let provider = ProviderBuilder::new().on_anvil_with_wallet();

    //spin up a few test accounts
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the ERC20 contract.
    let contract = ERC20Example::deploy(&provider).await?;
    println!("Deployed contract at: {}", contract.address());

    // === Test token transfer ===

    // Register the balances of Alice and Bob before the transfer.
    let alice_before_balance = contract.balanceOf(alice).call().await?._0;
    let bob_before_balance = contract.balanceOf(bob).call().await?._0;

    // Transfer and wait for inclusion.
    let amount = U256::from(100);
    let tx_hash = contract.transfer(bob, amount).send().await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    // Register the balances of Alice and Bob after the transfer.
    let alice_after_balance = contract.balanceOf(alice).call().await?._0;
    let bob_after_balance = contract.balanceOf(bob).call().await?._0;

    // Check the balances of Alice and Bob after the transfer.
    assert_eq!(alice_before_balance - alice_after_balance, amount);
    assert_eq!(bob_after_balance - bob_before_balance, amount);
    Ok(())
}

#[tokio::test]
async fn test_approvals() -> Result<(), Box<dyn std::error::Error>> {
    // Spawn an anvil instance (local testnet).
    let provider = ProviderBuilder::new().on_anvil_with_wallet();

    //spin up a few test accounts
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the ERC20 contract.
    let contract = ERC20Example::deploy(&provider).await?;
    println!("Deployed contract at: {}", contract.address());

    // === Test token approval ===

    // Register the allowance of Alice to Bob before the approval.
    let alice_before_allowance = contract.allowance(alice, bob).call().await?._0;

    // Approve and wait for inclusion.
    let amount = U256::from(100);
    let tx_hash = contract.approve(bob, amount).send().await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    // Register the allowance of Alice to Bob after the approval.
    let alice_after_allowance = contract.allowance(alice, bob).call().await?._0;

    // Check the allowance of Alice to Bob after the approval.
    assert_eq!(alice_after_allowance - alice_before_allowance, amount);
    Ok(())
}



