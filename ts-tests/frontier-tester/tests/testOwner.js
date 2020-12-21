const { assert } = require('chai');
const OwnerContract = require('../build/contracts/OwnerContract.json');
const contract = require("@truffle/contract");
const { deployContract, account, initWeb3, describeWithEdgeware } = require('../helpers/utils');

describeWithEdgeware("OwnerContract test", async () => {
  let OC;
  let web3;

  before(async function() {
    web3 = await initWeb3();

    OCDeploy = contract({
      abi: OwnerContract.abi,
      unlinked_binary: OwnerContract.bytecode,
    });
    OCDeploy.setProvider(web3.currentProvider);
    OC = await OCDeploy.new({ from: account });
  });

  it("should have owner", async () => {
    let result = await OC.makeCall({
      from: account,
    });
    assert.isTrue(result);
  });

  it("should fail with wrong owner", async () => {
    // NOTE: this will fail, because the error will be:
    //     { code: -32603, message: 'inner executing call failed' }
    // rather than a revert!
    try {
      await OC.makeCall({ from: '0xF8cef78E923919054037a1D03662bBD884fF4edf' });
      assert.fail('should throw');
    } catch (e) {
      
    }
  });
});
