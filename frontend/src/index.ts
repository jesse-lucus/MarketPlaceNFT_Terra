import {
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  Coins,
} from "@terra-money/terra.js";
import info from "./constant";
import fetch from 'isomorphic-fetch';

(async () => {
	try {
		// Create LCDClient for Bombay-12 TestNet
		const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json();
		const gasPricesCoins = new Coins(gasPrices);
		const terra: LCDClient = new LCDClient({
			URL: info.NETWORK,
			chainID: info.CHAIN_ID,
			gasPrices: gasPricesCoins,
			gasAdjustment: "1.5",
		});
		// Get deployer wallet
		const wallet = terra.wallet(new MnemonicKey({ mnemonic: info.WALLET_SEEDS }));
		console.log("Wallet: ", wallet.key.accAddress);

		let timeStamp = Math.floor(Date.now() / 1000 ) + 3600 * 24 * 7
		console.log('timeStamp', timeStamp)
		const expire_at = {
				"at_time": timeStamp.toString(),
				// "never": {}
		},
		price = {
			"amount": "200", //0.000001 Luna
			"info": {
				"native_token": {"denom": "uluna"}
			}
		}

		const setPausedMsg = {set_paused: {paused: false}}
		const createOrderMsg = { create_order: { token_id: "2", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju", price, expire_at: timeStamp } }
		// const cancelOrderMsg = { cancel_order: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }
		// const exeOrderMsg = { execute_order: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }

		// const createBidMsg = { create_bid: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju", price, expire_at } }
		// const cancelBidMsg = { cancel_bid: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }
		const increase = new MsgExecuteContract(
			wallet.key.accAddress, // sender
			info.MARKET_PLACE_ADDRESS,
			createOrderMsg
		)
		const increaseTx = await wallet.createAndSignTx({
			msgs: [increase]
		})
		console.log("increaseTx?", increaseTx && increaseTx?.body.messages)
		if (increaseTx) {
			const increaseTxTxResult = await terra.tx.broadcast(increaseTx);
			console.log("increaseTxTxResult?", increaseTxTxResult)
		}
	} catch (e) {
		console.log(e)
	}
})();

