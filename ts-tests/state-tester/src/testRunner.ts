import child_process from 'child_process';
import fs from 'fs';
import rimraf from 'rimraf';
import BN from 'bn.js';

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { UnsubscribePromise } from '@polkadot/api/types';
import { TypeRegistry } from '@polkadot/types';
import { compactAddLength } from '@polkadot/util';
import { dev } from '@edgeware/node-types';
import StateTest from './stateTest';

import { factory, formatFilename } from './logging';
const log = factory.getLogger(formatFilename(__filename));

// configuration options for test runner
export interface ITestOptions {
  // spec of chain to run, should be 'dev' for test chains
  chainspec: string;

  // path to the `edgeware` binary
  binaryPath: string;

  // path to a directory to initialize the chain database
  chainBasePath: string;

  // list of account seeds to pass into tests
  accountSeeds: string[];

  // prefix used in SS58 address generation, 0 on test chains
  ss58Prefix: number;

  // websocket url exposed by a running chain, used to initialize the polkadot API
  // defaults to 'ws://localhost:9944'
  wsUrl?: string;

  // path or stream specifier to pipe chain stdout/stderr into.
  // leave undefined to ignore chain output
  chainLogPath?: string | 'stdout' | 'stderr';

  // upgrade-specific configuration:
  upgrade?: {
    // path to a file containing the WASM hex string used in `setCode`
    codePath: string,

    // block to send upgrade tx
    block: number;

    // seed of sudo account, which can execute `setCode`
    sudoSeed: string;

    // path to the binary file containing the upgraded chain executable
    // leave blank to upgrade without requiring a chain restart/change in chain binary
    binaryPath?: string;
  }
}

// Testing fixture for automating chain and API startup and upgrades
// such that general tests can run against it, maintaining state across
// API sessions and upgrades.
class TestRunner {
  private _api: ApiPromise;
  private _chainOutfile: fs.WriteStream;
  private _chainOutstream: NodeJS.WritableStream;
  private _chainProcess: child_process.ChildProcess;

  constructor(
    private tests: StateTest[],
    private options: ITestOptions,
  ) {
    // verify options args
    if (!options.chainspec) {
      throw new Error('missing chainspec!');
    }
    if (!options.binaryPath || !fs.existsSync(options.binaryPath)) {
      throw new Error('cannot find chain executable!');
    }

    // set defaults
    if (typeof options.ss58Prefix !== 'number') {
      log.info('No SS58 prefix found, defaulting to 0.');
      options.ss58Prefix = 0;
    }
    if (options.upgrade) {
      if (!options.upgrade.codePath || !fs.existsSync(options.upgrade.codePath)) {
        throw new Error('cannot find upgrade codepath!');
      }
      if (!options.upgrade.block) {
        throw new Error('invalid upgrade block!');
      }
      if (!options.upgrade.sudoSeed) {
        throw new Error('invalid sudo seed!');
      }
      log.info(`Will perform upgrade on block ${options.upgrade.block}.`);
    } else {
      log.info('Will not perform upgrade during testing.');
    }
    if (!options.wsUrl) {
      log.info('No websocket URL found, defaulting to ws://localhost:9944.');
      options.wsUrl = 'ws://localhost:9944';
    }
  }

