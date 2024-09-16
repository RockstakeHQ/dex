#![no_std]

pub mod storage;
pub mod constants;
pub mod p2p;
pub mod events;
pub mod p2e;
pub mod errors;
pub mod place_bet;
pub mod betslip_nft;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
#[multiversx_sc::contract]
pub trait BetCube:
storage::StorageModule
+ events::EventsModule
+ p2e::P2EModule
+ p2p::P2PModule{
    #[upgrade]
    fn upgrade(&self) {}

    #[init]
    fn init(&self) {
       
    }
}