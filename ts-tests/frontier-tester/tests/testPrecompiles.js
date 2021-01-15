const { assert } = require('chai');
const contract = require("@truffle/contract");
const { account, describeWithEdgeware } = require('../helpers/utils');

describeWithEdgeware('Precompiles test', async (context) => {
  let web3;

  before(async () => {
    web3 = context.web3;
  });

  it('should perform identity directly', async () => {
    const message = '0x1234567890'
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000004',
      from: account,
      data: message,
    });
    assert.equal(callResult, message);
  });

  // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-198.md
  it('should perform modexp directly', async () => {
    // 3**(2**256 - 2**32 - 978) % (2**256 - 2**32 - 977) = 1
    const message = '0x0000000000000000000000000000000000000000000000000000000000000001'
      + '0000000000000000000000000000000000000000000000000000000000000020'
      + '0000000000000000000000000000000000000000000000000000000000000020'
      + '03'
      + 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e'
      + 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f';
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000005',
      from: account,
      data: message,
    });
    const localResult = '0x0000000000000000000000000000000000000000000000000000000000000001';
    assert.equal(callResult, localResult);
  });

  it('should perform ed25519verify directly', async () => {
    // 'test' + //Alice pubkey + sig, generated via subkey
    const message = '0x0000000000000000000000000000000000000000000000000000000074657374'
      + '88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee'
      + '003d4e3fca386deff58df1d86f8bb490d3204e14d14ef5e51af03e34b72f7abea34ec295d6a0c055388e521c91f60b25e5199ec5d4c2c1af433ac040a75d8f0a';
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000006',
      from: account,
      data: message,
    });
    const localResult = '0x00000000';
    assert.equal(callResult, localResult);
  });

  it('should perform invalid ed25519verify directly', async () => {
    // invalid test
    const message = '0x0000000000000000000000000000000000000000000000000000000074657375'
      + '88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee'
      + '003d4e3fca386deff58df1d86f8bb490d3204e14d14ef5e51af03e34b72f7abea34ec295d6a0c055388e521c91f60b25e5199ec5d4c2c1af433ac040a75d8f0a';
    const callResult = await web3.eth.call({
      to: '0000000000000000000000000000000000000006',
      from: account,
      data: message,
    });
    const localResult = '0x00000001';
    assert.equal(callResult, localResult);
  })
});
