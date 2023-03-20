PROJECT="${PWD}"

COLLECTION_ID="XCARD-6c24da"
COLLECTION_ID_HEX="0x$(echo -n ${COLLECTION_ID} | xxd -p -u | tr -d '\n')"

TOKEN_ID="ESTAR-ccc274"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

PEM_FILE="/home/edi/Desktop/my-wallet/my_wallet.pem"
PROXY=https://devnet-gateway.multiversx.com
CHAINID=D
ADDRESS=erd1qqqqqqqqqqqqqpgqfq7py50tnxg6042yg6wanshfspf204ptxszqnzdggr
MY_ADDRESS="erd1a6p39rlsn2lm20adqe5tmzy543luwqx4dywzflr2dmtwdf75xszqdw9454"


deploy() {
  mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/deploy.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX $TOKEN_ID_HEX || return
}

updateContract() {
  mxpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=60000000 --send --outfile="${PROJECT}/interactions/logs/update.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments $COLLECTION_ID_HEX $TOKEN_ID_HEX
}

stake() {
  method_name="0x$(echo -n 'stake' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${MY_ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=60000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTNFTTransfer" \
    --arguments $COLLECTION_ID_HEX 1 1 $ADDRESS $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/stake.json"
}

unStake() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="unStake" \
    --arguments $TOKEN_ID_HEX 1 1 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

unBond() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="unBond" \
    --arguments $TOKEN_ID_HEX 1 15 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

fundSystem() {
  method_name="0x$(echo -n 'fundSystem' | xxd -p -u | tr -d '\n')"
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $TOKEN_XAPE_ID_HEX 281000000000000000000000 $method_name \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

withdrawFunds() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="withdrawFunds" \
    --arguments 98142999999999999999124 \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

claimRewards() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="claimRewards" \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

togglePause() {
  mxpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=30000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="togglePause" \
    --send \
    --outfile="${PROJECT}/interactions/logs/unstake.json"
}

getToken() {
  mxpy --verbose contract query ${ADDRESS} --function="getToken" \
    --proxy=${PROXY}
}

getPause() {
  mxpy --verbose contract query ${ADDRESS} --function="getPause" \
    --proxy=${PROXY}
}

getSftsAllowed() {
  mxpy --verbose contract query ${ADDRESS} --function="getSftsAllowed" \
    --proxy=${PROXY}
}

getSftsStaked() {
  mxpy --verbose contract query ${ADDRESS} --function="getSftsStaked" --arguments $MY_ADDRESS \
    --proxy=${PROXY}
}

getSftStakedAmount() {
  mxpy --verbose contract query ${ADDRESS} --function="getSftStakedAmount" --arguments $MY_ADDRESS 1 \
    --proxy=${PROXY}
}

getSftStakedAt() {
  mxpy --verbose contract query ${ADDRESS} --function="getSftStakedAt" --arguments $MY_ADDRESS 1 \
    --proxy=${PROXY}
}

getSftReward() {
  mxpy --verbose contract query ${ADDRESS} --function="getSftReward" --arguments 1 \
    --proxy=${PROXY}
}

getUsersStaked() {
  mxpy --verbose contract query ${ADDRESS} --function="getUsersStaked" \
    --proxy=${PROXY}
}

getTokenPayment() {
  mxpy --verbose contract query ${ADDRESS} --function="getTokenPayment" \
    --proxy=${PROXY}
}

getTokenAmount() {
  mxpy --verbose contract query ${ADDRESS} --function="getTokenAmount" \
    --proxy=${PROXY}
}