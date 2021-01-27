// TODO:
// 1. Check storage items before/after upgrade to see if any have changed
// 2. Check module numbering to ensure none have changed

import { ApiPromise } from '@polkadot/api';
import { HttpProvider } from '@polkadot/rpc-provider';
import { xxhashAsHex } from '@polkadot/util-crypto';
import { assert } from 'chai';
import _ from 'underscore';
import StateTest from '../stateTest';

type StoragePrefixMap = { [sec: string]: { name: string, mods: { [mod: string ]: string } } };

export default class extends StateTest {
  private _pairs: { [k: string]: string };
  private _storageKeys: StoragePrefixMap;
  private _modules: { [name: string]: number };

  constructor() {
    super('storage test');
  }

  private async _fetchPairs(): Promise<{ [k: string]: string }> {
    // Using http endpoint since substrate's Ws endpoint has a size limit.
    // note that this is a "hack" that will only work locally
    const provider = new HttpProvider('http://localhost:9933');

    // fetch storage items and module numbers
    const pairs: [string, string][] = await provider.send('state_getPairs', ['0x']);
    await provider.disconnect();
    const pairsObj = _.object(pairs);

    // remove manually modified changes (see: forker.ts)
    // runtime code
    delete pairsObj['0x3a636f6465'];
    // Staking.ForceEra
    delete pairsObj['0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3'];
    // System.LastRuntimeUpgrade
    delete pairsObj['0x26aa394eea5630e07c48ae0c9558cef7f9cce9c888469bb1a0dceaa129672ef8'];
    const sudoKeyHash = xxhashAsHex('Sudo', 128) + xxhashAsHex('Key', 128).slice(2);
    delete pairsObj[sudoKeyHash];
    const randomMaterialHash = xxhashAsHex('RandomnessCollectiveFlip', 128)
      + xxhashAsHex('RandomMaterial', 128).slice(2);
    delete pairsObj[randomMaterialHash];
    return pairsObj;
  }

  private async _genStorageKeys(api: ApiPromise): Promise<StoragePrefixMap> {
    const metadata = await api.rpc.state.getMetadata();
    const modules = metadata.asLatest.modules;
    const keys = {};
    for (const m of modules) {
      if (m.storage.isSome) {
        const storage = m.storage.unwrap();
        const prefix = storage.prefix;
        const prefixKey = xxhashAsHex(prefix.toString(), 128);
        keys[prefixKey] = { name: prefix, mods: { } };
        for (const { name } of storage.items) {
          const hash = xxhashAsHex(name.toString(), 128).slice(2);
          keys[prefixKey].mods[hash] = name.toString();
        }
      }
    }
    return keys;
  }

  private _lookupStorageName(storageMap: StoragePrefixMap, key: string): string {
    if (key.length < 66) return key;
    // e.g. 0xf68f425cf5645aacb2ae59b51baed90420d49a14a763e1cbc887acd097f92014
    const prefixKey = key.slice(0, 34);
    const modKey = key.slice(34, 66);
    const suffix = key.slice(66);
    const res = storageMap[prefixKey];
    if (!res) return key;
    const { name, mods } = res;
    const modRes = mods[modKey];
    if (!modRes) return `${name}.${modKey}${suffix ? '.' : ''}${suffix}`;
    else return `${name}.${modRes}${suffix ? '.' : ''}${suffix}`;
  }

  private async _fetchModules(api: ApiPromise): Promise<{ [name: string]: number }> {
    const metadata = await api.rpc.state.getMetadata();
    const modules = {};
    for (const m of metadata.asLatest.modules) {
      modules[m.name.toString()] = +m.index;
    }
    return modules;
  }

  public async before(api: ApiPromise) {
    this._pairs = await this._fetchPairs();
    this._modules = await this._fetchModules(api);
    this._storageKeys = await this._genStorageKeys(api);
    await super.before(api);
  }

  public async after(api: ApiPromise) {
    // most pairs should not change before/after upgrade unless a migration was performed
    const newPairs = await this._fetchPairs();
    for (const k of Object.keys(this._pairs)) {
      if (!newPairs[k]) {
        const name = this._lookupStorageName(this._storageKeys, k);
        console.log(`Item removed from storage: ${name} = ${this._pairs[k]}`);
      }
    }

    const newStorageKeys = await this._genStorageKeys(api);
    for (const k of Object.keys(newPairs)) {
      const name = this._lookupStorageName(newStorageKeys, k);
      if (!this._pairs[k]) {
        console.log(`New item in storage: ${name} = ${newPairs[k]}`);
      } else if (this._pairs[k] !== newPairs[k]) {
        console.log(`Changed item in storage: ${name}`);
        console.log(`\tOld: ${this._pairs[k]}`);
        console.log(`\tNew: ${newPairs[k]}`);
      }
    }

    // all common keys of new modules should be the same
    const newModules = await this._fetchModules(api);
    for (const key of Object.keys(this._modules)) {
      if (newModules[key] === undefined) {
        console.log(`Module removed: ${key}:${this._modules[key]}.`);
      }
    }
    for (const key of Object.keys(newModules)) {
      if (this._modules[key] !== undefined) {
        assert.equal(newModules[key], this._modules[key]);
      } else {
        console.log(`Module added: ${key}:${newModules[key]}.`);
      }
    }

    // compare updated storage items and module numbers to verify they're the same
    await super.after(api);
  }
}
