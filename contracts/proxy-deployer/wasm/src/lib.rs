// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           23
// Async Callback (empty):               1
// Total number of exported functions:  26

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    proxy_deployer
    (
        init => init
        upgrade => upgrade
        contractDeploy => contract_deploy
        contractUpgrade => contract_upgrade
        contractCallByAddress => contract_call_by_address
        changeOwnerAddress => change_owner
        upgradeContractsByTemplate => upgrade_contracts_by_template
        clearOngoingUpgradeOperation => clear_ongoing_upgrade_operation
        addDeployerToBlacklist => add_deployer_to_blacklist
        removeDeployerFromBlacklist => remove_deployer_from_blacklist
        setDefaultGasForSaveOperation => set_default_gas_for_save_operation
        addTemplateAddress => add_template_address
        removeTemplateAddress => remove_template_address
        getDeployerContractsByTemplate => get_deployer_contracts_by_template
        getAllDeployerContracts => get_all_deployer_contracts
        getAllDeployedContractsByTemplate => deployed_contracts_list_by_template
        getContractTemplate => contract_template
        getOngoingUpgradeOperations => ongoing_upgrade_operation
        getDefaultGasForSaveOperation => default_gas_for_save_operation
        getAllDeployers => deployers_list
        getAllTemplates => templates_list
        getAllBlacklistedDeployers => blacklisted_deployers_list
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
