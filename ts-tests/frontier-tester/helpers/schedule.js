const { toBN, toWei } = require('web3').utils;

const getEarlyParticipationBonus = (lockTime, lockStart) => {
  const JUNE_1ST_UTC = 1559347200;
  const JUNE_16TH_UTC = 1560643200;
  const JULY_1ST_UTC = 1561939200;
  const JULY_16TH_UTC = 1563235200;
  const JULY_31ST_UTC = 1564531200;
  const AUG_15TH_UTC = 1565827200;
  const AUG_30TH_UTC = 1567123200;

  if (toBN(lockTime).lte(toBN(JUNE_16TH_UTC))) {
    return toBN(150);
  } else if (toBN(lockTime).lte(toBN(JULY_1ST_UTC))) {
    return toBN(135);
  } else if (toBN(lockTime).lte(toBN(JULY_16TH_UTC))) {
    return toBN(123);
  } else if (toBN(lockTime).lte(toBN(JULY_31ST_UTC))) {
    return toBN(114);
  } else if (toBN(lockTime).lte(toBN(AUG_15TH_UTC))) {
    return toBN(108);
  } else if (toBN(lockTime).lte(toBN(AUG_30TH_UTC))) {
    return toBN(105);
  } else {
    return toBN(100);
  }
};

module.exports = {
  getEarlyParticipationBonus,
}
