import { ApiPromise } from '@polkadot/api';
import { u8aToString } from '@polkadot/util';
import chai from 'chai';
import StateTest from '../stateTest';

export default class extends StateTest {
  private _name: string;

  constructor() {
    super('identity test');
  }

  public async before(api: ApiPromise) {
    // register an identity
    this._name = 'i am test';
    const identityInfo = api.createType('IdentityInfo', {
      additional: [],
      display: { raw: this._name },
      legal: { none: null },
      web: { none: null },
      riot: { none: null },
      email: { none: null },
      image: { none: null },
      twitter: { none: null },
    });

    await new Promise<void>((resolve, reject) => {
      api.tx.identity.setIdentity(identityInfo)
        .signAndSend(this.accounts.alice, (status) => {
          if (status.isCompleted) {
            resolve();
          } else if (status.isError) {
            reject(new Error('got tx error for setIdentity'));
          }
        });
    });
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    // query the identity
    const identity = await api.query.identity.identityOf(this.accounts.alice.address);
    chai.assert.equal(
      u8aToString(identity.unwrap().info.display.toU8a()).replace(/[^\x20-\x7E]/g, ''),
      this._name,
      'identity should not change',
    );
    await super.after(api);
  }
}
