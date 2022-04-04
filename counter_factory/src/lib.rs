use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

near_sdk::setup_alloc!();

const CODE: &[u8] = include_bytes!("../../near_counter/target/wasm32-unknown-unknown/release/near_counter.wasm");

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct CounterFactory {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl CounterFactory {
    #[payable]
    pub fn new_counter(&mut self, name: AccountId) -> Promise {
        let account_id = format!("{}.{}", name, env::current_account_id());
        Promise::new(account_id).create_account().deploy_contract(CODE.to_vec()).transfer(30_000_000_000_000_000_000_000_000)
    }
}
