import {
  Coin,
  isTxError,
  LCDClient,
  MnemonicKey,
  Msg,
  MsgInstantiateContract,
  MsgExecuteContract,
  MsgStoreCode,
  Fee,
  Wallet,
  WasmAPI,
} from "@terra-money/terra.js";
import info from "./constant";
import * as path from "path";
import * as fs from "fs";

(async () => {
	try {
		// Create LCDClient for Bombay-12 TestNet
		const terra: LCDClient = new LCDClient({
			URL: info.NETWORK,
			chainID: info.CHAIN_ID,
			gasPrices: { uluna: 0.015 },
			gasAdjustment: 1.4	
		});

		// Get deployer wallet
		const wallet = terra.wallet(new MnemonicKey({ mnemonic: info.WALLET_SEEDS }));
		console.log("Wallet: ", wallet.key.accAddress);

		// Deploy wasm to testnet
		// const storeCode = new MsgStoreCode(
		// 	wallet.key.accAddress,
		// 	fs.readFileSync('../artifacts/mftmx_marketplace.wasm').toString('base64')
		// );
		// const storeCodeTx = await wallet.createAndSignTx({
		// 	msgs: [storeCode],
		// });
		// console.log("storeCodeTx", storeCodeTx);
		// const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx)
		
		// console.log("storeCodeTxResult", storeCodeTxResult);
		
		// if (isTxError(storeCodeTxResult)) {
		// 	throw new Error(
		// 		`store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}`
		// 	);
		// }
		
		// const {
		// 	store_code: { code_id },
		// } = storeCodeTxResult.logs[0].eventsByType;
		// console.log("Done");
		// console.log("\nCodeId: ", code_id[0]);

		// // Instantiate contract
		// console.log("\n\nInstantiate token contract");
		// const instantiate = new MsgInstantiateContract(
		// 	wallet.key.accAddress,wallet.key.accAddress,
		// 	Number(code_id[0]), // code ID
		// 	{
		// 		"decimals": "6",
		// 		"name": "simple_test2",
		// 		"symbol":"1233"
		// 	}
		// );
		
		// const instantiateTx = await wallet.createAndSignTx({
		// 	msgs: [instantiate],
		// });
		// const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

		// if (isTxError(instantiateTxResult)) {
		// 	throw new Error(
		// 		`instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}}`
		// 	);
		// }
		
		// const {
		// 	instantiate_contract: { contract_address },
		// } = instantiateTxResult.logs[0].eventsByType;

		// console.log("contract_address?", contract_address[0])

		let timeStamp = Math.floor(Date.now() / 1000 ) + 3600 * 24 * 7
		console.log('timeStamp', timeStamp)
		const expire_at = {
				// "at_time": "1668544526734254325", // 19 digits format
				"never": {}
		},
		price = {
			"amount": "200", //0.000001 Luna
			"info": {
				"native_token": {"denom": "uluna"}
			}
		}
		const createOrderMsg = { create_order: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju", price, expire_at } }
		const cancelOrderMsg = { cancel_order: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }
		const exeOrderMsg = { execute_order: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }

		const createBidMsg = { create_bid: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju", price, expire_at } }
		const cancelBidMsg = { cancel_bid: { token_id: "1", nft_address: "terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju" } }

		// const mintMsg = {
		// 		mint: {
		// 			name: "coderighter2-nft",
		// 			owner: wallet.key.accAddress,
		// 			token_id: "1",
		// 			description: "This is for testing",
		// 			image: ""
		// 		}
		// 	}

		// transfer nft to market place contracts
		// const transferNftMsg = {transfer_nft : {recipient: "terra1xdrunssyep0242tpwx8d87w9c8e2vutqz2528u", token_id: "1"}}
		// const increase = new MsgExecuteContract(
		// 	wallet.key.accAddress, // sender
		// 	// contract_address[0], // contract account address,
		// 	"terra1rmw87h769rt553myzcvnqavvnqzqxm2r9twsju",
		// 	transferNftMsg
		// )

		const increase = new MsgExecuteContract(
			wallet.key.accAddress, // sender
			// contract_address[0], // contract account address,
			"terra1vfas6tt3tsdnzqy42g7u0my68gln4ep9n34w5u",
			exeOrderMsg
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