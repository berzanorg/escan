# escan
Ξ Escan is a Rust API wrapper for Etherscan and its derivatives

> A production ready Rust Crate to work with Ethereum blockchain data

```rust
  // Create your client
  let client = Client::new(API_TOKEN, Chain::Ethereum); // Or Chain::BinanceSmartChain
  // Get balance of an address
  let bal = client.balance("0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8", Tag::Latest).await?;
  // Print it
  println!("{} ETH", bal);
  // Prints:
  // "1.927 ETH"
```

### Supported Chains
- Ethereum
- Binance Smart Chain
> New chain supports are coming

### Usage
add Escan to dependencies list in Cargo.toml
```toml
  # ...
  [dependencies]
  # other deps
  escan = "0.1.0"
  # other deps
  # also don't forget to add Tokio runtime
  tokio = { version = "1.19.2", features = ["full"] }
```

### API Endpoints
- Accounts 🗸
- Contracts ✖️
- Transactions ✖️
- Blocks ✖️
- Logs ✖️
- Geth/Parity Proxy ✖️
- Tokens ✖️
- Gas Tracker ✖️
- Stats ✖️
> 🗸 Supported
> 
> ✖️ Not supported

### Development
Pull requests are welcome

If you want to contribute just let me know ;)

> Made with ❤️ by Berzan
