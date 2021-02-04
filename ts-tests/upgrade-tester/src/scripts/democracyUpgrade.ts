import fs from 'fs';
import { ApiPromise } from '@polkadot/api';
import { ReferendumIndex } from '@polkadot/types/interfaces';
import { Codec } from '@polkadot/types/types';
import { spec } from '@edgeware/node-types';
import { createTestPairs, TestKeyringMap } from '@polkadot/keyring/testingPairs';
import { compactAddLength } from '@polkadot/util';
import { makeTx } from '../util';

import { factory, formatFilename } from '../logging';
import { UnsubscribePromise } from '@polkadot/api/types';
const log = factory.getLogger(formatFilename(__filename));

// submits the upgrade as a democracy proposal + its associated preimage
async function submitUpgrade(api: ApiPromise, pairs: TestKeyringMap, codePath: string): Promise<void> {
  log.info('Submitting upgrade proposal.');
  const wasmFileData = fs.readFileSync(codePath);
  const data = new Uint8Array(wasmFileData);
  const codecData = compactAddLength(data);

  // construct upgrade call that sudo will run
  const upgrade = api.tx.system.setCode(codecData);

  // propose the upgrade
  const propose = api.tx.democracy.propose(upgrade.method.hash, api.consts.democracy.minimumDeposit);
  await makeTx(api, propose, pairs.alice);

  // note the preimage
  const preimage = api.tx.democracy.notePreimage(upgrade.method.toHex());
  await makeTx(api, preimage, pairs.alice);
}

// waits for the proposal to get tabled and the referendum to start
async function waitForReferendum(api: ApiPromise): Promise<number> {
  log.info('Waiting for upgrade proposal to table.');
  // check if proposal already began and return immediately if so
  const refCount = await api.query.democracy.referendumCount();
  if (+refCount > 0) {
    return +refCount - 1;
  }

  // otherwise, listen to events until it starts
  let unsubscribe: UnsubscribePromise;
  const refIdx = await new Promise<number>((resolve) => {
    unsubscribe = api.query.system.events((events) => {
      events.forEach((record) => {
        const { event } = record;
        if (event.method !== 'TreasuryMinting' && event.method !== 'ExtrinsicSuccess') {
          log.debug(`\t${event.section}:${event.method}`);
        }
        if (event.section === 'democracy' && event.method === 'Started') {
          const [ referendumIndex ] = event.data as unknown as [ ReferendumIndex ] & Codec;
          resolve(+referendumIndex);
        }
      });
    });
  });
  if (unsubscribe) {
    await unsubscribe;
  }
  return refIdx;
}

// votes on the upgrade so it passes, then waits for the referendum to execute
async function passUpgrade(api: ApiPromise, pairs: TestKeyringMap, refIdx: number): Promise<void> {
  log.info('Voting for upgrade referendum.');
  const { data: { free } } = await api.query.system.account(pairs.bob.address);
  const voteOptions = {
    Standard: {
      vote: {
        aye: true,
        conviction: 1,
      },
      // save some balance for the voting fees/deposit
      balance: free.divn(2).toString(),
    }
  };
  const vote = api.tx.democracy.vote(refIdx, voteOptions);
  await makeTx(api, vote, pairs.bob);

  const referendum = await api.query.democracy.referendumInfoOf(refIdx);
  log.debug(`Got referendum: ${JSON.stringify(referendum.toHuman())}`);

  log.info('Waiting for referendum to pass and execute.');
  return new Promise<void>((resolve) => {
    api.query.system.events((events) => {
      events.forEach((record) => {
        const { event } = record;
        if (event.method !== 'TreasuryMinting' && event.method !== 'ExtrinsicSuccess') {
          log.debug(`\t${event.section}:${event.method}`);
        }
        if (event.section === 'democracy' && event.method === 'Executed') {
          resolve();
        }
      });
    });
  });
}

async function main() {
  const api = await ApiPromise.create({ ...spec });
  const pairs = createTestPairs({ ss58Format: 7 });
  await submitUpgrade(api, pairs, `${__dirname}/../../../../edgeware_runtime.wasm`);
  const refIdx = await waitForReferendum(api);
  await passUpgrade(api, pairs, refIdx);
  log.info('Upgrade completed. Goodbye.');
  process.exit(0);
}

main();
