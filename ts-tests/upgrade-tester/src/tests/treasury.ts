import { ApiPromise } from '@polkadot/api';
import { u8aToString } from '@polkadot/util';
import { TreasuryProposal } from '@polkadot/types/interfaces';
import chai from 'chai';
import StateTest from '../stateTest';

export default class extends StateTest {
  private _proposal: TreasuryProposal;

  constructor() {
    super('identity test');
  }

  public async before(api: ApiPromise) {
    // create a treausury proposal
    // TODO: handle errors
    this._proposal = await new Promise<TreasuryProposal>((resolve, reject) => {
      api.tx.treasury.proposeSpend('1000000000000', this.accounts.bob.address)
        .signAndSend(this.accounts.alice, async (status) => {
          if (status.isCompleted) {
            const proposalCount = await api.query.treasury.proposalCount();
            const p = await api.query.treasury.proposals(+proposalCount - 1);
            if (!p.isSome) {
              reject(new Error('treasury proposal not found'));
            } else {
              resolve(p.unwrap());
            }
          } else if (status.isError) {
            reject(new Error('got tx error on proposeSpend'));
          }
        });
    });
    if (!this._proposal) {
      throw new Error('unable to fetch treasury proposal');
    }
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    if (!this._proposal) {
      throw new Error('saved proposal not found');
    }

    // query the proposal
    const proposalCount = await api.query.treasury.proposalCount();
    const p = await api.query.treasury.proposals(+proposalCount - 1);
    chai.assert.deepEqual(
      this._proposal.toHuman(),
      p.toHuman(),
      'treasury proposal should not change',
    );
    await super.after(api);
  }
}
