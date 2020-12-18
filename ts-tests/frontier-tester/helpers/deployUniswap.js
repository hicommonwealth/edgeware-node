const contract = require("@truffle/contract");

// libraries
const Multicall = require('../build/contracts/Multicall.json');
const WETH9 = require('@uniswap/v2-periphery/build/WETH9.json');
const UniswapV2Factory = require('@uniswap/v2-core/build/UniswapV2Factory.json');
const UniswapV2Router02 = require('@uniswap/v2-periphery/build/UniswapV2Router02.json');

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
};

module.exports = {
   deploy,
 }