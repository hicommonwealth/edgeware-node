import { ApiPromise } from '@polkadot/api';
import chai from 'chai';
import StateTest from '../stateTest';

class BalanceQueryTest extends StateTest {
  private _bal: string;

  constructor(accountSeeds: string[], ss58Prefix: number) {
    super('Balance Query Test', accountSeeds, ss58Prefix);
    if (accountSeeds.length === 0) throw new Error(`${this.name} requires at least one account!`);
  }

  public async before(api: ApiPromise) {
    const bal = await api.query.balances.account(this.account(0));
    this._bal = JSON.stringify(bal);
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    const bal = await api.query.balances.account(this.account(0));
    chai.assert.equal(this._bal, JSON.stringify(bal));
    await super.after(api);
  }
}

export default BalanceQueryTest;
