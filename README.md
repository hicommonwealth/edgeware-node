# edgeware-node - Luke's version to run a validator using Linode

Note: I have removed parts of the original Readme and only left the parts that I need.
If you're a visitor to this repo, please refer to the original repo too first so you're not missing out on anything important.

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
* Select Nanode 1GB instance
* Select node location - i.e. Singapore
* Click Create

Deploy an Image
* Go to "Dashboard" of Linode instance
* Click Deploy an Image
* Select Ubuntu 18.04 LTS or Debian 10
* Select Disk 25000 MB (note that 12 GB is insufficient)
* Select Swap Disk 512 MB

Boot Image
* Go to "Dashboard" of Linode instance
* Click "Boot"

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

```
rsync -az --verbose --progress --stats --exclude='.git/' ~/code/src/ltfschoen/edgeware-node root@139.162.31.81:/root;
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
cd edgeware-node;
docker-compose up --force-recreate --build -d;
```

### Access the Docker Container in the Linode Instance

```
docker exec -it $(docker ps -q) bash;
```

* Create root screen. Later create other windows (non-root) using the `screen` program by pressing CTRL + A + C so that when you close the terminal window, it does not close the original screen's process.

```
edgeware --validator \
  --chain "edgeware" \
  --base-path "/root/edgeware" \
  --execution both \
  --key "<INSERT_ACCOUNT_RAW_SEED_WITHOUT_0x_PREFIX>" \
  --keystore-path "/root/edgeware/keys" \
  --name "🔥🔥🔥" \
  --port 30333 \
  --pruning 256 \
  --rpc-port 9933 \
  --ws-port 9944
```

* Check disk spaced used by chain

```
du -hs /root/edgeware-node
```

* Check if listed as validator in Telemetry at https://telemetry.polkadot.io/#list/Edgeware%20Testnet%20V7
* Check if the displayed "Aura Key" shown in the keygen output matches the Telemetry output
* Check if listed on Polkascan and that stash is bonded https://polkascan.io/pre/edgeware-testnet/session/validator since it should be automatically bonded from genesis if you're in the validator set, and check that your correct session account is shown there too. Click on details next to a validator
* Check that you're earning staking rewards when running session keyed validator. See what's shown under "Additional bonded by nominators" or "Commission"

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
du -hs /root/edgeware-node
```

### Share Chain Database

* Zip latest chain (i.e. if user on MacOS wants to share latest chain, just zip it)

```
tar -cvzf 2019-08-01-db-edgeware.tar.gz "/Users/Ls/Library/Application Support/Edgeware/chains/edgeware/db"
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

Edgeware launches 15th Sept 2019

## Session Key Setup
If you plan to validate on Edgeware or a testnet with any non-default keys, then you will need to store the keys so that the node has access to them, for signing transactions and authoring new blocks. Keys in Edgeware are stored in the keystore in the file system. To store keys into this keystore, you need to use one of the two provided RPC calls. If your keys are encrypted or should be encrypted by the keystore, you need to provide the key using one of the cli arguments --password, --password-interactive or --password-filename.

### Recommended RPC call
For most users who want to run a validator node, the author_rotateKeys RPC call is sufficient. The RPC call will generate N Session keys for you and return their public keys. N is the number of session keys configured in the runtime. The output of the RPC call can be used as input for the session::set_keys transaction.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' localhost:9933
```

### Advanced RPC call
If the Session keys need to match a fixed seed, they can be set individually key by key. The RPC call expects the key seed and the key type. The key types supported by default in Edgeware are `aura`, `gran`, and `imon`.
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_insertKey", "params":["KEY_TYPE", "SEED"],"id":1 }' localhost:9933
```
`KEY_TYPE` - needs to be replaced with the 4-character key type identifier. `SEED` - is the seed of the key.

### Troubleshooting

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

## Misc Resources
  * https://edgewa.re
  * https://edgewa.re/dev/
  * https://github.com/hicommonwealth/edgeware-node/wiki
  * blog.edgewa.re
  * http://twitter.com/heyedgeware
  * Discussion and governance https://commonwealth.im [Commonwealth.im](https://commonwealth.im)
  * https://github.com/hicommonwealth/edgeware-node
  * https://medium.com/@meleacrypto (Edgeware Validator Guide)
  * https://wiki.polkadot.network/en/latest/polkadot/node/guides/how-to-validate/
  * https://github.com/ltfschoen/polkadot-linode
