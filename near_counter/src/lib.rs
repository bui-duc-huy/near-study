use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl Counter {
    pub fn get_val(&self) -> i8 {
        self.val
    }

    pub fn set_val(&mut self, new_val: i8) -> i8 {
        self.val = new_val;
        return self.val;
    }
}