  // Starts a chain and configures its output to write to a given location, as
  // specified in the options object.
  // 'clearBasePath' is set to true to remove the chain database at startup,
  //   for a clean start, whereas post-upgrade it should be false.
  private _startChain(clearBasePath: boolean) {
    // pass through SIGINT to chain process
    process.on('SIGINT', () => {
      if (this._chainProcess && this._chainProcess.connected) {
        this._chainProcess.kill('SIGINT');
      }
    });

    if (clearBasePath) {
      // clear base path and replace with an empty directory
      if (fs.existsSync(this.options.chainBasePath)) {
        // we use rimraf because fs.remove doesn't support recursive removal
        rimraf.sync(this.options.chainBasePath);
      }
      fs.mkdirSync(this.options.chainBasePath);
    }

    // open log files if necessary to configure the chain output stream
    if (this.options.chainLogPath) {
      // we set the 'a' flag to avoid overwriting the file when we re-init this
      // file stream on upgrade
      this._chainOutfile = fs.createWriteStream(this.options.chainLogPath, { flags: 'a' });
      this._chainOutstream = this._chainOutfile;
    } else {
      this._chainOutstream = process.stdout;
    }

    // start the chain with specified spec and basepath
    const args = [
      '--chain', this.options.chainspec,
      '--base-path', this.options.chainBasePath,
      '--alice', // TODO: abstract this into accounts somehow
      '-l', 'ws::handler=info'
    ];
    log.info(`Executing ${this.options.binaryPath} with args ${JSON.stringify(args)}`);
    this._chainProcess = child_process.spawn(
      this.options.binaryPath,
      args,
      // (error) => {
      //   // callback on exit
      //   if (error) log.info(`Received chain process error: ${error.message}.`);
      //   log.info('Chain exited.');
      // }
    );

    // pipe edgeware output to file in temp dir/process output if set
    if (this._chainOutstream) {
      this._chainProcess.stdout.pipe(this._chainOutstream);
      this._chainProcess.stderr.pipe(this._chainOutstream);
    }
  }

  // Stops an active chain and closes any file used to store its output.
  private async _stopChain() {
    // if (this._chainOutstream) {
    //   this._chainOutstream = undefined;
    // }
    // if (this._chainOutfile) {
    //   this._chainOutfile.close();
    //   this._chainOutfile = undefined;
    // }
    if (this._chainProcess) {
      await new Promise<void>((resolve) => {
        this._chainProcess.on('close', (code) => {
          log.info(`Edgeware exited with code ${code}.`);
          resolve();
        });
        log.info('Sending kill signal to Edgeware.');
        this._chainProcess.kill(9);
        this._chainProcess = undefined;
      });
    }
  }

  // With a valid chain running, construct a polkadot-js API and
  // initialize a connection to the chain. Returns the spec version.
  private async _startApi(): Promise<number> {
    log.info(`Connecting to chain at ${this.options.wsUrl}...`);

    // initialize provider separately from the API: the API throws an error
    // if the chain is not available immediately
    const provider = new WsProvider(this.options.wsUrl);

    // this promise waits for the provider to connect to the chain, and then
    // removes the listener for 'connected' events.
    let unsubscribe: () => void;
    await new Promise<void>((resolve) => {
      unsubscribe = provider.on('connected', () => resolve());
    });
    unsubscribe();

    // initialize the API itself
    const registry = new TypeRegistry();
    this._api = new ApiPromise({ provider, registry, ...dev });
    await this._api.isReady;

    // fetch and print chain information
    const chainInfo = await this._api.rpc.state.getRuntimeVersion();
    log.info(`API connected to chain ${chainInfo.specName.toString()}:${+chainInfo.specVersion}!`);
    return +chainInfo.specVersion;
  }

  // Disconnect an active polkadot-js API from the chain.
  private _stopApi() {
    if (this._api) {
      this._api.disconnect();
    }
    delete this._api;
  }

  // Performs an upgrade via a `sudo(setCode())` API call.
  // 'useCodeChecks' specifies whether to perform API-level checks on the WASM blob,
  //   and should be set to false for the current edgeware upgrade.
  private async _doUpgrade(useCodeChecks: boolean = true): Promise<any> {
    if (!this.options.upgrade) {
      log.info('No upgrade to perform!');
      return;
    }

    log.info('Performing upgrade...');
    const { sudoSeed, codePath } = this.options.upgrade;

    // construct sudo-er keyring
    const sudoKey = (new Keyring({ ss58Format: this.options.ss58Prefix, type: 'sr25519' }))
      .addFromUri(sudoSeed);

    // read WASM blob into memory
    const wasmFileData = fs.readFileSync(codePath);
    const data = new Uint8Array(wasmFileData);
    const codecData = compactAddLength(data);

    // construct upgrade call that sudo will run
    const upgradeCall = useCodeChecks
      ? this._api.tx.system.setCode(codecData)
      : this._api.tx.system.setCodeWithoutChecks(codecData);

    // construct and submit sudo call using the sudo seed and weight 1
    const sudoCall = this._api.tx.sudo.sudoUncheckedWeight(upgradeCall, 1);
    let txUnsubscribe: () => void;
    /* eslint-disable-next-line no-async-promise-executor */
    await new Promise<void>(async (resolve) => {
      txUnsubscribe = await sudoCall.signAndSend(sudoKey, (result) => {
        if (result.status.isInBlock) {
          log.info('Upgrade TX in block!');
          resolve();
        }
      });
    });
    if (txUnsubscribe) txUnsubscribe();
  }

