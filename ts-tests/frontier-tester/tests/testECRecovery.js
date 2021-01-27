const { assert } = require('chai');
const contract = require("@truffle/contract");
const { account, privKey, describeWithEdgeware } = require('../helpers/utils');
const ECRecovery = require('../build/contracts/ECRecovery.json');

describeWithEdgeware('ECRecovery test', async (context) => {
  it('should recover account from signature and hash', async () => {
    const web3 = context.web3;

    let ECR = contract({
      abi: ECRecovery.abi,
      unlinked_binary: ECRecovery.bytecode,
    });
    ECR.setProvider(web3.currentProvider);

    const c = await ECR.new({ from: account });

    // prepare a signed message
    const message = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.'
    const messageHex = '0x' + Buffer.from(message).toString('hex');
    const signature = await web3.eth.sign(messageHex, account);
    const hash = web3.utils.sha3('\x19Ethereum Signed Message:\n' + message.length + message);
    
    // recover the signer
    const address = await c.recover(hash, signature, { from: account, gas: web3.utils.toWei('1', 'ether') });
    assert.equal(address.toLowerCase(), account.toLowerCase());
  });

  it('should interact with precompile directly', async () => {
    const web3 = context.web3;
    const ECRECOVER_PRECOMPILE_ADDRESS = '0000000000000000000000000000000000000000';

    const message = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.'
    const messageHex = '0x' + Buffer.from(message).toString('hex');
    const sig = (await web3.eth.sign(messageHex, account)).slice(2);
    const r = `${sig.slice(0, 64)}`
    const s = `${sig.slice(64, 128)}`
    const v = `${sig.slice(128, 130)}`
    const sigPart = `${Buffer.alloc(31).toString('hex')}${v}${r}${s}`;
    const hash = web3.utils.sha3('\x19Ethereum Signed Message:\n' + message.length + message).slice(2);

    const RAW_TX = {
      from: account,
      gas: '27720',
      to: ECRECOVER_PRECOMPILE_ADDRESS,
      value: "0x0",
      data: `0x${hash.toString('hex')}${sigPart}`,
    };

    const SIGNED_TX = await web3.eth.accounts.signTransaction(
      RAW_TX,
      privKey
    );

    const tx = await web3.eth.sendTransaction({
      from: account,
      to: ECRECOVER_PRECOMPILE_ADDRESS,
      value: '0x0',
      gas: '27720',
      data: `0x${hash.toString('hex')}${sigPart}`,
    });

    assert.equal(tx.transactionHash, SIGNED_TX.transactionHash);
  });
});
