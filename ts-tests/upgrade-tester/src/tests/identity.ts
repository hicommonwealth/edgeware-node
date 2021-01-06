import { ApiPromise } from '@polkadot/api';
import { u8aToString } from '@polkadot/util';
import { IdentityInfo } from '@polkadot/types/interfaces';
import chai from 'chai';
import StateTest from '../stateTest';
import { makeTx } from '../util';

export default class extends StateTest {
  private _identity: IdentityInfo;

  constructor() {
    super('identity test');
  }

  public async before(api: ApiPromise) {
    // register an identity
    const identityInfo = api.createType('IdentityInfo', {
      additional: [],
      display: { raw: 'i am test' },
      legal: { none: null },
      web: { none: null },
      riot: { none: null },
      email: { none: null },
      image: { none: null },
      twitter: { none: null },
    });

    await makeTx(api, api.tx.identity.setIdentity(identityInfo), this.accounts.alice);
    const registration = await api.query.identity.identityOf(this.accounts.alice.address);
    if (!registration.isSome) {
      throw new Error('identity registration not found');
    }

    this._identity = registration.unwrap().info;
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    if (!this._identity) {
      throw new Error('stored identity not found');
    }

    // query the identity
    const registration = await api.query.identity.identityOf(this.accounts.alice.address);
    if (!registration.isSome) {
      throw new Error('identity registration not found');
    }

    chai.assert.deepEqual(
      registration.unwrap().info.toHuman(),
      this._identity.toHuman(),
      'identity should not change',
    );
    await super.after(api);
  }
}
