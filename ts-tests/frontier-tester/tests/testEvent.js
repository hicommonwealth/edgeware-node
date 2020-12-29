const { assert } = require('chai');
const Web3 = require('web3');
const EventContract = require('../build/contracts/EventContract.json');
const { deployContract, account, describeWithEdgeware } = require('../helpers/utils');
const contract = require("@truffle/contract");

describeWithEdgeware("EventContract test", async (context) => {
  it("should emit event", async () => {
    const web3 = context.web3;
    let EC = contract({
      abi: EventContract.abi,
      unlinked_binary: EventContract.bytecode,
    });
    EC.setProvider(web3.currentProvider);
    let c = await EC.new({ from: account });
    let res = await c.emitEvent({ from: account });
    assert.equal(res.receipt.logs.length, 1);
    assert.equal(res.receipt.logs[0].event, 'e');
  });

  it('should receive event thru web3 subscribe', async () => {
    // init an HTTP web3 and use to set up another account for ws
    const web3Http = context.web3;
    const wsPrivKey = '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342';
    const wsAccount = '0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b';
    const value = web3Http.utils.toWei(web3Http.utils.toBN('1'), 'ether');
    const balanceTx = await web3Http.eth.sendTransaction({
      from: account, to: wsAccount, value, gas: value,
    });

    // init web3 with wsprovider
    const web3Ws = new Web3(new Web3.providers.WebsocketProvider("ws://localhost:9944/"));
    web3Ws.eth.accounts.wallet.add(wsPrivKey);
    web3Ws.eth.defaultAccount = wsAccount;
    const c = await deployContract('EventContract', EventContract, [], web3Ws, wsPrivKey);
    const cAddress = c._address;

    // init subscription
    await new Promise(async (resolve) => {
      const sid = await new Promise((innerResolve) => {
        c.events.allEvents()
        .on('data', (data) => {
          assert.equal(data.event, 'e');
          assert.equal(data.address, cAddress);
          resolve();
        })
        .on('error', console.error)
        .on('connected', (id) => innerResolve(id));
      })
      console.log(`Event subscription connected with id ${sid}.`);

      // use HTTP web3 connection to fire a tx to emit event
      let EC = contract({
        abi: EventContract.abi,
        unlinked_binary: EventContract.bytecode,
      });
      EC.setProvider(web3Http.currentProvider);
      const cc = await EC.at(cAddress);
      const emitTx = await cc.emitEvent({ from: account });
    });
  })
});
