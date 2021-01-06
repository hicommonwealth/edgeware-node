import path from 'path';
import TestRunner from './testRunner';
import StateTest from './stateTest';

import { factory, formatFilename } from './logging';
const log = factory.getLogger(formatFilename(__filename));

const CHAINSPEC = 'dev';
const BINARY_PATH = '../../../edgeware-node-3.1.0/target/release/edgeware';
const CHAIN_BASE_PATH = `${__dirname}/../db`;
const ACCOUNTS = [ '//Alice' ];
const SS58_PREFIX = 42; // default for testing chain specs

const UPGRADE_BINARY = '../../target/release/edgeware';
const UPGRADE_CODE = '../../edgeware_runtime.wasm';
const SUDO_SEED = '//Alice';
const UPGRADE_ON_NEW_NODE = true;
const POST_UPGRADE_COMMAND = {
  env: { BASE_PATH: `${__dirname}/../db` },
  cmd: `cd ${__dirname}/../../frontier-tester && yarn init-eth-balance && yarn test-ci`,
};

async function main() {
  // construct some migration tests
  // TODO: make this a part of the arg initialization
  const tests: StateTest[] = [];
  tests.push(
    new ((await import('./tests/democracy')).default)(),
    // new ((await import('./tests/staking')).default)(),
    // new ((await import('./tests/identity')).default)(),
    // new ((await import('./tests/treasury')).default)(),
  );

  // construct tester
  const tester = new TestRunner(tests, {
    chainspec: CHAINSPEC,
    binaryPath: BINARY_PATH,
    chainBasePath: CHAIN_BASE_PATH,
    accountSeeds: ACCOUNTS,
    ss58Prefix: SS58_PREFIX,
    chainLogPath: path.join(CHAIN_BASE_PATH, 'out.log'),
    // upgrade: null,
    upgrade: {
      codePath: UPGRADE_CODE,
      binaryPath: UPGRADE_BINARY,
      sudoSeed: SUDO_SEED,
      upgradeOnNewNode: UPGRADE_ON_NEW_NODE,
      postUpgradeCommand: null // POST_UPGRADE_COMMAND,
    },
  });

  try {
    await tester.run();
    process.exit(0);
  } catch (e) {
    log.error(`TESTER FAILURE: ${e.message}`);
    process.exit(1);
  }
}

// kick off test script
main();
