// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct ProxyDeployerProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ProxyDeployerProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ProxyDeployerProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ProxyDeployerProxyMethods { wrapped_tx: tx }
    }
}

pub struct ProxyDeployerProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> ProxyDeployerProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<u64>,
    >(
        self,
        default_gas_for_save: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&default_gas_for_save)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ProxyDeployerProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ProxyDeployerProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn contract_deploy<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        template_address: Arg0,
        args: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("contractDeploy")
            .argument(&template_address)
            .argument(&args)
            .original_result()
    }

    pub fn contract_upgrade<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        contract_address: Arg0,
        args: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("contractUpgrade")
            .argument(&contract_address)
            .argument(&args)
            .original_result()
    }

    pub fn contract_call_by_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        contract_address: Arg0,
        function_name: Arg1,
        args: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("contractCallByAddress")
            .argument(&contract_address)
            .argument(&function_name)
            .argument(&args)
            .original_result()
    }

    /// Use this endpoint to transfer the ownership 
    /// This is needed to properly update the stored data 
    pub fn change_owner<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        contract_address: Arg0,
        new_owner: Arg1,
        opt_orig_owner: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeOwnerAddress")
            .argument(&contract_address)
            .argument(&new_owner)
            .argument(&opt_orig_owner)
            .original_result()
    }

    /// Allows the owner to bulk upgrade all the contracts by starting an ongoing upgrade operation 
    /// The first time when the endpoint is called, the optional arguments are required 
    /// After that the endpoint needs to be called without the optional args, until the upgrade operation is finished 
    pub fn upgrade_contracts_by_template<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        gas_per_action: Arg0,
        opt_template_address: Arg1,
        args: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("upgradeContractsByTemplate")
            .argument(&gas_per_action)
            .argument(&opt_template_address)
            .argument(&args)
            .original_result()
    }

    pub fn clear_ongoing_upgrade_operation(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clearOngoingUpgradeOperation")
            .original_result()
    }

    pub fn add_deployer_to_blacklist<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        blacklisted_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addDeployerToBlacklist")
            .argument(&blacklisted_address)
            .original_result()
    }

    pub fn remove_deployer_from_blacklist<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeDeployerFromBlacklist")
            .argument(&address)
            .original_result()
    }

    pub fn set_default_gas_for_save_operation<
        Arg0: ProxyArg<u64>,
    >(
        self,
        default_gas_for_save_operation: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setDefaultGasForSaveOperation")
            .argument(&default_gas_for_save_operation)
            .original_result()
    }

    pub fn add_template_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        template_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addTemplateAddress")
            .argument(&template_address)
            .original_result()
    }

    pub fn remove_template_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        template_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeTemplateAddress")
            .argument(&template_address)
            .original_result()
    }

    pub fn get_deployer_contracts_by_template<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
        template_address: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDeployerContractsByTemplate")
            .argument(&user)
            .argument(&template_address)
            .original_result()
    }

    pub fn get_all_deployer_contracts<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllDeployerContracts")
            .argument(&user)
            .original_result()
    }

    pub fn deployed_contracts_list_by_template<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        template_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllDeployedContractsByTemplate")
            .argument(&template_address)
            .original_result()
    }

    pub fn contract_template<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        contract_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractTemplate")
            .argument(&contract_address)
            .original_result()
    }

    pub fn ongoing_upgrade_operation(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OngoingUpgradeOperation<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getOngoingUpgradeOperations")
            .original_result()
    }

    pub fn default_gas_for_save_operation(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDefaultGasForSaveOperation")
            .original_result()
    }

    pub fn deployers_list(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllDeployers")
            .original_result()
    }

    pub fn templates_list(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllTemplates")
            .original_result()
    }

    pub fn blacklisted_deployers_list(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllBlacklistedDeployers")
            .original_result()
    }

    pub fn pause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pause")
            .original_result()
    }

    pub fn unpause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpause")
            .original_result()
    }

    pub fn paused_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isPaused")
            .original_result()
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OngoingUpgradeOperation<Api>
where
    Api: ManagedTypeApi,
{
    pub template_address: ManagedAddress<Api>,
    pub arguments: ManagedArgBuffer<Api>,
    pub contracts_remaining: ManagedVec<Api, ManagedAddress<Api>>,
    pub processed_contracts: ManagedVec<Api, ManagedAddress<Api>>,
}

#[type_abi]
#[derive(TopEncode)]
pub struct DeployContractEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub template: ManagedAddress<Api>,
    pub deployed_address: ManagedAddress<Api>,
    pub arguments: ManagedVec<Api, ManagedBuffer<Api>>,
}

#[type_abi]
#[derive(TopEncode)]
pub struct UpgradeContractEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub template: ManagedAddress<Api>,
    pub upgraded_address: ManagedAddress<Api>,
    pub arguments: ManagedVec<Api, ManagedBuffer<Api>>,
}

#[type_abi]
#[derive(TopEncode)]
pub struct ContractCallEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub contract_address: ManagedAddress<Api>,
    pub function: ManagedBuffer<Api>,
    pub arguments: ManagedVec<Api, ManagedBuffer<Api>>,
}

#[type_abi]
#[derive(TopEncode)]
pub struct ChangeOwnerEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub contract_address: ManagedAddress<Api>,
    pub old_owner: ManagedAddress<Api>,
    pub new_owner: ManagedAddress<Api>,
}
