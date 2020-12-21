import { ApiPromise, WsProvider } from '@polkadot/api';
import { createTestPairs } from '@polkadot/keyring/testingPairs';
import { assert } from 'chai';
const { account, convertToSubstrateAddress, initWeb3 } = require('./utils.js');
import { dev } from '@edgeware/node-types';
import { TypeRegistry } from '@polkadot/types';

let sendSubstrateBalance = async (): Promise<void> => {
  // initialize polkadot API
  const polkadotUrl = 'ws://localhost:9944';
  const registry = new TypeRegistry();
  const api = await (new ApiPromise({
    provider: new WsProvider(polkadotUrl),
    registry,
    ...dev,
  })).isReady;

  // configure funded substrate account, target account, and balance to send
  const keyring = createTestPairs().dave;
  const balance = await api.query.system.account(keyring.address);
  if (balance.data.free.eqn(0)) {
    throw new Error(`Fetched no balance for address ${keyring.address}`);
  }
  const value = balance.data.free.divn(2); // onl; send half of Bob's balance
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
  const web3 = await initWeb3();
  const web3Balance = await web3.eth.getBalance(account);
  console.log(`Transfer complete, web3 fetched balance: ${web3Balance}`);
}

sendSubstrateBalance().then(() => process.exit(0));
