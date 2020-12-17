import { ApiPromise, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { createTestPairs, TestKeyringMap } from '@polkadot/keyring/testingPairs';
import { assert } from 'chai';
import { dev } from '@edgeware/node-types';
import { TypeRegistry } from '@polkadot/types';
import BN from 'bn.js';
import { SubmittableExtrinsic } from '@polkadot/api/types';

describe('Upgrade Tests', async () => {
  let api: ApiPromise;
  let pairs: TestKeyringMap;

  before(async () => {
    const polkadotUrl = 'ws://localhost:9944';
    const registry = new TypeRegistry();
    api = await (new ApiPromise({
      provider: new WsProvider(polkadotUrl),
      registry,
      ...dev,
    })).isReady;
    pairs = createTestPairs();
  });

  const submitTxWithFee = async (tx: SubmittableExtrinsic<'promise'>, from: KeyringPair): Promise<BN> => {
    return new Promise(async (resolve) => {
      const { partialFee } = await tx.paymentInfo(from);
      tx.signAndSend(from, (result) => {
        if (result.isError) {
          assert.fail('tx failure');
        }
        if (result.isCompleted) {
          resolve(partialFee);
        }
      });
    })
  }

  const fetchBalance = async (acct: string): Promise<BN> => {
    const res = await api.query.system.account(acct);
    return res.data.free;
  }

  it('should have correct chain information', async () => {
    // TODO: fetch chain params and verify against node types/known values
  });

  it('should transfer balances', async () => {
    const charlie = pairs.charlie;
    const dave = pairs.dave;
    const charlieStartBal = await fetchBalance(charlie.address);
    const daveStartBal = await fetchBalance(dave.address);

    // send funds from charlie to dave
    const value = new BN('10000000000000000000');
    const tx = api.tx.balances.transfer(dave.address, value);
    const fees = await submitTxWithFee(tx, charlie);

    // verify results
    const charlieEndBal = await fetchBalance(charlie.address);
    const daveEndBal = await fetchBalance(dave.address);
    assert.equal(daveStartBal.add(value).toString(), daveEndBal.toString());
    assert.equal((charlieStartBal.sub(value)).sub(fees).toString(), charlieEndBal.toString());
  });

  it('should create and second democracy proposal', async () => {

  });

  it('should create treasury proposal', async () => {
    // setup args
    const bob = pairs.bob;
    const startBal = await fetchBalance(bob.address);
    const value = new BN('10000000000000000000');
    const beneficiary = pairs.alice.address;
    const bondPermill = api.consts.treasury.proposalBond;
    const bondMinimum = api.consts.treasury.proposalBondMinimum;
    const bondFromPct = value.mul(bondPermill).divn(1_000_000);
    const bond = BN.max(bondFromPct, bondMinimum);

    // make transaction
    const tx = api.tx.treasury.proposeSpend(value, beneficiary);
    const fee = await submitTxWithFee(tx, bob);

    // fetch result on success
    const endBal = await fetchBalance(bob.address);
    const proposalCount = await api.query.treasury.proposalCount();
    const proposal = await api.query.treasury.proposals(proposalCount.subn(1));

    // verify results
    assert.equal((startBal.sub(bond)).sub(fee).toString(), endBal.toString());
    assert.isTrue(proposal.isSome);
    assert.equal(proposal.unwrap().beneficiary.toString(), beneficiary);
    assert.equal(proposal.unwrap().value.toString(), value.toString());
    assert.equal(proposal.unwrap().proposer.toString(), bob.address);
    assert.equal(proposal.unwrap().bond.toString(), bond.toString());
  });

  it('should apply for council', async () => {

  });

  it('should register an identity', async () => {

  });

  it('should deploy WASM', async () => {

  });
});
