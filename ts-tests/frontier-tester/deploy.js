const EdgewarePrivateKeyProvider = require ('./private-provider')
const Web3 = require('web3');
const contract = require("@truffle/contract");

// libraries
const Multicall = require('./build/contracts/Multicall.json');
const WETH9 = require('@uniswap/v2-periphery/build/WETH9.json');
const UniswapV2Factory = require('@uniswap/v2-core/build/UniswapV2Factory.json');
const UniswapV2Router02 = require('@uniswap/v2-periphery/build/UniswapV2Router02.json');

// tokens
const TokenA = require('./build/contracts/TokenA.json');
const TokenB = require('./build/contracts/TokenB.json');

// Initialization
const { account, privKey, initWeb3 } = require('./utils');
const web3 = initWeb3();

const deploy = async () => {
   const d = async (name, Contract, args = []) => {
      let c = contract({
         abi: Contract.abi,
         unlinked_binary: Contract.bytecode,
      });
      c.setProvider(web3.currentProvider);
      let res = await c.new(...args, { from: account });
      console.log(`${name} deployed at address ${res.address}`);
      return res;
   };
   const multicall = await d("Multicall", Multicall);
   const factory = await d("UniswapV2Factory", UniswapV2Factory, [ account ]);
   const weth9 = await d("WETH9", WETH9);
   const router = await d(
      "UniswapV2Router02",
     UniswapV2Router02,
     [ factory.address, weth9.address ],
   );
   return [factory.address, router.address];
   // const tokenBAddress = await deployContract("TokenB", TokenB, [ web3.utils.toWei('8000000') ]);
   // const tokenAAddress = await deployContract("TokenA", TokenA, [ web3.utils.toWei('8000000') ]);
   // await deployPair();
};

const FACTORY_ADDRESS = '0x5c4242beB94dE30b922f57241f1D02f36e906915';
const TOKEN_A_ADDRESS = '0xe573BCA813c741229ffB2488F7856C6cAa841041';
const WETH_ADDRESS = '0x42e2EE7Ba8975c473157634Ac2AF4098190fc741';

const deployPair = async () => {
   // deploy a pair immediately
   const factory = new web3.eth.Contract(UniswapV2Factory.abi, FACTORY_ADDRESS);
   try {
      const data = factory.methods.createPair(TOKEN_A_ADDRESS, WETH_ADDRESS).encodeABI();
      const createTransaction = await web3.eth.accounts.signTransaction(
         {
            from: address,
            data,
            gas: '0x4000000000000000',
            gasLimit: '0x5000000000000000',
            gasPrice: 10,
         },
         privKey
      );
      const createReceipt = await web3.eth.sendSignedTransaction(
         createTransaction.rawTransaction
      );
      console.log(createReceipt);
      const pairAddress = await factory.methods.getPair(TOKEN_A_ADDRESS, WETH_ADDRESS).call();
      console.log(pairAddress);
      process.exit(0);
   } catch (e) {
      console.log('Failed to create pair: ', e.message);
      process.exit(1);
   }
}

module.exports = {
   deploy,
   deployPair
 }