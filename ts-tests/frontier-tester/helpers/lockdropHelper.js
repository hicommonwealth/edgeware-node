const Promise = require('bluebird');
const keyring = require('@polkadot/keyring');
const { toBN, fromWei, hexToNumber } = require('web3').utils;
const bs58 = require('bs58');
const schedule = require('./schedule');
const generalizedLocks = require('./generalizedLocks');

function getEffectiveValue(ethAmount, term, lockTime, lockStart, totalETH) {
  // multiplicative bonus starts at 100 / 100 = 1
  let bonus = toBN(100);
  // get multiplicative bonus if calculating allocation of locks
  if (lockTime && lockStart) {
    bonus = schedule.getEarlyParticipationBonus(lockTime, lockStart);
  }

  if (term == '0') {
    // three month term yields no bonus
    return toBN(ethAmount).mul(toBN(100).mul(bonus)).div(toBN(10000));
  } else if (term == '1') {
    // six month term yields 30% bonus
    return toBN(ethAmount).mul(toBN(130).mul(bonus)).div(toBN(10000));
  } else if (term == '2') {
    // twelve month term yields 120% bonus
    return toBN(ethAmount).mul(toBN(220).mul(bonus)).div(toBN(10000));
  } else if (term == 'signaling') {
    // 80% deduction
    return toBN(ethAmount).mul(toBN(20)).div(toBN(100));
  } else {
    // invalid term
    return toBN(0);
  }
}

const getLocks = async (lockdropContract, address) => {
  // let events = await lockdropContract.events.allEvents();
  // console.log(events);
  // return events;
  console.log(address);

  await lockdropContract.getPastEvents('Locked', 
  {
    fromBlock: 0,
    toBlock: 'latest',
    filter: {
    //   edgewareAddr: 0xa469e40f0a073be5b28e2df6e746ce6519260cdd764bc5f6b3fb3aac5cda3c35,
    //   // // transacationHash: 0x2d0120f7efc7d594e738cf68013d2c8d8b6bee3d92d69a8f58f30fe5186daa56,
      owner: address,
    }
  }).then((events) => {
    console.log(events);
    return events;
  });
};

const getSignals = async (lockdropContract, address) => {
  return await lockdropContract.getPastEvents('Signaled', {
    fromBlock: 0,
    toBlock: 'latest',
    filter: {
      contractAddr: address,
    }
  });
};

const getTotalLockedBalance = async (lockdropContract) => {
  let { totalETHLocked, totalEffectiveETHLocked } = await calculateEffectiveLocks(lockdropContract);
  return { totalETHLocked, totalEffectiveETHLocked };
};

const getTotalSignaledBalance = async (web3, lockdropContract) => {
  let { totalETHSignaled, totalEffectiveETHSignaled } = await calculateEffectiveSignals(web3, lockdropContract);
  return { totalETHSignaled, totalEffectiveETHSignaled };
};

