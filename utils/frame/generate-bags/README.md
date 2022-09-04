# node-runtime-generate-bags

This tool shall be used to generate the voter bags for staking and elections.
Those bags should ideally never need to be modified, unless the issuance speed changes dramatically, or the currency is re-denominated with a different rebasing, and/or a different existential deposit.

The command used to generate the current file is:

```
./target/release/node-runtime-generate-bags --total-issuance 10000000000000000000000000000 --minimum-balance 10000000000000 voter_bags.rs
```

It assumes a total issuance of 10B tokens. It should be enough to sustain many years of growth as is it meant to accomodate the highest amount nominated by a given nominator so the total issuance is already extremely conservative.

Because of the wide range of Edgeware balances (order of 10B tokens issued, with a 10 Millicents existential deposit), the range of the bags is sparser than on some other chains and may fail to adequately favour higher stakes in some cases as the nominators are ordered in a given bag in their insertion order and not by stake. If this is a problem, one possibility could be to add more bags (decreasing the performance) or to introduce a minimal amount for staking higher than the existential deposit.