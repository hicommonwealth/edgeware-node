import { ApiPromise } from '@polkadot/api';
import { TreasuryProposal } from '@polkadot/types/interfaces';
import chai from 'chai';
import StateTest from '../stateTest';
import { makeTx } from './util';

export default class extends StateTest {
  private _proposal: TreasuryProposal;

  constructor() {
    super('treasury test');
  }

  public async before(api: ApiPromise) {
    // create a treasury proposal
    await makeTx(api.tx.treasury.proposeSpend('1000000000000', this.accounts.bob.address), this.accounts.alice);
    const proposalCount = await api.query.treasury.proposalCount();
    const p = await api.query.treasury.proposals(+proposalCount - 1);
    if (!p.isSome) {
      throw new Error('treasury proposal not found');
    }

    this._proposal = p.unwrap();
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