const calculateEffectiveLocks = async (lockdropContract) => {
  let totalETHLocked = toBN(0);
  let totalEffectiveETHLocked = toBN(0);
  const locks = {};
  const validatingLocks = {};

  let lockEvents = []
  let events = await lockdropContract.getPastEvents('Locked', {
    fromBlock: 0,
    toBlock: 'latest',
  });

  lockEvents = [ ...lockEvents, ...events ];

  // For truffle tests
  let lockdropStartTime;
  if (typeof lockdropContract.LOCK_START_TIME === 'function') {
    lockdropStartTime = (await lockdropContract.LOCK_START_TIME());
  } else {
    lockdropStartTime = (await lockdropContract.methods.LOCK_START_TIME().call());
  }

  lockEvents.forEach((event) => {
    const data = event.returnValues;
    let value = getEffectiveValue(data.eth, data.term, data.time, lockdropStartTime, totalETHLocked);
    totalETHLocked = totalETHLocked.add(toBN(data.eth));
    totalEffectiveETHLocked = totalEffectiveETHLocked.add(value);

    // Add all validators to a separate collection to do validator election over later
    if (data.isValidator) {
      if (data.edgewareAddr in validatingLocks) {
        validatingLocks[data.edgewareAddr] = {
          lockAmt: toBN(data.eth).add(toBN(validatingLocks[data.edgewareAddr].lockAmt)).toString(),
          effectiveValue: toBN(validatingLocks[data.edgewareAddr].effectiveValue).add(value).toString(),
          lockAddrs: [data.lockAddr, ...validatingLocks[data.edgewareAddr].lockAddrs],
        };
      } else {
        validatingLocks[data.edgewareAddr] = {
          lockAmt: toBN(data.eth).toString(),
          effectiveValue: value.toString(),
          lockAddrs: [data.lockAddr],
        };
      }
    }


    // Add all locks to collection, calculating/updating effective value of lock
    if (data.edgewareAddr in locks) {
      locks[data.edgewareAddr] = {
        lockAmt: toBN(data.eth).add(toBN(locks[data.edgewareAddr].lockAmt)).toString(),
        effectiveValue: toBN(locks[data.edgewareAddr].effectiveValue).add(value).toString(),
        lockAddrs: [data.lockAddr, ...locks[data.edgewareAddr].lockAddrs],
      };
    } else {
      locks[data.edgewareAddr] = {
        lockAmt: toBN(data.eth).toString(),
        effectiveValue: value.toString(),
        lockAddrs: [data.lockAddr],
      };
    }
  });
  // Return validating locks, locks, and total ETH locked
  return { validatingLocks, locks, totalETHLocked, totalEffectiveETHLocked };
};

const calculateEffectiveSignals = async (web3, lockdropContracts, blockNumber=8461046) => {
  let totalETHSignaled = toBN(0);
  let totalEffectiveETHSignaled = toBN(0);
  let signals = {};
  let seenContracts = {};
  let signalEvents = [];
  for (index in lockdropContracts) {
    let events = await lockdropContracts[index].getPastEvents('Signaled', {
      fromBlock: 0,
      toBlock: 'latest',
    });

    signalEvents = [ ...signalEvents, ...events ];
  }

  const promises = signalEvents.map(async (event) => {
    const data = event.returnValues;
    // Get balance at block that lockdrop ends
    let balance = -1;
    while (balance == -1) {
      try {
        if (blockNumber) {
          balance = await web3.eth.getBalance(data.contractAddr, blockNumber);
        } else {
          balance = await web3.eth.getBalance(data.contractAddr);
        }
      } catch(e) {
        console.log(`${balance} Couldn't find: ${JSON.stringify(data)}`);
        await Promise.delay(5000);
      }
    }

    return balance;
  });
  // Resolve promises to ensure all inner async functions have finished
  let balances = await Promise.all(promises);
  let gLocks = {};
  signalEvents.forEach((event, index) => {
    const data = event.returnValues;
    // if contract address has been seen (it is in a previously processed signal)
    // then we ignore it; this means that we only acknolwedge the first signal
    // for a given address.
    if (!(data.contractAddr in seenContracts)) {
      seenContracts[data.contractAddr] = true;
      // Get value for each signal event and add it to the collection
      let value;
      // Treat generalized locks as 3 month locks
      if (generalizedLocks.lockedContractAddresses.includes(data.contractAddr)) {
        console.log('Generalized lock:', balances[index], data.contractAddr);
        value = getEffectiveValue(balances[index], '0')
        if (data.edgewareAddr in gLocks) {
          gLocks[data.edgewareAddr] = toBN(gLocks[data.edgewareAddr]).add(value).toString();
        } else {
          gLocks[data.edgewareAddr] = value.toString();
        }
      } else {
        value = getEffectiveValue(balances[index], 'signaling');
      }
      // Add value to total signaled ETH

      totalETHSignaled = totalETHSignaled.add(toBN(balances[index]));
      totalEffectiveETHSignaled = totalEffectiveETHSignaled.add(value);
      // Iterate over signals, partition reward into delayed and immediate amounts
      if (data.edgewareAddr in signals) {
        signals[data.edgewareAddr] = {
          signalAmt: toBN(balances[index]).add(toBN(signals[data.edgewareAddr].signalAmt)).toString(),
          effectiveValue: toBN(signals[data.edgewareAddr]
                                  .effectiveValue)
                                  .add(value)
                                  .toString(),
        };
      } else {
        signals[data.edgewareAddr] = {
          signalAmt: toBN(balances[index]).toString(),
          effectiveValue: value.toString(),
        };
      }
    }
  });
  // Return signals and total ETH signaled
  return { signals, totalETHSignaled, totalEffectiveETHSignaled, generalizedLocks: gLocks }
}

