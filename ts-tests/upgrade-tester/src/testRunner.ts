import child_process from 'child_process';
import fs from 'fs';
import rimraf from 'rimraf';

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { UnsubscribePromise } from '@polkadot/api/types';
import { u128 } from '@polkadot/types';
import { compactAddLength } from '@polkadot/util';
import { spec } from '@edgeware/node-types';
import StateTest from './stateTest';
const path = require('path');
const schemaPath = path.join(__dirname, 'forker-data', 'schema.json');

import { makeTx } from './util';
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

    // seed of sudo account, which can execute `setCode`
    sudoSeed: string;

    // path to the binary file containing the upgraded chain executable
    // leave blank to upgrade without requiring a chain restart/change in chain binary
    binaryPath?: string;

    // run upgrade on new node with older runtime as specified in binaryPath
    // if false or unset, will run upgrade on old node then switch to new node
    upgradeOnNewNode?: boolean;

    // command to run after all tests, will not exit chain until complete
    postUpgradeCommand?: { cmd: string, env: NodeJS.Dict<string> };
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
    if (!options.chainspec || !fs.existsSync(options.chainspec)) {
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
      if (!options.upgrade.sudoSeed) {
        throw new Error('invalid sudo seed!');
      }
    } else {
      log.info('Will not perform upgrade during testing.');
    }
    if (!options.wsUrl) {
      log.info('No websocket URL found, defaulting to ws://localhost:9744.');
      options.wsUrl = 'ws://localhost:9744';
    }
  }

  // Starts a chain and configures its output to write to a given location, as
  // specified in the options object.
  // 'clearBasePath' is set to true to remove the chain database at startup,
  //   for a clean start, whereas post-upgrade it should be false.
  private _startChain(clearBasePath: boolean) {
    // pass through SIGINT to chain process
    process.on('SIGINT', () => {
      this._stopChain().then(() => process.exit(1));
    });

    if (clearBasePath) {
      // clear base path and replace with an empty directory
      if (fs.existsSync(this.options.chainBasePath)) {
        // we use rimraf because fs.remove doesn't support recursive removal
        log.info(`rimraf ${this.options.chainBasePath}`);
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
      '--wasm-execution', 'Compiled',
      '--alice', // TODO: abstract this into accounts somehow
      '--force-authoring',
      '--no-telemetry',
      '--no-prometheus',      
      '-linfo',
      '--rpc-port', '9733',
      '--ws-port', '9744'
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
    if (this._api) {
      await this._api.disconnect();
    }
    delete this._api;

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

    // wait 5s for port to reopen
    log.info('Waiting 5s for chain to exit...');
    await new Promise<void>((resolve) => setTimeout(() => resolve(), 5000));
  }

  // With a valid chain running, construct a polkadot-js API and
  // initialize a connection to the chain. Returns the spec version.
  private async _startApi(): Promise<number> {
    log.info(`Connecting to chain at ${this.options.wsUrl}...`);

    // initialize provider separately from the API: the API throws an error
    // if the chain is not available immediately
    const provider = new WsProvider(this.options.wsUrl, 5000); // 5s reconnect time

    // this promise waits for the provider to connect to the chain, and then
    // removes the listener for 'connected' events.
    let unsubscribe: () => void;
    await new Promise<void>((resolve) => {
      unsubscribe = provider.on('connected', () => resolve());
    });
    unsubscribe();

    // initialize the API itself    
    if (!fs.existsSync(schemaPath)) {
      this._api = await ApiPromise.create({ provider, ...spec });
    } else {
      const { types, rpc } = JSON.parse(fs.readFileSync(schemaPath, 'utf8'));
      this._api = await ApiPromise.create({
        provider,
        types,
        rpc,
      });
    }


    // this._api = await ApiPromise.create({ provider, ...spec });

    // fetch and print chain information
    const chainInfo = await this._api.rpc.state.getRuntimeVersion();
    log.info(`API connected to chain ${chainInfo.specName.toString()}:${+chainInfo.specVersion}!`);
    return +chainInfo.specVersion;
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
  private async _runTests(preUpgrade: boolean): Promise<boolean> {
    if (!this._api) throw new Error('API not initialized!');

    // TODO: move this set-balance into a test case
    if (this.options.upgrade.sudoSeed && preUpgrade) {
      const sudoKeyring = (new Keyring({ ss58Format: this.options.ss58Prefix, type: 'sr25519' }))
        .addFromUri(this.options.upgrade.sudoSeed);              
      const newBalance = new u128(this._api.registry, '1000000000000000000000000');
      const setBalanceTx = this._api.tx.sudo.sudo(
        this._api.tx.balances.setBalance(sudoKeyring.address, newBalance, 0)
      );
      await makeTx(this._api, setBalanceTx, sudoKeyring);
    }
    log.info(`KR KR KR '${true}' .`);
    let rpcSubscription: UnsubscribePromise;

    // run all tests, then perform upgrade if needed
    let needsUpgrade = false;
    if (preUpgrade) {
      for (const t of this.tests) {
        try {
          await t.before(this._api);
          log.info(`Test '${t.name}' action 'before' succeeded.`);
        } catch (e) {
          log.error(`Test '${t.name}' action 'before' failed: ${e.message}.`);
        }
      }

      // set flag to run upgrade
      if (this.options.upgrade) {
        needsUpgrade = true;
      }
    } else {
      for (const t of this.tests) {
        try {
          await t.after(this._api);
          log.info(`Test '${t.name}' action 'after' succeeded.`);
        } catch (e) {
          log.error(`Test '${t.name}' action 'after' failed: ${e.message}.`);
        }
      }
      log.info('All tests complete!');
    }

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

    // [3.5.] If upgradeOnNewNode is set, restart chain immediately on new node
    if (this.options.upgrade.binaryPath && this.options.upgrade.upgradeOnNewNode) {
      await this._stopChain();
      this.options.binaryPath = this.options.upgrade.binaryPath;
      this._startChain(false);
      const version = await this._startApi();
      if (version !== startVersion) {
        await this._stopChain();
        throw new Error('Version should not change on node switch!');
      }
    }

    // 4. Run tests via API
    const needsUpgrade = await this._runTests(true);

    // end run if no upgrade needed
    if (!needsUpgrade) {
      await this._stopChain();
      return;
    }

    // [5.] Upgrade chain via API
    await this._doUpgrade(false);

    // [6.] Restart chain with upgraded binary (if needed)
    if (this.options.upgrade.binaryPath
        && this.options.binaryPath !== this.options.upgrade.binaryPath
        && !this.options.upgrade.upgradeOnNewNode) {
      await this._stopChain();
      this.options.binaryPath = this.options.upgrade.binaryPath;
      this._startChain(false);
    }

    // [7.] Reconstruct API
    const newVersion = await this._startApi();
    if (startVersion >= newVersion) {
      await this._stopChain();
      throw new Error('Upgrade failed! Version did not change.');
    }

    // [8.] Run additional tests post-upgrade
    await this._runTests(false);

    // Cleanup and exit
    await this._stopChain();

    // [9.] Run post-upgrade command if present
    if (this.options.upgrade.postUpgradeCommand) {
      log.info(`Running post-upgrade command:\n\t${this.options.upgrade.postUpgradeCommand.cmd}`
        + `\n\t\t(env: ${JSON.stringify(this.options.upgrade.postUpgradeCommand.env)})`
        + '\n\t\t...(may take some time)...');
      await new Promise<void>((resolve, reject) => {
        child_process.exec(
          this.options.upgrade.postUpgradeCommand.cmd,
          { env: this.options.upgrade.postUpgradeCommand.env },
          (error, stdout, stderr) => {
            if (error) {
              log.error(`error: ${error.message}`);
              reject(new Error('post-upgrade command failed'));
            }
            if (stderr) {
              log.error(`stderr: ${stderr}`);
            }
            log.info(`stdout: ${stdout}`);
            resolve();
          }
        );
      });
    }

    process.exit(0);
  }
}

export default TestRunner;
