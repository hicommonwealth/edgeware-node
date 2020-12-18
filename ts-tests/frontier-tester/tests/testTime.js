const { assert } = require("chai");
const TimeContract = require("../build/contracts/TimeContract.json");
const { initWeb3, account } = require('../helpers/utils');
const contract = require("@truffle/contract");

function timeout(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function blockTimeifyDate(n) {
  return Math.floor(n / 1000);
}

describe("TimeContract test", async () => {
  it("should be testable", async () => {
    const web3 = initWeb3();
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
    assert.equal(dNow.substring(0, dNow.length - 1), now.toString().substring(0, now.toString().length - 1));

    // wait 1s
    await timeout(1000);
    const now2 = await t.viewNow.call({ from: account });
    dNow = blockTimeifyDate(Date.now()).toString();
    assert.equal(dNow.substring(0, dNow.length - 1), now2.toString().substring(0, now2.toString().length - 1));

    // wait 1s
    await timeout(1000);
    const now3 = await t.viewNow.call({ from: account });
    dNow = blockTimeifyDate(Date.now()).toString();
    assert.equal(dNow.substring(0, dNow.length - 1), now3.toString().substring(0, now3.toString().length - 1));
  });
});
