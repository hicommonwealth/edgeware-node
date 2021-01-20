import { ApiPromise } from '@polkadot/api';
import { ITuple } from '@polkadot/types/types';
import { PropIndex, Hash, AccountId, PreimageStatusAvailable } from '@polkadot/types/interfaces';
import { assert } from 'chai';
import StateTest from '../stateTest';
import { makeTx } from '../util';

export default class extends StateTest {
  private _proposal: ITuple<[PropIndex, Hash, AccountId]>;
  private _preimage: PreimageStatusAvailable;

  constructor() {
    super('democracy test');
  }

  public async before(api: ApiPromise) {
    // create a democracy proposal
    const nProposals = await api.query.democracy.publicPropCount();
    const call = api.tx.system.fillBlock(1);
    await makeTx(api, api.tx.democracy.propose(call.hash, api.consts.democracy.minimumDeposit), this.accounts.alice);

    const proposals = await api.query.democracy.publicProps();
    assert.isAbove(proposals.length, 0, 'publicProps array should have proposals');
    this._proposal = proposals.find(([ idx ]) => +idx === +nProposals);
    assert.exists(this._proposal, 'proposal must exist');

    // submit the preimage
    await makeTx(api, api.tx.democracy.notePreimage(call.toHex()), this.accounts.alice);
    const preimage = await api.query.democracy.preimages(call.hash);
    if (!preimage.isSome || !preimage.unwrap().isAvailable) {
      throw new Error('preimage not found');
    }

    this._preimage = preimage.unwrap().asAvailable;
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    if (!this._proposal) {
      throw new Error('saved proposal not found');
    }
    if (!this._preimage) {
      throw new Error('saved preimage not found');
    }

    // check the proposal
    const nProposals = await api.query.democracy.publicPropCount();
    const proposals = await api.query.democracy.publicProps();
    assert.lengthOf(proposals, 1, 'proposals should still exist');
    const proposal = proposals.find(([ idx ]) => +idx === (+nProposals - 1));
    assert.exists(proposal, 'proposal must exist');
    assert.deepEqual(
      proposal.toHuman(),
      this._proposal.toHuman(),
      'democracy proposal should be identical',
    );
    const proposalId = proposal[0];
    const deposits = await api.query.democracy.depositOf(proposalId);
    assert.isTrue(deposits.isSome);
    const [ voters, balance ] = deposits.unwrap();

    // check the preimage
    const call = api.tx.system.fillBlock(1);
    const preimage = await api.query.democracy.preimages(call.hash);
    if (!preimage.isSome || !preimage.unwrap().isAvailable) {
      throw new Error('preimage not found');
    }

    assert.deepEqual(
      preimage.unwrap().asAvailable.toHuman(),
      this._preimage.toHuman(),
      'preimage should be identical',
    );

    // attempt to second the proposal and verify it works
    await makeTx(api, api.tx.democracy.second(proposalId, 5), this.accounts.bob);
    const updatedDeposits = await api.query.democracy.depositOf(proposalId);
    assert.isTrue(updatedDeposits.isSome, 'should find deposits for proposal');
    const [ updatedVoters, updatedBalance ] = updatedDeposits.unwrap();
    assert.equal(+balance, +updatedBalance, 'proposal deposit should not change');
    assert.sameMembers(
      updatedVoters.map((v) => v.toString()),
      [ ...voters.map((v) => v.toString()), this.accounts.bob.address ],
      'proposal seconders should match'
    );

    await super.after(api);
  }
}
