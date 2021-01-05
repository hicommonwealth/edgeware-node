import { ApiPromise } from '@polkadot/api';
import chai from 'chai';
import StateTest from '../stateTest';
export default class extends StateTest {
  private _bal: any;

  constructor() {
    super('balance query test');
  }

  public async before(api: ApiPromise) {
    const bal = await api.query.system.account(this.accounts.eve.address);
    chai.assert.isTrue(bal.data.free.gtn(0), 'eve should have balance');
    this._bal = bal.data.toHuman();
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    const bal = await api.query.system.account(this.accounts.eve.address);
    console.log(JSON.stringify(this._bal), JSON.stringify(bal.data.toHuman()));
    chai.assert.deepEqual(this._bal, bal.data.toHuman(), 'eve balance should not change');
    await super.after(api);
  }
}