const getLockStorage = async (web3, lockAddress) => {
  return Promise.all([0,1].map(v => {
    return web3.eth.getStorageAt(lockAddress, v);
  }))
  .then(vals => {
    return {
      owner: vals[0],
      unlockTime: hexToNumber(vals[1]),
    };
  });
};

const selectEdgewareValidators = (validatingLocks, totalAllocation, totalEffectiveETH, numOfValidators, existentialBalance=100000000000000) => {
  const sortable = [];
  // Add the calculated edgeware balances with the respective key to a collection
  for (var key in validatingLocks) {
    const keys = key.slice(2).match(/.{1,64}/g).map(key => `0x${key}`);;
    if (keys.length === 3) {
      sortable.push([
        keys,
        toBN(validatingLocks[key].effectiveValue).sub(toBN(existentialBalance)).mul(toBN(totalAllocation)).div(totalEffectiveETH)
      ]);
    }
  }

  // Sort and take the top "numOfValidators" from the collection
  return sortable
    .sort((a,b) => (a[1].lt(b[1])) ? 1 : ((b[1].lt(a[1])) ? -1 : 0))
    .map(v => {
      return ([
        ...v[0].map(k => (k.slice(2))), // stash, controller, session
        v[1].toString(), // staked balance
      ]);
    });
};

