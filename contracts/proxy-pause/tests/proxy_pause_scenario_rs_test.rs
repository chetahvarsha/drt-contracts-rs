use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/proxy-pause");
    blockchain.register_contract(
        "drtsc:output/proxy-pause.drtsc.json",
        proxy_pause::ContractBuilder,
    );

    blockchain.register_contract(
        "drtsc:../check-pause/output/check-pause.drtsc.json",
        check_pause::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn pause_and_unpause_rs() {
    world().run("scenarios/pause-and-unpause.scen.json");
}
