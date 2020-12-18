const Promise = require('bluebird');

const advanceTimeAndBlock = async (time) => {
    await advanceTime(time);
    await advanceBlock();

    return getCurrentBlock();
};

const advanceTime = (time, web3) => {
    return new Promise((resolve, reject) => {
        web3.currentProvider.send({
            jsonrpc: '2.0',
            method: 'evm_increaseTime',
            params: [time],
            id: new Date().getTime()
        }, (err, result) => {
            if (err) { return reject(err); }
            else {
              if (!err) {
                web3.currentProvider.send({
                  jsonrpc: '2.0', 
                  method: 'evm_mine', 
                  params: [], 
                  id: new Date().getSeconds()
                }, (e, res) => {
                  if (e) reject(e);
                  else resolve(res);
                });
              }
            }
        });
    });
};

const advanceBlock = (web3) => {
    return new Promise((resolve, reject) => {
        web3.currentProvider.send({
            jsonrpc: '2.0',
            method: 'evm_mine',
            id: new Date().getTime()
        }, (err, result) => {
            if (err) { return reject(err); }
            web3.eth.getBlock('latest', function (err, res) {
              if (err) reject(err);
              resolve(res.hash);
            });
        });
    });
};

function getCurrentBlock(web3) {
  return new Promise((resolve, reject) => {
    web3.eth.getBlock('latest', function (err, res) {
      if (err) return reject(err);
      resolve(res);
    });
  });
}

async function getCurrentTimestamp(web3) {
  const block = await getCurrentBlock(web3);
  return block.timestamp;
}


const getBalance = (account, web3) => {
  return new Promise((resolve, reject) => {
    web3.eth.getBalance(account, (err, res) => {
      if (err) reject(err);
      else resolve(res);
    });
  });
};

const getTxReceipt = async (txHash, web3) => {
  return await web3.eth.getTransactionReceipt(txHash);
}

async function assertRevert(promise, invariants = () => {}) {
  try {
    await promise;
    assert.fail('Expected revert not received');
  } catch (error) {
    const revertFound = error.message.search('revert') >= 0 || error.message.search('invalid opcode');
    assert(revertFound, `Expected 'revert', got ${error} instead`);
    invariants.call()
  }
}

module.exports = {
  advanceTimeAndBlock,
  advanceTime,
  advanceBlock,
  getCurrentBlock,
  getCurrentTimestamp,
  getBalance,
  assertRevert,
  getTxReceipt,
};
