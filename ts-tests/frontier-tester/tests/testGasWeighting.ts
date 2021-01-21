// TODO: try to send in many txs at once with different gas values and ensure that the highest ones go through
//  - see what happens, do some get dropped? how does it work?
//  - basically try to break the gas system

import { assert } from 'chai';
import StoreNum from '../build/contracts/StoreNum.json';
const contract = require("@truffle/contract");
const { account, describeWithEdgeware } = require('../helpers/utils.js');

describeWithEdgeware('Gas Weighting Test', async (context) => {
  it.skip('should resolve higher priced tx earlier', async () => {
    const web3 = context.web3;

    // init a contract that stores numbers
    let SN = contract({
      abi: StoreNum.abi,
      unlinked_binary: StoreNum.bytecode,
    });
    SN.setProvider(web3.currentProvider);
    const sn = await SN.new({ from: account });

    // send out then wait for several transactions with different gas prices
    const txPs = [];
    const gas = await sn.addToStore.estimateGas(1);
    console.log(`Estimated gas: ${gas}.`);
    for (let i = 1; i <= 200; ++i) {
      try {
        const tx = sn.addToStore(i, {
          from: account,
          gasPrice: `${i % 2 ? 0 : 1000}`,
          gas: (new web3.utils.BN(gas)).muln(2).toString(),
        });
        txPs.push(tx);
      } catch (e) {
        console.log(`Failed to submit tx ${i} -- stopping.`);
        break;
      }
    }
    const receipts = await Promise.all(txPs);
    const indexedReceipts = receipts.map((v, i) => [ v.receipt, i ]);
    console.log(indexedReceipts.map((r) => `tx idx ${r[0].transactionIndex} adding value ${r[1]} processed on block ${r[0].blockNumber}`));

    // check which order they were processed
    const store = await sn.getStore();
    console.log(store.map((n) => +n));
  });
});
