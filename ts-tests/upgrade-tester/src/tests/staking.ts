import { ApiPromise } from '@polkadot/api';
import { AccountData, Exposure } from '@polkadot/types/interfaces';
import { assert } from 'chai';
import StateTest from '../stateTest';
import { makeTx } from '../util';

export default class extends StateTest {
  private _stashBal: AccountData;
  private _controllerBal: AccountData;
  private _validator: string;
  private _exposure: Exposure;

  constructor() {
    super('staking test');
  }

  public async before(api: ApiPromise) {
    // bob should have a balance to start with
    const startingStashBal = await api.query.system.account(this.accounts.bob_stash.address);
    assert.isTrue(startingStashBal.data.free.gtn(0), 'bob stash should have balance');
    const startingFree = startingStashBal.data.free;

    // bob should bond half using its stash
    await makeTx(api, api.tx.staking.bond(
      this.accounts.bob.address,
      startingFree.divn(2),
      { Staked: true } // reinvest returns
    ), this.accounts.bob_stash);

    // nominate all validators from controller (should only be alice)
    const validators = await api.query.session.validators();
    this._validator = validators[0].toString();
    await makeTx(api, api.tx.staking.nominate(validators), this.accounts.bob);

    // save balances and staking configurations to test against after upgrade
    const stashBal = await api.query.system.account(this.accounts.bob_stash.address);
    const controllerBal = await api.query.system.account(this.accounts.bob.address);
    this._stashBal = stashBal.data;
    this._controllerBal = controllerBal.data;

    const era = await api.query.staking.currentEra();
    this._exposure = await api.query.staking.erasStakers(era.unwrap(), this._validator);

    await super.before(api);
  }

  public async after(api: ApiPromise) {
    // fetch bonding state and balances after upgrade
    const controllerAddress = await api.query.staking.bonded(this.accounts.bob_stash.address);
    assert.equal(controllerAddress.toString(), this.accounts.bob.address);
    const stashBal = await api.query.system.account(this.accounts.bob_stash.address);
    const controllerBal = await api.query.system.account(this.accounts.bob.address);
    assert.deepEqual(stashBal.data.toHuman(), this._stashBal.toHuman());
    assert.deepEqual(controllerBal.data.toHuman(), this._controllerBal.toHuman());

    // fetch validators and exposure
    const validators = await api.query.session.validators();
    assert.sameMembers(validators.map((v) => v.toString()), [ this._validator ]);
    const era = await api.query.staking.currentEra();
    const exposure = await api.query.staking.erasStakers(era.unwrap(), this._validator);
    assert.deepEqual(exposure.toHuman(), this._exposure.toHuman());

    await super.after(api);
  }
}
