import { ApiPromise, WsProvider } from '@polkadot/api';
import { createTestPairs } from '@polkadot/keyring/testingPairs';
import { assert } from 'chai';
const { account, convertToSubstrateAddress, describeWithEdgeware } = require('./utils.js');
import { spec } from '@edgeware/node-types';
import Web3 from 'web3';

let sendSubstrateBalance = async (web3: Web3): Promise<void> => {
  // initialize polkadot API
  const polkadotUrl = 'ws://localhost:9944';
  const api = await ApiPromise.create({
    provider: new WsProvider(polkadotUrl),
    ...spec,
  });

  // configure funded substrate account, target account, and balance to send
  const keyring = createTestPairs().dave;
  const balance = await api.query.system.account(keyring.address);
  if (balance.data.free.eqn(0)) {
    throw new Error(`Fetched no balance for address ${keyring.address}`);
  }
  const value = balance.data.free.divn(2); // only send half of Bob's balance
  const target = convertToSubstrateAddress(account);

  // send funds from Bob to the target account
  console.log(`Transferring ${value.toString()} from Bob to default account.`);
  const tx = api.tx.balances.transfer(target, value);
  await new Promise<void>((resolve) => {
    tx.signAndSend(keyring, (result) => {
      if (result.isError) {
        assert.fail('tx failure');
      }
      if (result.isCompleted) {
        resolve();
      }
    });
  });

  // ensure the account has funds via web3
  const web3Balance = await web3.eth.getBalance(account);
  await api.disconnect();
  console.log(`Transfer complete, web3 fetched balance: ${web3Balance}`);
}

describeWithEdgeware('init default account balance', (context) => {
  it('should initialize default account balance', async () => {
    const web3 = context.web3;
    await sendSubstrateBalance(web3);
  });
});
