use crate::address_info::*;

#[dharitri_sc::module]
pub trait EventsModule {
    #[event("new_claim")]
    fn new_claim_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);

    #[event("new_claim_and_repair")]
    fn new_claim_and_repair_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);

    #[event("new_update_state")]
    fn new_update_state_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);
}
