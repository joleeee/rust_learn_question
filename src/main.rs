use std::sync::Arc;

use ethers::{prelude::*, abi::Uint};

#[tokio::main]
fn main() {
    let ws = Ws::connect("").await.unwrap();
    let wsProvider = Provider::new(ws);
    check_balance(wsProvider,vec![""]);
    let httpProvider = Provider::<Http>::try_from("").unwrap();
    check_balance(httpProvider, vec![""])

}

async fn check_balance(
    client: Arc<Provider<impl JsonRpcClient>>,
    addresses: Vec<Address>,
) -> Result<Vec<Uint>,ProviderError> {
    let mut multicall = Multicall::<Provider<_>>::new(client, None).await.unwrap();

    for address in addresses {
        multicall.eth_balance_of(address);
    }
    let data = multicall.call_raw().await?;
    Ok(data
        .into_iter()
        .map(|token| token.into_uint().unwrap())
        .collect::<Vec<_>>())
}