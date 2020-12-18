const EdgewarePrivateKeyProvider = require ('./private-provider')
const Web3 = require('web3');
const { assert } = require('chai');

const privKeyA = '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342';
const providerA = new EdgewarePrivateKeyProvider(privKeyA, 'http://localhost:9933/', 42);
const privKeyB = '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E343';
const providerB = new EdgewarePrivateKeyProvider(privKeyB, 'http://localhost:9933/', 42);

const web3 = new Web3(providerA);
// web3.eth.accounts.wallet.add({
//   privateKey: privKeyA,
//   address: providerA.address,
// });

const test = async () => {
  const [ A, B ] = [ providerA.address, providerB.address ];

  // fetch initial balances
  console.log('Fetching initial balances...');
  const startingBalanceA = await web3.eth.getBalance(A);
  const startingBalanceB = await web3.eth.getBalance(B);

  // perform transfer 1: A -> B
  console.log('Performing first transfer...');
  const amount1 = web3.utils.toWei(web3.utils.toBN('5'), 'ether');
  const tx1 = await web3.eth.sendTransaction({
    from: A, to: B, value: amount1,
    gas: web3.utils.toWei('1', 'ether'),
  });

  // NOTE: Since transactions on edgeware do not return a populated gasUsed value,
  //   this value is derived "observationally" from `edgeware --dev` on my local
  //   machine. No guarantee it will work elsewhere.
  const gasUsed = web3.utils.toBN('21000');
  
  // check updated balances
  console.log('Fetching updated balances...');
  const updatedBalance1A = await web3.eth.getBalance(A);
  const updatedBalance1B = await web3.eth.getBalance(B);
  const expectedBalance1A = web3.utils.toBN(startingBalanceA).sub(amount1).sub(gasUsed);
  const expectedBalance1B = web3.utils.toBN(startingBalanceB).add(amount1);
  assert.equal(updatedBalance1A.toString(), expectedBalance1A.toString());
  assert.equal(updatedBalance1B.toString(), expectedBalance1B.toString());

  // perform transfer 2: B -> A
  console.log('Performing second transfer...');
  const amount2 = web3.utils.toWei(web3.utils.toBN('4'), 'ether');
  // use alternate web3 here to support B's provider
  const web3B = new Web3(providerB);
  web3B.eth.accounts.wallet.add({
    privateKey: privKeyB,
    address: providerB.address,
  });
  const tx2 = await web3B.eth.sendTransaction({
    from: B, to: A, value: amount2,
    gas: web3.utils.toWei('1', 'ether'),
  });

  // check updated balances
  console.log('Fetching updated balances...');
  const updatedBalance2A = await web3.eth.getBalance(A);
  const updatedBalance2B = await web3.eth.getBalance(B);
  const expectedBalance2A = web3.utils.toBN(updatedBalance1A).add(amount2);
  const expectedBalance2B = web3.utils.toBN(updatedBalance1B).sub(amount2).sub(gasUsed);
  assert.equal(updatedBalance2A.toString(), expectedBalance2A.toString());
  assert.equal(updatedBalance2B.toString(), expectedBalance2B.toString());

  console.log('Test completed successfully!');
  process.exit(0);
};

test();
