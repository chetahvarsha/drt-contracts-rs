// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dharitri_wrewa_swap_sc
    (
        init => init
        wrapRewa => wrap_rewa
        unwrapRewa => unwrap_rewa
        getLockedRewaBalance => get_locked_rewa_balance
        getWrappedRewaTokenId => wrapped_rewa_token_id
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
