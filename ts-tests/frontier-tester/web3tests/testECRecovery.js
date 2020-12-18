const { assert } = require('chai');
const contract = require("@truffle/contract");
const { account, initWeb3, privKey } = require('../utils');
const ECRecovery = require('../build/contracts/ECRecovery.json');
const EdgewarePrivateKeyProvider = require('../private-provider');
const signing_account = account;
const signing_privKey = '1111111111111111111111111111111111111111111111111111111111111111';
const Web3 = require('web3');

describe('ECRecovery test', async () => {
  let signWeb3

    beforeEach(async function() {
      const signingWeb3 = (pkey = privKey) => {
        const provider = new EdgewarePrivateKeyProvider(pkey, "http://localhost:9933/", 42);
        const web3 = new Web3(provider);
        return web3;
      };
  
      signWeb3 = signingWeb3(signing_privKey);
    });

  it('should recover account from signature and hash', async () => {
    const web3 = initWeb3();

    let ECR = contract({
      abi: ECRecovery.abi,
      unlinked_binary: ECRecovery.bytecode,
    });
    ECR.setProvider(web3.currentProvider);

    const c = await ECR.new({ from: account });

    // prepare a signed message
    const message = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.'
    const messageHex = '0x' + Buffer.from(message).toString('hex');
    const signature = await signWeb3.eth.sign(messageHex, signing_account);
    const hash = signWeb3.utils.sha3('\x19Ethereum Signed Message:\n' + message.length + message);
    
    // recover the signer
    const address = await c.recover(hash, signature, { from: account, gas: web3.utils.toWei('1', 'ether') });
    assert.equal(address.toLowerCase(), signing_account.toLowerCase());
  });

  it('should interact with precompile directly', async () => {
    const web3 = initWeb3();
    const ECRECOVER_PRECOMPILE_ADDRESS = '0000000000000000000000000000000000000001';

    const message = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.'
    const messageHex = '0x' + Buffer.from(message).toString('hex');
    const sig = (await signWeb3.eth.sign(messageHex, signing_account)).slice(2);
    const r = `${sig.slice(0, 64)}`
    const s = `${sig.slice(64, 128)}`
    const v = `${sig.slice(128, 130)}`
    const sigPart = `${Buffer.alloc(31).toString('hex')}${v}${r}${s}`;
    const hash = signWeb3.utils.sha3('\x19Ethereum Signed Message:\n' + message.length + message).slice(2);

    const RAW_TX = {
      from: signing_account,
      gasPrice: "0x01",
      gas: web3.utils.toWei('1', 'ether'),
      to: ECRECOVER_PRECOMPILE_ADDRESS,
      value: "0x0",
      data: `0x${hash.toString('hex')}${sigPart}`,
    };

    const SIGNED_TX = await signWeb3.eth.accounts.signTransaction(
      RAW_TX,
      privKey
    );

    const tx = await web3.eth.sendTransaction({
      from: signing_account,
      to: ECRECOVER_PRECOMPILE_ADDRESS,
      value: '0x0',
      gas: web3.utils.toWei('1', 'ether'),
      data: `0x${hash.toString('hex')}${sigPart}`,
    });

    assert.equal(tx.transactionHash, SIGNED_TX.transactionHash);
  });
});
