const { assert } = require('chai');
const { account, describeWithEdgeware } = require('../helpers/utils');
const contract = require("@truffle/contract");
const FallbackContract = require('../build/contracts/FallbackContract.json');

describeWithEdgeware('Fallback test', async (context) => {
  it('should return funds sent to invalid function', async () => {
    const web3 = context.web3;

    // deploy contract
    const FB = contract({
      abi: FallbackContract.abi,
      unlinked_binary: FallbackContract.bytecode,
    });
    FB.setProvider(web3.currentProvider);
    const c = await FB.new({ from: account });

    // prepare an invalid function call
    const balanceBefore = await web3.eth.getBalance(account);
    const functionSig = web3.eth.abi.encodeFunctionSignature('myMethod()');
    const valueSent = new web3.utils.BN(web3.utils.toWei('10', 'ether'));
    const receipt = await web3.eth.sendTransaction({
      from: account,
      to: c.address,
      value: valueSent.toString(),
      gasLimit: new web3.utils.BN(web3.utils.toWei('1', 'gwei')),
      data: functionSig,
    });
    const balanceAfter = await web3.eth.getBalance(account);
    const balanceDiff = new web3.utils.BN(balanceBefore).sub(new web3.utils.BN(balanceAfter));

    // ensure the value sent was (mostly) returned
    assert.isTrue(balanceDiff.lt(valueSent));

    // ensure some gas fees were spent
    assert.isTrue(balanceDiff.gtn(0));
  });
})