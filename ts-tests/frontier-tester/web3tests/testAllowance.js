const { assert } = require("chai");
const { account, initWeb3 } = require('../utils');
// const UniswapV2ERC20 = require('../node_modules/@uniswap/v2-core/build/UniswapV2ERC20.json');
const ERC20 = require('../node_modules/@openzeppelin/contracts/build/contracts/ERC20.json');
const contract = require("@truffle/contract");

describe("Allowance test", async () => {
  it("should compute allowance", async () => {
    const web3 = initWeb3();

    let erc = contract({
      abi: ERC20.abi,
      unlinked_binary: ERC20.bytecode,
    });
    erc.setProvider(web3.currentProvider);

    const v = web3.utils.toWei('10', 'ether');
    let c = await erc.new({ from: account });

    // create with value
    const approvalAccount = '0xc0ffee254729296a45a3885639AC7E10F9d54979';
    await c.approve(approvalAccount, v, { from: account });

    const allowance = await c.allowance.call(account, approvalAccount, { from: account });
    assert.equal(allowance, v);
  });
});
