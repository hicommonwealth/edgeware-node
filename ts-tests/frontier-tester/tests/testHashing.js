const { assert } = require('chai');
const contract = require("@truffle/contract");
const { account, initWeb3, describeWithEdgeware } = require('../helpers/utils');
const HashingContract = require('../build/contracts/Hashing.json');
const RIPEMD160 = require('ripemd160');
const { sha256 } = require('js-sha256');

// web3.eth.call returns a 32-byte hex string, but RIPEMD160 only returns a 20-byte result,
// so we need to trim the leading 0s off the call's return value in order to compare strings
// for equality.
const trimLeadingHex0s = (web3, n) => {
  return web3.utils.numberToHex(web3.utils.hexToNumberString(n));
}

describeWithEdgeware('Hashing test', async () => {
  let web3;
  let c;
  const message = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.'
  const messageBuf = Buffer.from(message);
  const messageHex = '0x' + messageBuf.toString('hex');

  before(async () => {
    // setup web3 and contract instance
    web3 = await initWeb3();
    const Hashing = contract({
      abi: HashingContract.abi,
      unlinked_binary: HashingContract.bytecode,
    });
    Hashing.setProvider(web3.currentProvider);
    c = await Hashing.new({ from: account });
  });

  it('should perform keccak256 thru contract', async () => {
    const contractResult = await c.callKeccak256.call(messageBuf, { from: account });
    const localResult = web3.utils.keccak256(messageHex);
    assert.equal(contractResult, localResult);
  });

  it('should perform ripemd160 thru contract', async () => {
    const contractResult = await c.callRipemd160.call(messageHex, { from: account });
    const localResult = '0x' + (new RIPEMD160().update(messageBuf).digest('hex'));
    assert.equal(contractResult, localResult);
  });

  it('should perform ripemd160 directly', async () => {
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000003',
      from: account,
      data: messageHex,
    });
    const localResult = '0x' + (new RIPEMD160().update(messageBuf).digest('hex'));
    assert.equal(trimLeadingHex0s(web3, callResult), localResult);
  });

  it('should obtain same ripemd results from contract and direct call', async () => {
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000003',
      from: account,
      data: messageHex,
    });
    const contractResult = await c.callRipemd160.call(messageHex, { from: account });
    assert.equal(trimLeadingHex0s(web3, callResult), contractResult);
  })

  it('should perform sha256 thru contract', async () => {
    const contractResult = await c.callSha256.call(messageHex, { from: account });
    const localResult = '0x' + sha256.hex(messageBuf);
    assert.equal(contractResult, localResult);
  });

  it('should perform sha256 directly', async () => {
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000002',
      from: account,
      data: messageHex,
    });
    const localResult = '0x' + sha256.hex(messageBuf);
    assert.equal(callResult, localResult);
  });

  it('should obtain same sha256 results from contract and direct call', async () => {
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000002',
      from: account,
      data: messageHex,
    });
    const contractResult = await c.callSha256.call(messageHex, { from: account });
    assert.equal(callResult, contractResult);
  })
});
