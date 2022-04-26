require('dotenv').config()

const NETWORK = "https://bombay-lcd.terra.dev";
const CHAIN_ID = "bombay-12";
const WALLET_SEEDS = process.env.WALLET_SEEDS;
const MARKET_PLACE_ADDRESS = "terra1d0n6e9k666xurqej0j0tl0znkwzrnqr8sayx37" // testnet
const TEST_NFT_ADDR = "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju"
const TEST_TOKEN_ID = "2"
const TEST_NFT_OWNER = "terra13h4nw2y0lkpz8xs55fxa9vaugrnjkyu4czvr8u"

export default {
  NETWORK,
  CHAIN_ID,
  WALLET_SEEDS,
  TEST_NFT_ADDR,
  TEST_TOKEN_ID,
  TEST_NFT_OWNER
};
