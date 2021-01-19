/* eslint-disable prefer-template */
import { spec } from '@edgeware/node-types';
import { ApiPromise } from '@polkadot/api';
import { HttpProvider } from '@polkadot/rpc-provider';
import { xxhashAsHex } from '@polkadot/util-crypto';
import fs from 'fs';
import path from 'path';
import chalk from 'chalk';
import { execFileSync, execSync } from 'child_process';

// input paths
const binaryPath = '../../target/release/edgeware';
const wasmPath = '../../../edgeware-node-3.1.0/edgeware_runtime.wasm';

// output paths
const outputDir = path.join(__dirname, 'forker-data');
const hexPath = path.join(outputDir, 'runtime.hex');
const originalSpecPath = path.join(outputDir, 'genesis.json');
const forkedSpecPath = path.join(outputDir, 'fork.json');
const storagePath = path.join(outputDir, 'storage.json');

// Using http endpoint since substrate's Ws endpoint has a size limit.
const provider = new HttpProvider(process.env.HTTP_RPC_ENDPOINT || 'http://beresheet2.edgewa.re:9933');

/**
 * All module prefixes except those mentioned in the skippedModulesPrefix will be added to this by the script.
 * If you want to add any past module or part of a skipped module, add the prefix here manually.
 *
 * Any storage value’s hex can be logged via console.log(api.query.<module>.<call>.key([...opt params])),
 * e.g. console.log(api.query.timestamp.now.key()).
 *
 * If you want a map/doublemap key prefix, you can do it via .keyPrefix(),
 * e.g. console.log(api.query.system.account.keyPrefix()).
 *
 * For module hashing, do it via xxhashAsHex,
 * e.g. console.log(xxhashAsHex('System', 128)).
 */
const prefixes = [
  // '0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9' /* System.Account */
];
const skippedModulesPrefix = ['System', 'Session', 'Aura', 'Grandpa', 'GrandpaFinality', 'FinalityTracker'];

async function main() {
  if (!fs.existsSync(binaryPath)) {
    console.log(chalk.red('Binary missing.'));
    process.exit(1);
  }
  execFileSync('chmod', ['+x', binaryPath]);

  if (!fs.existsSync(wasmPath)) {
    console.log(chalk.red('WASM missing.'));
    process.exit(1);
  }

  // create data folder if needed
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir);
  }

  execSync('cat ' + wasmPath + ' | hexdump -ve \'/1 "%02x"\' > ' + hexPath);

  console.log(chalk.green('We are intentionally using the HTTP endpoint. '
    + 'If you see any warnings about that, please ignore them.'));
  const api = await ApiPromise.create({ provider, ...spec });

  // Download state of original chain
  console.log(chalk.green('Fetching current state of the live chain. '
    + 'Please wait, it can take a while depending on the size of your chain.'));
  const pairs = await provider.send('state_getPairs', ['0x']);
  fs.writeFileSync(storagePath, JSON.stringify(pairs));

  const metadata: any = await api.rpc.state.getMetadata();
  // Populate the prefixes array
  const modules = JSON.parse(metadata.asLatest.modules);
  modules.forEach((m) => {
    if (m.storage) {
      if (!skippedModulesPrefix.includes(m.storage.prefix)) {
        prefixes.push(xxhashAsHex(m.storage.prefix, 128));
      }
    }
  });

  // Generate chain spec for original and forked chains
  execSync(`${binaryPath} build-spec --raw --chain=beresheet > ${originalSpecPath}`);
  execSync(`${binaryPath} build-spec --dev --raw > ${forkedSpecPath}`);

  const storage = JSON.parse(fs.readFileSync(storagePath, 'utf8'));
  const originalSpec = JSON.parse(fs.readFileSync(originalSpecPath, 'utf8'));
  const forkedSpec = JSON.parse(fs.readFileSync(forkedSpecPath, 'utf8'));

  // Modify chain name and id
  forkedSpec.name = originalSpec.name + '-fork';
  forkedSpec.id = originalSpec.id + '-fork';
  forkedSpec.protocolId = originalSpec.protocolId;

  // Grab the items to be moved, then iterate through and insert into storage
  storage
    .filter((i) => prefixes.some((prefix) => i[0].startsWith(prefix)))
    .forEach(([key, value]) => {
      forkedSpec.genesis.raw.top[key] = value;
    });

  // Delete System.LastRuntimeUpgrade to ensure that the on_runtime_upgrade event is triggered
  delete forkedSpec.genesis.raw.top['0x26aa394eea5630e07c48ae0c9558cef7f9cce9c888469bb1a0dceaa129672ef8'];

  // Set the code to the current runtime code
  forkedSpec.genesis.raw.top['0x3a636f6465'] = '0x' + fs.readFileSync(hexPath, 'utf8').trim();

  // To prevent the validator set from changing mid-test, set Staking.ForceEra to ForceNone ('0x02')
  forkedSpec.genesis.raw.top['0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3'] = '0x02';

  fs.writeFileSync(forkedSpecPath, JSON.stringify(forkedSpec, null, 4));

  console.log(`Forked genesis generated successfully. Find it at ./${outputDir}/fork.json`);
  process.exit();
}

main();
