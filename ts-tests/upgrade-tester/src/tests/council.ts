import { ApiPromise } from '@polkadot/api';
// augment API
import '@polkadot/api-augment';
import { BalanceOf, AccountId, Voter } from '@polkadot/types/interfaces';
import { Vec } from '@polkadot/types';
import { ITuple } from '@polkadot/types/types';
import { assert } from 'chai';
import StateTest from '../stateTest';
import { makeTx } from '../util';

export default class extends StateTest {
  private _candidates: string[];
  private _council: string[];
  private _charlieVotes: Voter;

  constructor() {
    super('council test');
  }

  public async before(api: ApiPromise) {
    // have charlie run for council
    const startCandidates = await api.query.elections.candidates();
    await makeTx(api, api.tx.elections.submitCandidacy(startCandidates.length), this.accounts.charlie);
    this._candidates = (await api.query.elections.candidates()).map((c) => c.toString());

    // have dave vote for charlie
    await makeTx(
      api,
      api.tx.elections.vote([ this.accounts.charlie.address ], api.consts.elections.candidacyBond),
      this.accounts.dave
    );
    this._charlieVotes = await api.query.elections.voting(this.accounts.charlie.address);

    // save council status
    this._council = (await api.query.council.members()).map((c) => c.toString());
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    // confirm candidates unchanged
    const candidates = (await api.query.elections.candidates()).map((c) => c[0].toString());
    assert.sameMembers(candidates, this._candidates);

    // confirm vote unchanged
    const charlieVotes = await api.query.elections.voting(this.accounts.charlie.address);
    console.log(charlieVotes.toHuman(), this._charlieVotes.toHuman());
    assert.equal(+charlieVotes.deposit, +this._charlieVotes[0]);
    assert.sameMembers(charlieVotes.votes.map((c) => c.toString()), this._charlieVotes[1].map((c) => c.toString()));

    // confirm council unchanged
    const council = (await api.query.council.members()).map((c) => c.toString());
    console.log('Council', council, this._council);
    assert.sameMembers(council, this._council);
    await super.after(api);
  }
}
