const Web3 = require('web3');
const HDWalletProvider = require("@truffle/hdwallet-provider");
const { decodeAddress, encodeAddress, blake2AsHex } = require('@polkadot/util-crypto');
const child_process = require('child_process');
const { exit } = require('process');

const DISPLAY_LOG = process.env.DISPLAY_LOG || false;
const BINARY_PATH = `../../target/release/edgeware`;
const SPAWNING_TIME = 30000;

// const account = '0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b';
const account = '0x19e7e376e7c213b7e7e7e46cc70a5dd086daff2a';
// const privKey = '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342';
const privKey = '1111111111111111111111111111111111111111111111111111111111111111';

const initWeb3 = async (pkey = privKey) => {
  // const provider = new Web3.providers.HttpProvider('http://localhost:9933/');
  const provider = new HDWalletProvider({
    privateKeys: [ pkey ],
    providerOrUrl: "http://localhost:9933/",
  });
  const web3 = new Web3(provider);
	const chainId = await web3.eth.getChainId();

  // ensure native web3 sending works as well as truffle provider
  web3.eth.accounts.wallet.add(privKey);
  web3.eth.defaultAccount = web3.eth.accounts.wallet[0].address;
  return web3;
};

const deployContract = async (name, c, args = [], web3 = undefined) => {
  let deployer, pkey;
  if (!web3) {
    web3 = await initWeb3();
    deployer = account;
    pkey = privKey;
  } else {
    deployer = web3.eth.accounts.wallet[0].address;
    pkey = web3.eth.accounts.wallet[0].privateKey;
  }

  console.log(`Attempting to deploy ${name} from account: ${deployer}`);
  const contract = new web3.eth.Contract(c.abi);

  const contractTx = contract.deploy({
     data: c.bytecode,
     arguments: args,
  });

  const data = contractTx.encodeABI();
  const createTransaction = await web3.eth.accounts.signTransaction(
     {
        from: deployer,
        data,
        gasLimit: 8000000,
        gasPrice: 1000000000,
     },
     pkey
  );

  const createReceipt = await web3.eth.sendSignedTransaction(
     createTransaction.rawTransaction
  );
  console.log(`${name} deployed at address ${createReceipt.contractAddress}`);
  return new web3.eth.Contract(c.abi, createReceipt.contractAddress);
};

const convertToEvmAddress = (substrateAddress) => {
  const addressBytes = decodeAddress(substrateAddress);
  return '0x' + Buffer.from(addressBytes.subarray(0, 20)).toString('hex');
}

const convertToSubstrateAddress = (evmAddress, prefix = 42) => {
  const addressBytes = Buffer.from(evmAddress.slice(2), 'hex');
  const prefixBytes = Buffer.from('evm:');
  const convertBytes = Uint8Array.from(Buffer.concat([ prefixBytes, addressBytes ]));
  const finalAddressHex = blake2AsHex(convertBytes, 256);
  return encodeAddress(finalAddressHex, prefix);
}

async function startEdgewareNode() {
	const cmd = BINARY_PATH;
	const args = [
    '--dev',
		'--no-telemetry',
		'--no-prometheus',
		'--tmp',
		'-lrpc=trace',
		'-levm=trace',
	];
	const binary = child_process.spawn(cmd, args);

	binary.on("error", (err) => {
		if (err.errno == "ENOENT") {
			console.error(
				`\x1b[31mMissing Frontier binary (${BINARY_PATH}).\nPlease compile the Frontier project:\ncargo build\x1b[0m`
			);
		} else {
			console.error(err);
		}
		process.exit(1);
	});

	const binaryLogs = [];
	await new Promise((resolve) => {
		const errHandle = () => {
			console.error(`\x1b[31m Failed to start Edgeware Node.\x1b[0m`);
			console.error(`Command: ${cmd} ${args.join(" ")}`);
			console.error(`Logs:`);
			console.error(binaryLogs.map((chunk) => chunk.toString()).join("\n"));
			process.exit(1);
		};
		const timer = setTimeout(errHandle, SPAWNING_TIME - 2000);

		const onData = async (chunk) => {
			if (DISPLAY_LOG) {
				console.log(chunk.toString());
			}
			binaryLogs.push(chunk);
			if (chunk.toString().match(/Address already in use/)) {
				clearTimeout(timer);
				errHandle();
			} else if (chunk.toString().match(/Prepared block for proposing at 1/)) {
				const web3 = await initWeb3();
				await web3.eth.getChainId();

				clearTimeout(timer);
				if (!DISPLAY_LOG) {
					binary.stderr.off("data", onData);
					binary.stdout.off("data", onData);
				}
				console.log(`\x1b[31m Starting RPC\x1b[0m`);
				resolve();
			}
		};

		// hook interrupt handler
		const exitHandler = () => {
			binary.kill();
			process.exit();
		};
		process.on('SIGINT', exitHandler);
		process.on('exit', exitHandler);

		// hook data printing
		binary.stderr.on("data", onData);
		binary.stdout.on("data", onData);
	});

	return binary;
}

function sleep(ms) {
	return new Promise(resolve => setTimeout(resolve, ms));
}

function describeWithEdgeware(title, cb) {
	describe(title, async () => {
		let binary;
		before("Starting Edgeware Node", async () => {
			binary = await startEdgewareNode();
		});

		after('Exiting Edgeware Node', async () => {
			console.log(`\x1b[31m Stopping RPC\x1b[0m`);
			await new Promise((resolve) => {
				binary.on('exit', () => {
					console.log('RPC STOPPED');
					resolve();
				})
				binary.kill();
			});
			await sleep(2000);
		});

		await cb();
  });
}

module.exports = {
  account,
  privKey,
  initWeb3,
  deployContract,
  convertToEvmAddress,
  convertToSubstrateAddress,
  describeWithEdgeware,
}
