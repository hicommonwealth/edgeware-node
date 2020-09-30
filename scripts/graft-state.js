// Edgeware pallet prefixes
const PREFIXES = [
    '26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9', /* System.Account */
    'bd2a529379475088d3e29a918cd47872', /* RandomnessCollectiveFlip */
    'f0c365c3cf59d671eb72da0e7a4113c4', /* Timestamp */
    '1a736d37504c2e3fb73dad160c55b291', /* Indices */
    'c2261276cc9d1f8598ea4b6a74b15c2f', /* Balances */
    '3f1467a096bcd71a5b6a0c8155e20810', /* TransactionPayment */
    'd57bce545fb382c34570e5dfbf338f5e', /* Authorship */
    // '5f3e4907f716ac89b6347d15ececedca', /* Staking */
    'd5c41b52a371aa36c9254ce34324f2a5', /* Offences */
    'cec5070d609dd3497f72bde07fc96ba0', /* Session */
    '2b06af9719ac64d755623cda8ddd9b94', /* ImOnline */
    '2099d7f109d6e535fb000bba623fd440', /* AuthorityDiscovery */
    'f2794c22e353e9a839f12faab03a911b', /* Democracy */
    '11f3ba2e1cdd6d62f2ff9b5589e7ff81', /* Instance1Collective */
    '8985776095addd4789fccbce8ca77b23', /* Instance2Collective */
    'e2e62dd81c48a88f73b6f6463555fd8e', /* PhragmenElection */
    '492a52699edf49c972c21db794cfcf57', /* Instance1Membership */
    '89d139e01a5eb2256f222e5fc5dbe6b3', /* Treasury */
    '9c5d795d0297be56027a4b2464e33397', /* Claims */
    'ae394d879ddf7f99595bc0dd36e355b5', /* Attestations */
    '6ac983d82528bf1595ab26438ae5b2cf', /* Slots */
    '3fba98689ebed1138735e0e7a5a790ab', /* Registrar */
    'd5e1a2fa16732ce6906189438c0a82c6', /* Utility */
    '2aeddc77fe58c98d50bd37f1b90840f9', /* Identity */
    '426e15054d267946093858132eb537f1', /* Society */
    'a2ce73642c549ae79c14f0a671cf45f9', /* Recovery */
    '5f27b51b5ec208ee9cb25b55d8728243', /* Vesting */
    '6843380eacef5618df465286e5652580', /* Signaling */
    'ee8f907cea35735e56a7613a005989a0', /* TreasuryReward */
    '71cd3068e6118bfb392b798317f63a89', /* Voting */
    '1da53b775b270400e7e61ed5cbc5a146', /* EVM */
];

// read and write json specs
const fs = require('fs');
function loadSpec(path) {
  return JSON.parse(fs.readFileSync(path, 'utf8'));
}
function writeSpec(path, spec) {
  fs.writeFile(path, JSON.stringify(spec, null, 4), () => {});
}

// default to live and dev chainspecs in the repo
console.log("usage: node scripts/graft-state.js /path/to/live.json /path/to/dev.json /path/for/output.json");
let livePath = process.argv[2] || './chains/live-state.chainspec.json';
let oldDevPath = process.argv[3] || './chains/old-dev.chainspec.json';
let newSpecPath = process.argv[4] || './chains/hybrid-mainnet-dev.json';
// load chain specs
let edgeware = loadSpec(livePath);
let spec = loadSpec(oldDevPath);
// adjust name and ids (for the UI)
spec.name = edgeware.name;
spec.id = edgeware.id;
spec.protocolId = edgeware.protocolId;

// TODO migration flags and or storage versions
// graft data under chosen prefixes into the new spec
Object.keys(edgeware.genesis.raw.top).filter(key => PREFIXES.some(prefix => key.startsWith('0x'+prefix))).forEach(key => spec.genesis.raw.top[key] = edgeware.genesis.raw.top[key]);
// replace dev code with edgeware code
const CODE_HASH = '0x3a636f6465';
spec.genesis.raw.top[CODE_HASH] = edgeware.genesis.raw.top[CODE_HASH];
// set ForceEra to None to keep producing blocks
const StakingForceEra = '0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3';
const ForceNone = '0x02';
spec.genesis.raw.top[StakingForceEra] = ForceNone;
// delete System.LastRuntimeUpgrade
const SystemLastRuntimeUpgrade = '0x26aa394eea5630e07c48ae0c9558cef7f9cce9c888469bb1a0dceaa129672ef8';
delete spec.genesis.raw.top[SystemLastRuntimeUpgrade];
// delete old `childen` entry and replace it with new `childrenDefault`
delete spec.genesis.raw.children;
spec.genesis.raw.childrenDefault = edgeware.genesis.raw.childrenDefault;

// write out the new spec
writeSpec(newSpecPath, spec);
console.log(`Done. Wrote new spec to '${newSpecPath}'.`);