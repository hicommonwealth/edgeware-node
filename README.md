# edgeware-node - Luke's version to run a validator using Linode

Note: I have removed parts of the original Readme and only left the parts that I need.
If you're a visitor to this repo, please refer to the original repo too first so you're not missing out on anything important.

## TODO

* Subkey supports password-protecting the keys now https://substrate.dev/docs/en/next/ecosystem/subkey#password-protected-keys, which wasn't available a few months ago, so early adopters of Edgware keys may wish to transfer their funds to another account that has password protection after Edgeware activates of token transfers- https://substrate.dev/docs/en/next/ecosystem/subkey 
* Unlock tool to unlock your locked ETH https://commonwealth.im/#!/unlock
* Blogpost on how to be an Edgeware beta-net and mainnet Validator https://commonwealth.im/#!/edgeware/proposal/discussion/20

## Setup Validator

### Reminder about security for validators
* Note: For Mainnet, setup additional safeguards such as cloud HSM that cost ~USD$5k upfront and ~USD$1.5k per month when significant amount to protect on keys (to avoid handing in keys and having DDOS protection)

### Setup account and get testnet EDG

* Run `./scripts/keygen.sh` to generate an account address to be used on the testnet.
You'll need to use that private session key with the `--key` (in the Dockerfile) to run a node and possibly be chosen as a validator through the on-chain bonding system (depends on how much is bonded and nominated to your address). See how you can run an RPC call below too.

  * Note: In the Genesis config, check if only Commonwealth authorities have the session keys configured https://github.com/hicommonwealth/edgeware-node/blob/master/node/service/src/chain_spec.rs#L78

* Ask someone from Edgware in Discord for Testnet EDG tokens for the Stash and Controller accounts that you generated, and provide them with you testnet address (NOT your seed or mnemonic)
  * Or try and request from https://faucets.blockxlabs.com/edgeware

### Create a Basic Linode Instance

