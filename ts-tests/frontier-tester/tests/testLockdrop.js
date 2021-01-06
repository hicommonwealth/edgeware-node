const Lockdrop = require("../build/contracts/Lockdrop.json");
const utility = require('../helpers/lockdropWeb3Util');
const rlp = require('rlp');
const keccak = require('keccak');
const { assert } = require('chai');
const contract = require("@truffle/contract");
const { account, describeWithEdgeware } = require('../helpers/utils');

describeWithEdgeware("Lockdrop test", async (context) => {
  const SECONDS_IN_DAY = 86400;
  const THREE_MONTHS = 0;
  const SIX_MONTHS = 1;
  const TWELVE_MONTHS = 2;
  let LD;
  let web3;

  before(async function() {
    web3 = context.web3;

    LD = contract({
      abi: Lockdrop.abi,
      unlinked_binary: Lockdrop.bytecode,
    });
    LD.setProvider(web3.currentProvider);
  });

  it('should setup and pull constants', async function () {
    let time = await utility.getCurrentTimestamp(web3);
    let lockdrop = await LD.new(time, { from: account });
    const LOCK_DROP_PERIOD = (await lockdrop.LOCK_DROP_PERIOD.call({ from: account })).toNumber();
    const LOCK_START_TIME = (await lockdrop.LOCK_START_TIME.call({ from: account })).toNumber();
    time = await utility.getCurrentTimestamp(web3);
    assert.equal(LOCK_DROP_PERIOD, SECONDS_IN_DAY * 92);
    assert.ok(LOCK_START_TIME <= time && time <= LOCK_START_TIME + 1000);
  });

  it('ensure the contract address matches JS RLP script', async function () {
    let time = await utility.getCurrentTimestamp(web3);
    let lockdrop = await LD.new(time, { from: account });
    const sender = account;
    const nonce = (await web3.eth.getTransactionCount(sender));
    const input = [ sender, nonce - 1 ];
    const rlpEncoded = rlp.encode(input);
    const contractAddressLong = keccak('keccak256').update(rlpEncoded).digest('hex');
    const contractAddr = contractAddressLong.substring(24);

    time = await utility.getCurrentTimestamp(web3);
    assert.equal(web3.utils.toBN(lockdrop.address).toString(), web3.utils.toBN(contractAddr).toString());
  });
  
  // Events don't work
  it('should lock funds and increment nonce', async function () {
    let time = await utility.getCurrentTimestamp(web3);
    let lockdrop = await LD.new(time, { from: account });

    let startNonce = await web3.eth.getTransactionCount(lockdrop.address);
    assert.equal(startNonce, '1', 'start nonce of deployed contract should be 1');

    let senderBalance = new web3.utils.BN(await web3.eth.getBalance(account));

    const bcontractAddr1 = getContractAddress(lockdrop.address, startNonce);
    const bcontractAddr2 = getContractAddress(lockdrop.address, startNonce + 1)
    const bcontractAddr3 = getContractAddress(lockdrop.address, startNonce + 2);
    const bcontractAddr4 = getContractAddress(lockdrop.address, startNonce + 3);

    const value = web3.utils.toWei('10', 'ether');

    await lockdrop.lock(THREE_MONTHS, account, true, {
      from: account,
      value: value,
      gas: 1500000,
      gasPrice: 1000,
    });

    let balLock1 = await web3.eth.getBalance(bcontractAddr1);
    let balLock2 = await web3.eth.getBalance(bcontractAddr2);
    let balLock3 = await web3.eth.getBalance(bcontractAddr3);
    let balLock4 = await web3.eth.getBalance(bcontractAddr4);

    assert.equal(value.toString(), balLock1, 'balance of first lock does not match expected');
    assert.equal(0, balLock2, 'balance of future second lock does not match expected');
    assert.equal(0, balLock3, 'balance of future third lock does not match expected');
    assert.equal(0, balLock4, 'balance of future fourth lock does not match expected');

    let senderBalanceAfter = new web3.utils.BN(await web3.eth.getBalance(account));
    let sentBalance = senderBalance.sub(senderBalanceAfter);
    assert.isTrue(sentBalance.gt(new web3.utils.BN(value)), 'sent balance should be greater than lock value');

    const nonce = (await web3.eth.getTransactionCount(lockdrop.address));
    const contractAddr = getContractAddress(lockdrop.address, nonce - 1);
    assert.equal(nonce, '2', 'contract nonce of Lockdrop contract should be 2 after lock')

    const bal0 = await web3.eth.getBalance(contractAddr);

    assert.equal(bal0, value, 'Lock value at address should be 10 eth after lock');

    const value2 = web3.utils.toWei('100', 'ether');

    await lockdrop.lock(THREE_MONTHS, account, true, {
      from: account,
      value: value2,
      gas: 1500000,
      gasPrice: 1000000000,
    });

    const new_nonce = (await web3.eth.getTransactionCount(lockdrop.address));
    const new_contractAddr = getContractAddress(lockdrop.address, new_nonce - 1);
    const bal2 = await web3.eth.getBalance(new_contractAddr);

    assert.equal(bal2, value2, '2nd lock value should be non zero after lock');
    assert.equal(new_nonce - 1, nonce, 'nonce should increment');

    balLock1 = await web3.eth.getBalance(bcontractAddr1);
    balLock2 = await web3.eth.getBalance(bcontractAddr2);
    balLock3 = await web3.eth.getBalance(bcontractAddr3);
    balLock4 = await web3.eth.getBalance(bcontractAddr4);

    assert.equal(value.toString(), balLock1, 'balance of first lock does not match expected');
    assert.equal(value2.toString(), balLock2, 'balance of second lock does not match expected');
    assert.equal(0, balLock3, 'balance of future third lock does not match expected');
    assert.equal(0, balLock4, 'balance of future fourth lock does not match expected');
  });
});

function getContractAddress(address, nonce)  {
  const input = [address, nonce]
  const rlpEncoded = rlp.encode(input);
  const contractAddressLong = keccak('keccak256').update(rlpEncoded).digest('hex');
  const contractAddr = contractAddressLong.substring(24);
  return contractAddr;
}