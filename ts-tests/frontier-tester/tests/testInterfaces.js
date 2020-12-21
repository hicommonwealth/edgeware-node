const { assert } = require("chai");
const IContractUser = require('../build/contracts/IContractUser.json');
const ContractImpl = require('../build/contracts/ContractImpl.json');
const { account, initWeb3, describeWithEdgeware } = require('../helpers/utils');
const contract = require("@truffle/contract");

describeWithEdgeware('Interfaces test', async () => {
  it('should access deployed interface', async () => {
    const web3 = await initWeb3();

    console.log('Deploying something-doer...')
    let CIContract = contract({
      abi: ContractImpl.abi,
      unlinked_binary: ContractImpl.bytecode,
    });
    CIContract.setProvider(web3.currentProvider);
    let ci = await CIContract.new({ from: account });

    console.log('Deploying caller of interface...');
    let ICUContract = contract({
      abi: IContractUser.abi,
      unlinked_binary: IContractUser.bytecode,
    });
    ICUContract.setProvider(web3.currentProvider);
    let icu = await ICUContract.new({ from: account });

    // test calling the subcontract
    console.log('Linking contracts...');
    const tx = await icu.linkContract(ci.address, { from: account })
    
    console.log('Calling method on subcontract...');
    const res = await icu.doTheThing.call({ from: account });
    assert.isTrue(res);
  });
});