Create a [Linode account](https://www.linode.com/?r=4dbc9d2dfa5ba217a93e48d74a5b230eb5810cc0)

Create Linode instance in Linode Manager
* Select Linode 4GB instance (2 CPU)
* Select node location - i.e. Singapore
* Click Create

Deploy an Image
* Go to "Dashboard" of Linode instance
* Click Deploy an Image
* Select Ubuntu 18.04 LTS or Debian 10
* Select Disk 80000 MB (note that 12 GB is insufficient)
* Select Swap Disk 512 MB (you'll reduce your disk space and use it to increase your swap later)

Boot Image
* Go to "Dashboard" of Linode instance
* Click "Boot"

### Increas Swap Space

https://www.linode.com/community/questions/9449/swap-resize-via-linode-manager

* Below assumes you're on the Linode Standard 4GB Plan, which has swap set to 512MB by default.
We're going to increase Swap space to 16 GB (should be at least 8 GB for a validator)
* Power off linode instance (so it's not running), go to "Advanced" section called "Disks"
  * Click the triple-dot icon next to "Debian 10 Disk" 81408 MB, 
    * Change to 81408 - 16384 = 65024
    * Wait for the size to update
    * Click the triple-dot icon next to "512 MB Swap Image", choose "Resize", change from 512 to 16896
* Power On the Linode again, enter the following when you're SSHed in again to see that it increased `cat /proc/swaps`
```
root@localhost:~# cat /proc/swaps
Filename				Type		Size	Used	Priority
/dev/sdb                                partition	17301500	0	-2
```

If you get the following error, it may have been due to incorrect swap size, but i'm not sure. This error triggered me to increase swap size:
```
Thread '<unnamed>' panicked at 'Due to validation `initial` and `maximum` should be valid: Memory("mmap returned an error")', src/libcore/result.rs:999
```

### Close repo to Host Machine

* Clone https://github.com/ltfschoen/edgeware-node
  ```
  git clone git@github.com:ltfschoen/edgeware-node.git
  ```
* Change to cloned directory
  ```
  cd ~/code/src/ltfschoen/edgeware-node
  ```

### Copy directory from Host Machine to Linode Instance

* Install Rsync on Remote Machine
```
ssh root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_EDGEWARE> "sh -c 'nohup apt install rsync > /dev/null 2>&1 &'"
```

* Install Docker CE on Remote Machine
```
ssh root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_SUBSTRATE> 'bash -s' < ./scripts/setup-docker.sh;
```

* Copy the cloned Edgeware directory to the Linode instance

  * Note: Instead of using `rsync` after the initial rsync, retrieve latest changes from within the VPS as soon as you SSH into it in the next step as follows:
    ```
    cd edgeware-node; 
    git checkout master;
    git pull --rebase upstream master;
    git checkout luke-validator;
    git merge master;
    git add .
    git commit -m "merged latest upstream"
    ```
  * Note: If Edgeware upstream repo has changed, then delete all the Docker containers in the next step before creating them again as follows:
    ```
    cd edgeware-node/scripts;
    bash docker-destroy.sh;
    cd ..
    ```

```
rsync -az --verbose --progress --stats ~/code/src/ltfschoen/edgeware-node root@<IP_ADDRESS>:/root;
```

### SSH Auth into to the Linode Instance

* Go to "Remote Access" of Linode instance
* Copy the "SSH Access" command from the Linode UI. i.e. ssh root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_EDGEWARE>, or copy the IP Address of the Linode instance and run:

```
ssh-keygen -R <INSERT_IP_ADDRESS_LINODE_INSTANCE_EDGEWARE>;
ssh root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_EDGEWARE>
```

### Create a Docker Container in the Linode Instance

* Change to the Edgeware directory on the Linode Instance and create a Docker container

```
apt-get update; apt-get install screen -y;
screen -S root;

cd edgeware-node; docker-compose up --force-recreate --build -d;
```

Note:
* Re-attach to a screen with `screen -r`
* Switch between screens CTRL+A
* Exit a screen with CTRL+A+D (MacOS)
Reference: https://linuxize.com/post/how-to-use-linux-screen/
https://www.cyberciti.biz/tips/linux-screen-command-howto.html

### Access the Docker Container in the Linode Instance

```
docker exec -it $(docker ps -q) bash;
```

### Sync to latest block

* Create root screen (`apt-get update; apt-get install screen -y`)
```
apt-get update; apt-get install screen -y; screen -S root
```

* Sync to the latest block. Ensure that you sync without using the `--validator` flag.
Switch out of screen with CTRL+A+D

* Note: For testnet use `edgware-testnet-v8`

```
cd /usr/local/bin;

edgeware --base-path "/root/edgeware" \
  --chain "edgeware" \
  --keystore-path "/root/edgeware/keys" \
  --name "Luke MXC 🔥🔥🔥" \
  --port 30333 \
  --rpc-port 9933 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --ws-port 9944
```

### Validator Setup

Luke's notes when following the Validating on Edgeware v0.8.0 Guide https://github.com/hicommonwealth/edgeware-node/wiki/Validating-on-Edgeware-v0.8.0

Note: The stash of validators from the lockdrop should already be bonded in https://github.com/hicommonwealth/edgeware-node/blob/master/node/service/src/genesis.json

* Prerequisites
  * https://github.com/hicommonwealth/edgeware-node/wiki/Validating-on-Edgeware

Note: Testnet v0.8.0 and Mainnet/Betanet isn't auto-bonded, you have to use the edgeware-cli. Its permissionless. The stash balances are there, just not bonded/staked
Note: You need EDG to be in your stash, and your controller will have existential balance to make the required transactions that include: bonding your stash to your controller, and using your controller to set the session keys and validator settings, and then load the sessions keys into the node's keystore, and you're off to the races.

* Bond the stash

Note that you need to access the Docker Container again to do this with the following first:

```
docker exec -it $(docker ps -q) bash;
```

Update to latest Python (maybe this wasn't required): https://www.digitalocean.com/community/tutorials/how-to-install-python-3-and-set-up-a-programming-environment-on-debian-10

Check you are using Node.js v12.10.0 (installed via Docker), and NOT v8 (see minimum requirements in the edgeware-cli repo)
```
node --version
```

DO NOT install edgeware-cli by installing it from NPM package with `npm install edgeware-cli` (i.e. so it won't be installed in a node_modules/ subfolder).
Instead just clone the Github repo and build it from source

```
apt update && \
apt-get install -y git cmake
```

Generate Github keypairs. Copy the output of `cat id_rsa.pub`, and create a new SSH Key at https://github.com/settings/keys, and paste it there. (see https://stackoverflow.com/a/2643584/3208553)
```
cd ~/.ssh && ssh-keygen && \
cat id_rsa.pub
```

```
cd ~ && \
git clone git@github.com:hicommonwealth/edgeware-cli.git && \
cd edgeware-cli
```

Install Yarn v1.17.3 (NOT v0.27.0). See https://github.com/yarnpkg/yarn/issues/2821#issuecomment-284181365

```
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
apt install -y yarn && \
yarn --version
```

Install dependencies for edgeware-cli
```
yarn && \
yarn run build
```

Check the list of validators, your account balance:
Note: freeBalance returns a 32-bit hex string. Since the EDG token is 18-decimal places,
calculate EDG value using Node.js console:
```
$ node
> parseInt("0x000000000001111d3762d881c9a13fb", 16) / 1000000000000000000;
80609.06227448891
```

Note: Connect to remote node with ` -r ws://mainnet1.edgewa.re:9944`

```
~/edgeware-cli/bin/edge session validators
~/edgeware-cli/bin/edge balances freeBalance <ACCOUNT_PUBLIC_KEY_SS58>
~/edgeware-cli/bin/edge -s <STASH_SEED> staking bond <CONTROLLER_PUBLIC_KEY_HEX> <AMOUNT> <REWARD_DESTINATION>
~/edgeware-cli/bin/edge -s "some words here some words here"//Stash staking bond 0x... 1000000000000000000 stash
```

Note: 1000000000000000000 is equivalent to 1 testEDG (testnet EDG token)
Note: Later you can Bond Extra with. See https://github.com/paritytech/substrate/blob/master/srml/staking/src/lib.rs#L744:
  ```
  ~/edgeware-cli/bin/edge -s <STASH_SEED> staking bondExtra <AMOUNT>
  ```
Note: If you're getting slashed hard, and can't figure out why, try chilling:
  ```
  ~/edgeware-cli/bin/edge -s <CONTROLLER_SEED>//Controller staking chill
  ```
Note: Be sure to check case when entering <STASH_SEED> (i.e. //Stash or //stash)
Note: You can recover key information with `subkey inspect...`
Note: <CONTROLLER_B58_ADDRESS> should actually be the Controller public key (hex)m, not 5...
Note: If it works, it should say the following. Then you just go to Polkascan, view your Stash public key SS58, and it'll show the transaction hash listed 
```
Making tx: staking.bond(["<CONTROLLER_PUBLIC_KEY_HEX>","<AMOUNT>","stash"])
Transfer status: Ready
Transfer status: Broadcast
Transfer status: Finalized
Completed at block hash <HASH>
Events:
	 {"ApplyExtrinsic":2} : system.ExtrinsicSuccess []
```
Note: You need to set a controller for bonding tokens on-chain
Note: In lockdrop there was only one "hot" session key called "authority", but now there are three "hot" session keys that you need to be a validator "aura", "grandpa", and "imonline", so you need to generate them.

```
~/edgeware-cli/bin/edge -r edgeware -s <CONTROLLER_SEED> staking validate <UNSTAKE_THRESHOLD> <VALIDATOR_PAYMENT>
~/edgeware-cli/bin/edge -r edgeware -s <CONTROLLER_SEED> staking validate 3 0
```
Note: If it worked it should output:
```
Making tx: staking.validate(["3","0"])
Transfer status: Ready
Transfer status: Broadcast
Transfer status: Finalized
Completed at block hash <HASH>
Events:
	 {"ApplyExtrinsic":2} : system.ExtrinsicSuccess []
```

Note: If you get error `Failed:  Error: submitAndWatchExtrinsic (extrinsic: Extrinsic): ExtrinsicStatus:: 1010: Invalid Transaction (Payment)` when you run `session setKeys` below, then it's because you have insufficient funds in your Controller account (i.e. 0.07 EDG is ok, buy 0.02 EDG is insufficient!)
SOLUTION: Use your Stash as your Controller too! (this is stupid and risky since we're exposing cold wallet so its warm, but since we can't transfer so we have more funds to cover transaction fees we don't have any choice but to shoot ourselves in the feet security-wise - i'm with stupid)

```
~/edgeware-cli/bin/edge -r edgeware -s <CONTROLLER_SEED> session setKeys <SESSION_PUBLIC_KEY1>,<SESSION_PUBLIC_KEY2>,<SESSION_PUBLIC_KEY3>
~/edgeware-cli/bin/edge -r edgeware -s "..."//Controller session setKeys <SESSION_PUBLIC_KEY1>,<SESSION_PUBLIC_KEY2>,<SESSION_PUBLIC_KEY3>
```
Note: If it works it should output the associated addresses of your aura, gran, and imon session keys (hot keys) as follows:
```
[ [ '5...',
    '5...',
    '5...' ],
  Uint8Array [  ] ]
Transfer status: Ready
Transfer status: Broadcast
Transfer status: Finalized
Completed at block hash <HASH>
Events:
	 {"ApplyExtrinsic":2} : system.ExtrinsicSuccess []
```

* Definitions
  * Grandpa - finalising
  * Aura - authoring
  * ImOnline (heartbeat) - signs a transaction that your validator is online to the chain
* Note: you can't use `rotateKeys` instead of manually generating 3x session keys as mentioned in the guide. someone used `rotateKeys` first and had the slashing issue, but then I created each session key separately, added them to node's keystore, and now my validator seems to work properly.
  * `rotateKeys` gives you one long string which you have to split into 3 keys and then add 0x at the start

If Edgware node is already running and synced to latest block but you can't access the screen with `screen -r`, then run `ps -a`, and kill the process ID (PID) with name `edgeware` process with `kill -9 <PID>` so you can restart it again but with the validator flag `--validator` 

### Run Validator

* Must use `--no-telemetry` otherwise it kills itself
* Rename below for Mainnet (edgeware) OR Testnet (edgeware-test)
```
edgeware --validator \
  --base-path "/root/edgeware" \
  --chain "edgeware" \
  --keystore-path "/root/edgeware/keys" \
  --no-telemetry
```

Wait until synced to latest block, then exit to different screen with CTRL+A+D, and set the session keys

Insert each session key that you generated with subkey into your node's keystore.
Note that you need to access the Docker Container again to do this with the following first:
```
docker exec -it $(docker ps -q) bash;

curl -vH 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["aura", "<mnemonic>//<derivation_path>", "<public_key>"],"id":1 }' localhost:9933
curl -vH 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran", "<mnemonic>//<derivation_path>", "<public_key>"],"id":1 }' localhost:9933
curl -vH 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["imon", "<mnemonic>//<derivation_path>", "<public_key>"],"id":1 }' localhost:9933
```
The output from each curl request should be: `{"jsonrpc":"2.0","result":"0x...","id":1}`.
If the "result" value is `null` then may not have worked, but check the following first: Another way to check (thanks [HashQuark] ZLH), is to go to the following folder, and check that there are 3x files/keys:
* ~/.local/share/edgeware/chains/edgeware/keystore OR
* /root/edgeware/keys

And you can also check it on https://polkadot.js.org/apps/#/staking/actions

CTRL+D to Exit
Then enter `screen -r` to switch back to the validator logs.

Note: See section "Session Key Setup" at the end of this README for more info.
Note: Stash and controller are "cold" keys. Aura, Gran (Grandpa), and Imon (Imonline) are "hot" keys

You should now see yourself in the list of newly/pending validators to go into effect in future sessions. In the next era (up to 1 hour), if there is a slot available, your node will become an active validator.

Now create another terminal tabs (non-root screen) using the screen program by pressing CTRL + A + C. Then close the whole terminal window (all screens at once) and it won't close the original screen's actual process. Major disadvantage: There isn't any notification to tell you if you see your node goes offline, apart from receiving an email notification from your VPS or it no longer appearing on telemetry. If that happens restart it.

* Other notes:

Note: DO NOT use the `--node-key` flag.
Note: If you provide the session key incorrectly, it'll give you an error like: `Error starting the node: Invalid node key: Invalid input length`
Note: Clear bash history after entering your keys with `history -c; rm ~/.bash_history`
Note: If you setup a password with `subkey`, then create a keystore password file and include your password in it, then load it with `--password-filename <PATH>` instead of using `--password "mypassword" \`

### Check Validator Status


* Check you're validator node is healthy and sending online ping events to the network each session to prevent slashing at - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/event, then for the recent sessions imonline heartbeat events. Check to see if one of them shows your "imon" session key's public key. You first need to be bonded, and a validator in the current session, otherwise your node is just passive.
  * Alternatively, uou could just send a transaction, and even if it fails, you know you're connected if it shows as a failed tx.

* Check disk spaced used by chain
```
du -hs /root/edgeware-node
```
* Note: New validators are entered every 10 blocks. See them here - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/session/session. Initially the only validators listed appeared to be Edgeware-owned because they were auto-bonded. there's no staking/bond transactions associated with them, as one that was bonded did the set keys/validate setting transactions in the reverse order of the documentation (i.e. https://polkascan.io/pre/edgeware-testnet/account/5G9UbiviqfuShqjmVqFAUr4BAxWk8KZh4ho9RW2ZoE1rZZnE). All of the validators have "validatorPayment": 0, which is based on the amount you choose to give to nominators. Validator's cannot be auto-bonded unless it's sure they're online at genesis or else the network stalls
* Check the bond shows up on Polkascan
* Check bonded amount Testnet -  - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/session/validator
* Watch the logs and check if you get slashed or not - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/session/validator/8461-12
* Check slashed amount - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/event/35350-1
* Check available Staking commands via CLI `./bin/edge -r edgeware staking list`. View Storage methods for different SRML modules here: https://polkadot.js.org/api/METHODS_STORAGE.html. Note that Edgeware CLI commands wrap around the Substrate interface.
* Check if listed in Telemetry when running node before disabling Telemetry when run validator https://telemetry.polkadot.io/#list/Edgeware%20Testnet
* Check if the displayed "Aura Key" shown in the keygen output matches the Telemetry output
* Check if listed on Polkascan and that stash is bonded - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/session/validator since it should be automatically bonded from genesis if you're in the validator set, and check that your correct session account is shown there too. Click on details next to a validator
* Check account balance,  - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) e.g. https://polkascan.io/pre/edgeware-testnet/account/5DP33MYJsMJi8FNfKHRMAPoGJ4rvNLt5o7CA4MumJPE1GDVE
* Check that you're earning staking rewards when running session keyed validator. See what's shown under "Additional bonded by nominators" or "Commission"

### Interact with Edgeware Node

* Edgeware UI - Use Edgeware's polkadot.js.org Apps equivalent
  * Go to https://polkadot.js.org/apps/#/settings
  * Toggle Custom Endpoint button
  * Enter depending on network for Substrate Address prefix, and then click "Save & Reload"? Get it from "NodeInfo" at https://commonwealth.im/#!/settings
    * Mainnet:
      ```
      wss://mainnet1.edgewa.re
      wss://mainnet2.edgewa.re
      wss://testnet1.edgewa.re
      ```
  * Add Custom Edgware Types by going to https://polkadot.js.org/apps/#/settings/developer and replacing `{}` with the contents of this Gist: https://gist.github.com/drewstone/cee02c503107d06badbdc49bea35c526
  * Import the Stash, Controller, Session accounts by going to https://polkadot.js.org/apps/#/accounts
  * Choose "Add Account"
  * Enter a name (i.e. "Luke_Stash")
  * Enter your "private key mnemonic seed" 
  * Enter a "password" (which will be used for signing transactions using polkadot.js.org/apps, and to restore the JSON backup file that'll be downloaded and encrypted with that password).
  * Enter for "keypair type": Use Schnorkell for Stash/Controller or Edwards for Session (see https://wiki.polkadot.network/en/latest/polkadot/node/guides/how-to-validate/#set-the-session-key)
  * Enter for "secret derivation path" the derivation that you used (if any) (i.e. `//stash` or `//Stash` that you would put after your private key mnemonic).
  * Note: When you enter your "private key mnemonic", or your "secret derivation path" you'll notice that it will automatically update your Public Key BS58. Check that this BS58 Public Key matches what you were provided with by Subkey!

* Edgeware CLI - https://github.com/hicommonwealth/edgeware-cli

### Eras vs Epochs/Sessions vs Slots

* you can see Eras and Epochs/Sessions changing in the Substrate/Polkadot UI here https://polkadot.js.org/apps/#/explorer.
* Eras comprise of a number of Sessions/Epochs (where Sessions are coupled to Epochs).
* Epochs have a duration that's measured in a number of Slots.
* Slots are part of BABE (block authoring) and are measured in milliseconds.
* Relevant Substrate interfaces values you'd use to calculate it include, SessionsPerEra from the Staking, EpochDuration from Babe, SlotDuration from Aura
* Latest metadata from Substrate that's used for the front-end https://github.com/polkadot-js/api/blob/master/packages/types/src/Metadata/v7/static-substrate.json. Shown under "Substrate interfaces" in the API Reference docs https://polkadot.js.org/api/api/#api-selection
https://substrate.dev/docs/en/overview/glossary#transaction-era
* Polkascan here  - Rename for either Mainnet (edgeware) OR Testnet (edgeware-test) https://polkascan.io/pre/edgeware-testnet/session/session shows the Session ID associated with each Block Number, and then click the "Details" button for a specific Block Number, and the "Details" page will include the associated Era

### Consider setting up an IP Failover solution

See https://www.linode.com/docs/platform/manager/remote-access/#configuring-ip-sharing. But have to somehow protect from double-signing. Say you have two linodes with IP failover detects it should switch after linode1 signs one block, and linode2 signs a different one just after. Credit: @fress

An alternative solution using a different VPS with failover is here: https://medium.com/hackernoon/a-serverless-failover-solution-for-web-3-0-validator-nodes-e26b9d24c71d

### View Node Information

Open a Bash Terminal tab and SSH into Linode
```
ssh root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_EDGEWARE>
```

Access Docker container with Bash prompt
```
docker exec -it $(docker ps -q) bash;
```

View Disk Usage of Substrate chain DB
```
du -hs /root/edgeware/chains/edgeware_testnet/db
```

### Share Chain Database

* Zip latest chain (i.e. if user on MacOS wants to share latest chain, just zip it)

```
tar -cvzf 2019-08-01-db-edgeware.tar.gz "/Users/Ls/Library/Application Support/Edgeware/chains/edgeware/db"
```

```
tar -cvzf 2019-09-09-db-edgeware.tar.gz "~/edgeware/chains/edgeware_testnet/db"
```

* Share zip file with your friend

* Copy latest chain to Linode

```
rsync -avz "/Users/Ls/Library/Application Support/Edgeware/chains/edgeware/db/2019-08-01-db-edgeware.tar.gz" root@<INSERT_IP_ADDRESS_LINODE_INSTANCE_SUBSTRATE_OR_POLKADOT>:/root/edgeware
```

### Show System Information of Linode Instance

```
cd /root/edgeware-node/scripts;
bash system-info.sh
```

### Show Docker Information of Linode Instance

```
cd /root/edgeware-node/scripts;
bash docker-info.sh
```

### Destroy all Docker Images and Containers on the Linode Instance

```
cd /root/edgeware-node/scripts;
bash docker-destroy.sh
```

### Creation of Additional Nodes

Creation of additional Edgeware Nodes should use a different `--base-path`, have a different name, run on a different port `--port` (i.e. initial node `30333`, second node `30334`, etc), and the `--bootnodes` should include details of other initial nodes shown in Bash Terminal (i.e. `--bootnodes 'enode://QmPLDpxxhYL7dBiaHH26YqzXjLaaADoa4ShJSDnufgPpm1@127.0.0.1:30333'`)

## Setup Nominator

* Nominators go through the on-chain nomination system. https://wiki.polkadot.network/en/latest/polkadot/node/nominator/. Try using the Polkadot.js Apps front-end

## Launch Date

Edgeware launches 15th Sept 2019. 00:00 UTC Sept 15. (8PM Sept 14 EDT)

## Session Key Setup
If you plan to validate on Edgeware or a testnet with any non-default keys, then you will need to store the keys so that the node has access to them, for signing transactions and authoring new blocks. Keys in Edgeware are stored in the keystore in the file system. To store keys into this keystore, you need to use one of the two provided RPC calls.

If your keys are encrypted or should be encrypted by the keystore, you need to provide the key using one of the cli arguments --password, --password-interactive or --password-filename.

### Recommended RPC call
For most users who want to run a validator node, the author_rotateKeys RPC call is sufficient. The RPC call will generate N Session keys for you and return their public keys. N is the number of session keys configured in the runtime. The output of the RPC call can be used as input for the session::set_keys transaction.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' localhost:9933
```

### Advanced RPC call
If the Session keys need to match a fixed seed, they can be set individually key by key. The RPC call expects the key seed and the key type. The key types supported by default in Edgeware are `aura`, `gran`, and `imon`.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["KEY_TYPE", "SEED", "PUBLIC_KEY"],"id":1 }' localhost:9933
```
`KEY_TYPE` - needs to be replaced with the 4-character key type identifier. `SEED` - is the seed of the key.

### Systemd Setup

INCOMPLETE, ask @gnossienli for help. More info https://wiki.polkadot.network/en/latest/polkadot/node/guides/how-to-systemd/

* Systemd Process Setup. Run `systemctl status edgeware.service`, `sudo journalctl -f -u edgeware` after setup with:
* Rename for either Mainnet (edgeware) OR Testnet (edgeware-test)
```
[Unit]
Description=edgeware Node
After=network-online.target

[Service]
User=root
WorkingDirectory=/home/root/edgeware-node
ExecStart=/root/edgeware-node/target/release/edgeware --chain=edgeware-testnet-v8 --name edge  

Restart=always
RestartSec=3
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
```

* Output should be:
```
● edgeware.service - edgeware Node
Loaded: loaded (/etc/systemd/system/edgeware.service; enabled; vendor preset: enabled)
Active: active (running) since Fri 2019-09-06 11:54:05 UTC; 5s ago
Main PID: 21574 (edgeware)
  Tasks: 1 (limit: 4915)
CGroup: /system.slice/edgeware.service
        └─21574 /root/edgeware-node/target/release/edgeware --chain=edgeware-testnet-v8 --name edge
```

### Genesis

For the Edgeware Mainnet Launch, we recommend the lockdrop allocation genesis spec with the following hashes. The hashes can be generated by running `shasum` and `b2sum` on the file `node/service/src/genesis.json`. 
```
➜  edgeware-node git:(master) ✗ shasum node/service/src/genesis.json
db5a838b01bc229f74777f4d549b274ecd03915f  node/service/src/genesis.json
➜  edgeware-node git:(master) ✗ b2sum node/service/src/genesis.json
f9052b22c3b2cdc4d4a8aa08c797ac95ea93aa9a912045c49600799135536eba0271a77694b23fe64385346dd1556811b890815c8febe62ad556e2e4e823dc2c  node/service/src/genesis.json
```

At launch, you can run `edgeware-node` and recommended chainspec, as is. However, individuals are able to independently regenerate and verify that the recommended `node/service/src/genesis.json` hashes follow the `edgeware-lockdrop` specification. To do this, you'll need to:
1. Clone [edgeware-lockdrop](https://github.com/hicommonwealth/edgeware-lockdrop)
2. Run `node scripts/lockdrop.js --allocation` to generate a lockdrop `genesis.json` in your working directory.
3. Copy the `genesis.json` into your copy of the `edgeware-node/node/service/src`.
4. Compute the hashes and verify they match.

From here, you can use the normal build and run commands.

### Troubleshooting / FAQ

Note that the following may no longer be necessary since there have been upates to the Edgeware repo.

* Use `--no-telemetry` if get error: `Rejected log entry because queue is full for`
and to fix the stuck 100% cpu issue (since Telemetry turned on by default).
Unless Cargo.lock already updated to use the fixed code in the substrate-telemetry package

* Prevent logs stopping or not syncing by increasing the username's per process limit by adding line just above `# End of file` and after doing the following restart computer
    ```
    ulimit -a
    sudo vi /etc/security/limits.conf
    username soft nofile 10240
    sudo reboot now
    ```

* If you're getting `Killed` shown in the stdout (i.e. `kernel out of memory error` in the syslog) then try getting synced to latest block with or without the `--validator` flag, then check you have around 2 CPU / 4 GB RAM (1 GB RAM is insufficient to run a node). View how much CPU % and RAM is being used when you're not is syncing to the latest block with Telemetry (but turn off when want to validate). Try also increasing your Swap to at least 8 GB
  * See https://github.com/hicommonwealth/edgeware-node/issues/93, and https://github.com/hicommonwealth/edgeware-node/issues/98

* If it's crashing when you're running with `--validator` then make sure you've turned off Telemetry with `--no-telemetry` (since it's not so efficient with memory and occasionally uses up too much CPU, and use Polkascan instead to check you're node is sending online ping events to the network each session to prevent slashing)

* If you get an `InvalidAuthoritiesSet` related error, then don't use the flag `--execution "both"`

* Why am I getting slashed to 0 beyond my staked bond?

11 Sept 2019. This is a bug that's being looked into. The reason validators are being slashed to 0 beyond their stake bond is that the slash is dependent on the bonded stake amount, but slashing doesn't reduce/update the network's value of your bonded stake. So your slash will be deducted from your stash account, but the bonded stake is not reduced -- so the values diverge.   This means your bonded stake becomes much larger than your stash and as the stash is slashed based on the larger bonded stake value, it will be  dropped to 0 regardless of the initial balance eventually. This is a bug that the Edgeware team have been notified about and is a major reason slashing will be set to 0 for the launch.  It causes you to get slashed beyond the amount you had locked in the bond, which should cap your exposure.

Slashing occurs when you do not notify the network each session that you're online.  A session is only about 90 second, so if your validator is unresponsive for more than 90 seconds you will be slashed.  The validators have some resource utilization issues (both CPU and memory) that seems to cause them to lock up, sometimes permanently until restarted, and sometimes temporarily.   The slashing issue seems mainly caused by the CPU slamming to 100% utilization and locking up making the node not send a heartbeat and get slashed.

It'll earn rewards quickly due to the bug that gave exponential rewards, then eventually will start to get slashed, while also earning rewards as the stash and bonded value grows from rewards, the slashes become bigger the node can run forever, even after the account is reaped took me 3 tries to get one stable, but even it's been slashed for about 4/5 of its rewards the first two (last week) were slashed to 0 like yours
in any event, the mainnet will have  0 slashing for these reasons.

Starting with a few tokens is what keeps the slashes small initially the rewards will be billions of EDG while you have 1 token bonded, it's unnoticeable that was my experience you can also go back to see where you were slashed, but polkascan is hard to go back in time, so easier to watch session by session in real time that's how I figured out what was going on there also appears to be a delay from when you're first online, even if no heartbeat, and when you get slashed.

The combination of the slashing being based on the bonded stake (5% or 10% of total bonded), but slashing not deducting from bonded stake (only slash), and the node instability with sending heartbeats, that makes keeping a node online very hard but 5-10% of the total every 90 seconds means you can get wrecked very quickly and not see it happen.

* What do to after being slashed?

Install latest edgeware-node
Install latest edgeware-cli
Purge chain if necessary
Create and insert new session keys into node
Purge chain db before re-running
Check sufficient swap available space
Check sufficient hard drive available space 

* What are minimum hardware specs to be a validator?

4GB/2CPU with 8GB swap and `--no-telemetry`
2GB/2CPU with 16GB swap and `--no-telemetry`
Note: Users have reported being killed using 8GB RAM but without an swap and `--no-telemetry`.
Note: You'll usually use 3.5GB memory, as shown consistently on Telemetry
Note: Using 1GB RAM on VPS is insufficient, it'll become unresponsive or crash

## Misc Resources
  * [Validating on Edgeware Mainnet](https://github.com/hicommonwealth/edgeware-node/wiki/Validating-on-Edgeware/_edit)
  * https://edgewa.re
  * https://edgewa.re/lockdrop/
  * https://blog.edgewa.re/edgeware-lockdrop-for-validators/
  * https://edgewa.re/dev/
  * https://github.com/hicommonwealth/edgeware-node/wiki
  * blog.edgewa.re
  * http://twitter.com/heyedgeware
  * Discussion and governance https://commonwealth.im [Commonwealth.im](https://commonwealth.im)
  * https://github.com/hicommonwealth/edgeware-node
  * https://medium.com/@meleacrypto (Edgeware Validator Guide)
  * https://wiki.polkadot.network/en/latest/polkadot/node/guides/how-to-validate/
  * https://wiki.polkadot.network/en/latest/polkadot/node/guides/how-to-systemd/
  * https://github.com/ltfschoen/polkadot-linode
  * https://github.com/luboremo/Edgeware-seed-generating-script-SSSS
  * https://wiki.polkadot.network/en/latest/polkadot/node/node-operator/#security-key-management
  * Validating discussion https://commonwealth.im/#!/edgeware/proposal/discussion/20
     * Rename link for either Mainnet (edgeware) OR Testnet (edgeware-test)
