import { ApiPromise } from '@polkadot/api';
import { ITuple } from '@polkadot/types/types';
import { PropIndex, Hash, AccountId, PreimageStatusAvailable } from '@polkadot/types/interfaces';
import chai from 'chai';
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
    const call = api.tx.system.fillBlock(1);
    await makeTx(api.tx.democracy.propose(call.hash, api.consts.democracy.minimumDeposit), this.accounts.alice);

    const proposals = await api.query.democracy.publicProps();
    chai.assert.lengthOf(proposals, 1, 'proposal should be in publicProps array');
    this._proposal = proposals[0];

    // submit the preimage
    await makeTx(api.tx.democracy.notePreimage(call.toHex()), this.accounts.alice);
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
    const proposals = await api.query.democracy.publicProps();
    chai.assert.lengthOf(proposals, 1);
    chai.assert.deepEqual(
      proposals[0].toHuman(),
      this._proposal.toHuman(),
      'democracy proposal should be identical',
    );

    // check the preimage
    const call = api.tx.system.fillBlock(1);
    const preimage = await api.query.democracy.preimages(call.hash);
    if (!preimage.isSome || !preimage.unwrap().isAvailable) {
      throw new Error('preimage not found');
    }

    chai.assert.deepEqual(
      preimage.unwrap().asAvailable.toHuman(),
      this._preimage.toHuman(),
      'preimage should be identical',
    );
    await super.after(api);
  }
}
