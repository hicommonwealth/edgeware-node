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
    chai.assert.isTrue(bal.data.free.gtn(0), 'alice should have balance');
    this._bal = bal.data.toHuman();
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    const bal = await api.query.system.account(this.accounts.eve.address);
    chai.assert.deepEqual(this._bal, bal.data.toHuman(), 'alice balance should not change');
    await super.after(api);
  }
}
