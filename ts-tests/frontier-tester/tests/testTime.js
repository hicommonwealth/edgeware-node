const { assert } = require("chai");
const TimeContract = require("../build/contracts/TimeContract.json");
const { account, describeWithEdgeware } = require('../helpers/utils');
const contract = require("@truffle/contract");

const BLOCK_TIME_MS = 6000;

function timeout(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function blockTimeifyDate(n) {
  return Math.floor(n / BLOCK_TIME_MS) * (BLOCK_TIME_MS / 1000);
}

describeWithEdgeware("TimeContract test", async (context) => {
  it("should be testable", async () => {
    const web3 = context.web3;
    let Time = contract({
      abi: TimeContract.abi,
      unlinked_binary: TimeContract.bytecode,
    });
    Time.setProvider(web3.currentProvider);

    let t = await Time.new({ from: account });
    await t.timeBeforeEnd({ from: account, gasPrice: 1000000000 });

    // fetch initial values
    let now = await t.viewNow.call({ from: account });
    let dNow = blockTimeifyDate(Date.now()).toString();
    assert.equal(dNow, now.toString());

    // wait a block
    await timeout(BLOCK_TIME_MS);
    const now2 = await t.viewNow.call({ from: account });
    dNow = blockTimeifyDate(Date.now()).toString();
    assert.equal(dNow, now2.toString());

    // wait a block
    await timeout(BLOCK_TIME_MS);
    const now3 = await t.viewNow.call({ from: account });
    dNow = blockTimeifyDate(Date.now()).toString();
    assert.equal(dNow, now3.toString());
  });
});
