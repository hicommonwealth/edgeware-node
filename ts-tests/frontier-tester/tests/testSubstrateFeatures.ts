import { ApiPromise, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { createTestPairs, TestKeyringMap } from '@polkadot/keyring/testingPairs';
import { u8aToString } from '@polkadot/util';
import { assert } from 'chai';
import { dev } from '@edgeware/node-types';
import { TypeRegistry } from '@polkadot/types';
import BN from 'bn.js';
import { SubmittableExtrinsic } from '@polkadot/api/types';
const { describeWithEdgeware } = require('../helpers/utils.js');

describeWithEdgeware('Upgrade Tests', async (context) => {
  let api: ApiPromise;
  let pairs: TestKeyringMap;

  before(async () => {
    const polkadotUrl = 'ws://localhost:9944';
    const provider = new WsProvider(polkadotUrl);
    const registry = new TypeRegistry();
    api = await new ApiPromise({
      provider,
      registry,
      ...dev,
    }).isReady;
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
    const charlie = pairs.charlie;
    const dave = pairs.dave;
    const value = api.consts.democracy.minimumDeposit;
    const propCount = await api.query.democracy.publicPropCount();

    // generate an arbitrary tx to propose
    const proposalFunc = api.tx.system.fillBlock(+propCount);

    // submit proposal and verify
    const tx = api.tx.democracy.propose(proposalFunc.method.hash, value);
    await submitTxWithFee(tx, charlie);
    const proposals = await api.query.democracy.publicProps();
    const proposal = proposals.find(([idx, hash, submitter]) => +idx === +propCount
      && hash.toString() === proposalFunc.method.hash.toString()
      && submitter.toString() === charlie.address
    );
    assert.isTrue(!!proposal);

    // submit associated preimage
    const preimageTx = api.tx.democracy.notePreimage(proposalFunc.method.toHex());
    await submitTxWithFee(preimageTx, charlie);
    const preimage = await api.query.democracy.preimages(proposalFunc.method.hash);
    assert.isTrue(preimage.isSome);
    assert.isTrue(preimage.unwrap().isAvailable);
    assert.equal(preimage.unwrap().asAvailable.provider.toString(), charlie.address);

    // second it
    const voters = await api.query.democracy.depositOf(+propCount);
    const secondTx = api.tx.democracy.second(+propCount, voters.unwrap()[0].length);
    await submitTxWithFee(secondTx, dave);
    const newVoters = await api.query.democracy.depositOf(+propCount);
    assert.isTrue(newVoters.isSome);
    assert.isTrue(newVoters.unwrap()[0].map((c) => c.toString()).includes(charlie.address));
    assert.isTrue(newVoters.unwrap()[0].map((c) => c.toString()).includes(dave.address));
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
    const bob = pairs.bob;
    const candidates = await api.query.elections.candidates();
    const submitCandidacyTx = api.tx.elections.submitCandidacy(candidates.length);
    await submitTxWithFee(submitCandidacyTx, bob);
    const newCandidates = await api.query.elections.candidates();
    assert.isTrue(newCandidates.map((c) => c.toString()).includes(bob.address));
  });

  it('should register an identity', async () => {
    const bob = pairs.bob;
    const name = 'i am bob';
    const identityInfo = api.createType('IdentityInfo', {
      additional: [],
      display: { raw: name },
      legal: { none: null },
      web: { none: null },
      riot: { none: null },
      email: { none: null },
      image: { none: null },
      twitter: { none: null },
    });
    const tx = api.tx.identity.setIdentity(identityInfo)
    await submitTxWithFee(tx, bob);
    const identity = await api.query.identity.identityOf(bob.address);
    assert.isTrue(identity.isSome);
    assert.equal(u8aToString(identity.unwrap().info.display.toU8a()).replace(/[^\x20-\x7E]/g, ''), name);
  });

/*
  it('should deploy WASM', async () => {
    // deploy a contract
    const eve = pairs.eve;
    const json = fs.readFileSync(`${__dirname}/../helpers/flipper.json`, { encoding: 'utf8' });
    const wasm = fs.readFileSync(`${__dirname}/../helpers/flipper.wasm`);

    // create the contract blueprint
    const code = new CodePromise(api, json, wasm);
    const blueprint: BlueprintPromise = await new Promise((resolve) => {
      code.createBlueprint().signAndSend(eve, (res) => {
        if (res.isCompleted) {
          resolve(res.blueprint);
        }
      });
    });
    assert.isTrue(!!blueprint);

    // deploy a contract
    const value = new BN('1230000000000');
    const gasLimit = new BN('100000000000');
    const contract: ContractPromise = await new Promise((resolve, reject) => {
      blueprint.createContract('new', value, gasLimit, true).signAndSend(eve, (res) => {
        if (res.dispatchError) {
          if (res.dispatchError.isModule) {
            const details = api.registry.findMetaError(res.dispatchError.asModule.toU8a());
            console.error(`${details.section}::${details.name}: ${details.documentation[0]}`);
          }
          reject(res.dispatchError);
        } else if (res.isCompleted) {
          resolve(res.contract);
        }
      })
    });
  
    // verify the contract flipped the value
    const getResult = await contract.query.get(eve.address, value, gasLimit);
    assert.isTrue(getResult.result.isOk);
    assert.isTrue((getResult.result.value as bool).isFalse);
  });
*/
});