const getEdgewareBalanceObjects = (locks, signals, generalizedLocks, totalAllocation, totalEffectiveETH, existentialBalance=100000000000000) => {
  let balances = [];
  let vesting = [];
  let existBalAllocation = mulByAllocationFraction(toBN(existentialBalance), totalAllocation, totalEffectiveETH).toString()
  // handle locks separately than signals at first, then we'll scan over all
  // entries and ensure that there are only unique entries in the collections.
  for (var key in locks) {
    let keys;
    if (key.length === 194) {
      keys = key.slice(2).match(/.{1,64}/g).map(key => `0x${key}`);
      // remove existential balance from this lock for controller account
      if (toBN(locks[key].effectiveValue).lte(toBN(existentialBalance))) {
        console.log('Warning! Validating lock with not enough balance for controller account:', locks[key], key, keys)
      }
      // ensure encodings work
      try {
        const encoded1 = keyring.encodeAddress(keys[0]);
        const encoded2 = keyring.encodeAddress(keys[1]);
        // add entry in for stash account
        balances.push([
          keys[0].slice(2),
          mulByAllocationFraction(toBN(locks[key].effectiveValue).sub(toBN(existentialBalance)), totalAllocation, totalEffectiveETH).toString(),
        ]);
        // add entry in for controller account with minimal existential balance
        balances.push([
          keys[1].slice(2),
          existBalAllocation,
        ])
      } catch(e) {
        console.log(e);
        console.log(`Error processing lock event: ${keys[0]} or ${keys[1]} (${locks[key].effectiveValue})`);
      }
    } else {
      try {
        const encoded = keyring.encodeAddress(key);
        balances.push([
          key.slice(2),
          mulByAllocationFraction(locks[key].effectiveValue, totalAllocation, totalEffectiveETH).toString(),
        ]);
      } catch(e) {
        console.log(e);
        console.log(`Error processing lock event: ${key} (${locks[key].effectiveValue})`);
      }
    }
  }
  // handle signal entries
  for (var key in signals) {
    try {
      let keys = [key];
      // allocate signals to first key if multiple submitted
      if (key.length === 194) {
        keys = key.slice(2).match(/.{1,64}/g).map(key => `0x${key}`);
      }
      const encoded = keyring.encodeAddress(keys[0]);

      if (keys[0] in generalizedLocks) {
        const gValue = generalizedLocks[keys[0]];
        const leftoverValue = toBN(signals[key].effectiveValue).sub(toBN(gValue));
        // add 25% of non-generalised signal value to the liquid amount for the vesting collection
        const vestingValue = toBN(gValue).add(leftoverValue.mul(toBN(25)).div(toBN(100)))
        // create new balance record for the signaler
        balances.push([
          keys[0].slice(2),
          mulByAllocationFraction(toBN(signals[key].effectiveValue), totalAllocation, totalEffectiveETH).toString(),
        ]);
        if (leftoverValue.gt(toBN(0))) {
          // create vesting record
          vesting.push([
            keys[0].slice(2),
            5256000,
            1,
            mulByAllocationFraction(vestingValue, totalAllocation, totalEffectiveETH).toString(),
          ]);
        }
      } else {
        // the liquid amount of the vesting is 25% of signaled value
        const vestingValue = toBN(signals[key].effectiveValue).mul(toBN(25)).div(toBN(100));
        // create new balance record for the signaler
        balances.push([
          keys[0].slice(2),
          mulByAllocationFraction(toBN(signals[key].effectiveValue), totalAllocation, totalEffectiveETH).toString(),
        ]);

        // create vesting record
        vesting.push([
          keys[0].slice(2),
          5256000,
          1,
          mulByAllocationFraction(vestingValue, totalAllocation, totalEffectiveETH).toString(),
        ]);
      }
    } catch(e) {
      console.log(e);
      console.log(`Error processing signal event: ${key} (${signals[key].effectiveValue})`);
    }
  }

  return { balances: balances, vesting: vesting };
};

const combineToUnique = (balances, vesting) => {
  let balancesMap = {};
  let vestingMap = {};
  balances.forEach(entry => {
    let account = entry[0];
    let amount = entry[1];

    if (account in balancesMap) {
      balancesMap[account] = toBN(balancesMap[account]).add(toBN(amount)).toString();
    } else {
      balancesMap[account] = amount
    }
  });

  vesting.forEach(entry => {
    let account = entry[0];
    let amount = entry[3];
    if (account in vestingMap) {
      vestingMap[account] = toBN(vestingMap[account]).add(toBN(amount)).toString();
    } else {
      vestingMap[account] = amount
    }
  });

  let newBalances = []
  let newVesting = [];
  let total = toBN(0);
  Object.keys(balancesMap).forEach(key => {
    total = total.add(toBN(balancesMap[key]));
    newBalances.push([
      key,
      balancesMap[key],
    ]);
  });

  Object.keys(vestingMap).forEach(key => {
    newVesting.push([
      key,
      5256000,
      1,
      vestingMap[key],
    ]);
  });
  console.log(`Balances: ${balances.length}`);
  console.log(`Balances with vesting: ${vesting.length}`);
  console.log(`EDG Total: ${total.toString()}`);
  return { balances: newBalances, vesting: newVesting, total: total };
}

const mulByAllocationFraction = (amount, totalAllocation, totalEffectiveETH) => {
  return toBN(amount).mul(toBN(totalAllocation)).div(toBN(totalEffectiveETH));
}

module.exports = {
  getLocks,
  getSignals,
  getTotalLockedBalance,
  getTotalSignaledBalance,
  calculateEffectiveLocks,
  calculateEffectiveSignals,
  getLockStorage,
  selectEdgewareValidators,
  getEdgewareBalanceObjects,
  combineToUnique,
};
