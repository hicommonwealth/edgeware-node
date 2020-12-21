const contract = require("@truffle/contract");
const { assert } = require("chai");
const { account, initWeb3, describeWithEdgeware } = require('../helpers/utils');
const { deploy } = require('../helpers/deployUniswap');

const TokenA = require('../build/contracts/TokenA.json');
const TokenB = require('../build/contracts/TokenB.json');
const UniswapV2Router02 = require('../node_modules/@uniswap/v2-periphery/build/UniswapV2Router02.json');
const UniswapV2Factory = require('../node_modules/@uniswap/v2-core/build/UniswapV2Factory.json');
const UniswapV2Pair = require('../node_modules/@uniswap/v2-core/build/UniswapV2Pair.json');

describeWithEdgeware('Add Liquidity Test', async () => {
   let FACTORY_ADDRESS;
   let ROUTER_ADDRESS;

   before('Deploy uniswap contracts', async () => {
      const d = await deploy();
      [FACTORY_ADDRESS, ROUTER_ADDRESS] = d;
    });

   it('should create uniswap pair', async () => {
      // deploy two tokens
      const web3 = await initWeb3();
      const amount0 = web3.utils.toWei('10');
      const amount1 = web3.utils.toWei('10');
   
      console.log('Deploying first token...');
      const TokenAContract = contract({
         abi: TokenA.abi,
         unlinked_binary: TokenA.bytecode,
      });
      TokenAContract.setProvider(web3.currentProvider);
      const token0 = await TokenAContract.new(web3.utils.toWei('100'), { from: account });
      const address0 = token0.address;

      console.log('Deploying second token...');
      const TokenBContract = contract({
         abi: TokenB.abi,
         unlinked_binary: TokenB.bytecode,
      });
      TokenBContract.setProvider(web3.currentProvider);
      const token1 = await TokenBContract.new(web3.utils.toWei('100'), { from: account });
      const address1 = token1.address;

      console.log('Approving first token...');
      const receipt0 = await token0.approve(ROUTER_ADDRESS, amount0, {
         from: account
      });
      console.log('Approving second token...');
      const receipt1 = await token1.approve(ROUTER_ADDRESS, amount1, {
         from: account
      });
   
      // create the pair
      const RouterContract = contract({
         abi: UniswapV2Router02.abi,
         unlinked_binary: UniswapV2Router02.bytecode,
      });
      RouterContract.setProvider(web3.currentProvider);
      const router = await RouterContract.at(ROUTER_ADDRESS);
      const args = [
         address0, address1,
         amount0, amount1,
         "0", "0",
         account,
         Math.ceil(Date.now() / 1000) + (60 * 20), // 1 day
         { from: account, gas: web3.utils.toWei('100') }, // { from: account, gasLimit: 10000000, gasPrice: 1500000000 },
      ];
      console.log('Adding liquidity with args: ', args);
      const liquidityReceipt = await router.addLiquidity(...args);
      
      // query the pair
      const FactoryContract = contract({
         abi: UniswapV2Factory.abi,
         unlinked_binary: UniswapV2Factory.bytecode,
      });
      FactoryContract.setProvider(web3.currentProvider);
      console.log('Querying factory for pair...');
      const factory = await FactoryContract.at(FACTORY_ADDRESS, { from: account });
      const pairAddress = await factory.getPair.call(address0, address1, {
         from: account,
      });
      console.log(pairAddress);
      const nPairs = await factory.allPairsLength.call({ from: account });
   
      // query the pair's reserves
      assert.notEqual(+nPairs, 0);
      assert.notEqual(+(pairAddress.slice(2)), 0);
      console.log(`Got pair: ${pairAddress} (${nPairs} total pairs).`);
      const PairContract = contract({
         abi: UniswapV2Pair.abi,
         unlinked_binary: UniswapV2Pair.bytecode,
      });
      PairContract.setProvider(web3.currentProvider);
      const pair = await PairContract.at(pairAddress);
      const result = await pair.getReserves.call({ from: account });
      console.log(result);
      console.log('result[0].toString()');
      console.log(result[0].toString());
      console.log(amount0);
      console.log('result[1].toString()');
      console.log(result[1].toString());
      console.log(amount1);
      assert.equal(result[0].toString(), amount0);
      assert.equal(result[1].toString(), amount1);
   });
});