  // with a valid chain and API connection, init tests
  private async _runTests(): Promise<boolean> {
    if (!this._api) throw new Error('API not initialized!');

    // TODO: move this set-balance into a test case
    if (this.options.upgrade.sudoSeed) {
      const sudoKeyring = (new Keyring({ ss58Format: this.options.ss58Prefix, type: 'sr25519' }))
        .addFromUri(this.options.upgrade.sudoSeed);
      const newBalance = new BN('1000000000000000000000000');
      const setBalanceTx = this._api.tx.sudo.sudo(
        this._api.tx.balances.setBalance(sudoKeyring.address, newBalance, 0)
      );
      const hash = await setBalanceTx.signAndSend(sudoKeyring);
      log.info('Set sudo balance!');
    }

    let rpcSubscription: UnsubscribePromise;
    // subscribe to new blocks and run tests as they occur
    // Promise resolves to "true" if an upgrade is pending,
    //   otherwise "false" if testing is completed.
    const testCompleteP: Promise<boolean> = new Promise((resolve) => {
      rpcSubscription = this._api.rpc.chain.subscribeNewHeads(async (header) => {
        const blockNumber = +header.number;
        log.info(`Got block ${blockNumber}.`);

        // perform upgrade after delay
        if (this.options.upgrade && blockNumber === this.options.upgrade.block) {
          resolve(true);
        }

        const runnableTests = this.tests.filter((t) => !!t.actions[blockNumber]);
        // run the selected tests
        await Promise.all(runnableTests.map(async (t) => {
          const { name, fn } = t.actions[blockNumber];
          try {
            await fn(this._api);
            log.info(`Test '${t.name}' action '${name}' succeeded.`);
          } catch (e) {
            log.info(`Test '${t.name}' action '${name}' failed: ${e.message}.`);
          }
        }));
        if (this.tests.every((test) => test.isComplete(blockNumber))) {
          log.info('All tests complete!');
          resolve(false);
        }
      });
    });

    // wait for the tests to complete
    const needsUpgrade = await testCompleteP;

    // once all tests complete, kill the chain subscription
    if (rpcSubscription) (await rpcSubscription)();
    return needsUpgrade;
  }

  // main function to begin the testing process
  public async run() {
    // 1. Prepare chain directories and chain output file (if used),
    //    then start the chain.
    this._startChain(true);

    // 3. Construct API via websockets
    const startVersion = await this._startApi();

    // 4. Run tests via API
    const needsUpgrade = await this._runTests();

    // end run if no upgrade needed
    if (!needsUpgrade) {
      this._stopApi();
      await this._stopChain();
      return;
    }

    // [5.] Upgrade chain via API
    await this._doUpgrade();

    // [6.] Restart chain with upgraded binary (if needed)
    this._stopApi();
    if (this.options.upgrade.binaryPath
        && this.options.binaryPath !== this.options.upgrade.binaryPath) {
      await this._stopChain();
      this.options.binaryPath = this.options.upgrade.binaryPath;
      this._startChain(false);
    }

    // [7.] Reconstruct API
    const newVersion = await this._startApi();
    if (startVersion >= newVersion) {
      this._stopApi();
      await this._stopChain();
      throw new Error('Upgrade failed! Version did not change.');
    }

    // [8.] Run additional tests post-upgrade
    await this._runTests();

    // Cleanup and exit
    this._stopApi();
    await this._stopChain();
  }
}

export default TestRunner;
