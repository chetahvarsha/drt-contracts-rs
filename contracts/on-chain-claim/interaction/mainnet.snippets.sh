WALLET="${PWD}/wallet.pem"
PROJECT="${PWD}"
PROXY=https://gateway.dharitri.com
CHAINID=D

DEPLOY_GAS="30000000"
SFT_IDENTIFIER=0x585354525245504149522d653162363733 #XSTRREPAIR-e1b673

CONTRACT_ADDRESS="drt1qqqqqqqqqqqqqpgqycdpxfmvxqm3cxylsyff3tkw6yhc6gwga6mq2tmedd"

deploy() {
    drtpy --verbose contract deploy \
          --bytecode="output/on-chain-claim.drtsc.json" \
          --arguments ${SFT_IDENTIFIER} \
          --pem=${WALLET} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} \
          --chain=${CHAINID} \
          --recall-nonce \
          --send \
          --outfile="deploy-mainnet.interaction.json" || return

    TRANSACTION=$(drtpy data parse --file="deploy-mainnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="deploy-mainnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-mainnet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-mainnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    drtpy --verbose contract upgrade ${CONTRACT_ADDRESS} \
          --bytecode="output/on-chain-claim.drtsc.json" \
          --pem=${WALLET} \
          --gas-limit=${DEPLOY_GAS} \
          --proxy=${PROXY} \
          --chain=${CHAINID} \
          --recall-nonce \
          --send \
          --outfile="upgrade-mainnet.interaction.json" || return

    TRANSACTION=$(drtpy data parse --file="upgrade-mainnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="upgrade-mainnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-mainnet --value=${ADDRESS}
    drtpy data store --key=upgradeTransaction-mainnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

addAdmin() {
     ADMIN=0x9ff3241fc3c4ffa009df088fdd3f33e4b10b24cfb9a28e525bc9c46e47b3e0e2

     drtpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
         --pem=${WALLET} \
         --gas-limit=10000000 \
         --proxy=${PROXY} --chain=${CHAIN_ID} \
         --function="addAdmin" \
         --arguments $ADMIN \
         --send || return
 }
