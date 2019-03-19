# sample-token

This sample is a ERC20-like token written in Rust for [icetea](https://github.com/TradaTech/icetea) platform.

### Note
* Install rust first (1.34.0-nightly or later)
* Do not modify `name` in [package] section in Cargo.toml

### Command
```bash
$ npm i
$ npm run build
```

### Directory
```
project
└───icetea_wasm <-- library
└───src
│   │   lib.rs  <-- your source here
└───pkg
    │   hello_world_bg.wasm <-- your wasm file here
```

### Source explain
#### External function
```rust
#[wasm_bindgen]
extern {
  fn log(text: &str); // log a message during runtime
  fn get_sender() -> String; // get sender address
  fn get_address() -> String; // get address of this contract
  fn now() -> i32; // get block timestamp
  fn get_block_hash() -> String; // get block hash
  fn get_block_number() -> i32; // get block number
  fn get_msg_value() -> i32; // get message value (if payable)
  fn emit_event(name: &str, data: &Value, indexes: &Value); // emit event
  fn load(key: &str) -> Value; // load value from state store
  fn save(key: &str, value: &Value); // create or update value in state store
  fn read_contract(address: &str, method: &str, params: Array) -> Value; // read method from remote contract
  fn write_contract(address: &str, method: &str, params: Array) -> Value; // write method from remote contract
}
```

#### Main function
* All method you call is proxied by this function

```rust
#[wasm_bindgen]
pub fn main(operation: &str, value: &Value) -> Value { // do not modify this api
  log(&format!("[RUST] Hello {}, you call method {}", get_sender(), operation));
  let params = value.as_array(); // get parameters as array

  match operation {
    "__on_deployed" => { // built-in message name when contract is deployed
      let sender = get_sender();
      _mint(&sender, 1000000000);
    },
    "balance_of" => { // custome message for this contract
      let owner = params[0].as_string().unwrap(); // cast the param to desired type
      return balance_of(&owner).to_value(); // your function
    },
    ...
  }
  ...

  pub fn balance_of(owner: &str) -> u128 {
    let balance = load!(u128, &get_key!(BALANCE_KEY, owner)); // macro function
    return balance;
  }
```

### API
#### Built-in message
* __on_deployed: contract is deployed
* __on_received: contract is recieved a value

#### Macro function
*Note: macro functions require external function*

* required!(condition, message) e.g: `require!(true, "it is always true")`
* load!(type, key) e.g: `load!(u128, "key")`, support u128, u64, u32, bool, String. Require `load` function
* save!(key, value) e.g: `load!("key", 123)`, support u128, u64, u32, bool, String. Require `save` function
* get_key!(...) e.g: `get_key!("key", "subkey")`
* array!(...) e.g: `array!("key".to_value(), "subkey".to_value())`

#### Type conversion
* as_u128, as_u64, as_u32, as_array: convert from Value to Rust type
* to_value: convert from Rust type to Value

