<p align="center">
  <a href="https://solana.com">
    <img alt="Solana" src="https://i.imgur.com/uBVzyX3.png" width="250" />
  </a>
</p>


# Build & Deploy a Solana Smart Contract

In this project we'll build a smart contract and deploy it to the Solana network (either by spinning up a local node or by uploading it to the Solana testnet). The project comprises of:

* An on-chain hello world program
* A smart contract upgrade to sending and unpacking _data instructions_
* A client that can send a "hello" to an account and get back the number of
  times "hello" has been sent


## Quick Start


The following dependencies are required to build and run this example, depending
on your OS, they may already be installed:

- Install node (v14 recommended)
- Install npm
- Install Rust v1.56.1 or later from https://rustup.rs/
- Install Solana v1.8.14 or later from
  https://docs.solana.com/cli/install-solana-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

### Configure CLI

> If you're on Windows, it is recommended to use [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to run these commands

1. Run a couple of checks to ensure all dependencies are correctly installed 

```bash
solana --version            # check Solana is correctly installed
solana config get           # check Solana's current configurations
solana-keygen --version     # check that `solana-keygen` is installed correctly
```

2. Set CLI config url to localhost cluster

```bash
solana config set --url localhost
```

3. Create CLI Keypair (if you don't have one yet) OR verify your Keypair. See Solana doc [here.](https://docs.solana.com/wallet-guide/paper-wallet#public-key-derivation) 

```bash
solana-keygen new                           # if this is your first time using the Solana CLI, you will need to generate a new keypair
solana-keygen verify <PUBKEY> prompt://     # to control the private key of a paper wallet address, use 'solana-keygen verify'
```

4. (optional) Play around with your account address. In the context of Solana _wallet_ and _account address_ are synonims. 

```bash
solana account <ADDRESS>               
solana balance <ADDRESS>                
solana airdrop <AMOUNT> <ADDRESS>
```

### Start local Solana cluster

This example connects to a local Solana cluster by default. It'll take a while for this command to execute, just be patient.

Start a local Solana cluster:
```bash
solana-test-validator
```
> Note: You may need to do some [system tuning](https://docs.solana.com/running-validator/validator-start#system-tuning) (and restart your computer) to get the validator to run

> :no_entry_sign: :radioactive: :warning: **Important**: The `solana-test-validator` is literally your active connection to the Solana network. So, leave the validator running in the current terminal ane open a **new** terminal to run _all_ of the command from now onwards. You _must_ run two terminals in parallel for the contract deploymen to be successful. If the `solana-test-validator` stops running at any point in time, you'll use connection with the Solana network and you won't be able to deploy your smart contract.

### Build rust files

```bash
export PATH="$HOME/.cargo/bin:$PATH"          # (optional) export the path to _cargo_ 
cargo clean                                   # clean outdated rust files if there are any
cargo build                                   # build files (like Cargo.lock) to allow rust execution
```

> Beware: `sudo` and `cargo` are antagonist. No **not** use them together! Using `sudo cargo build` makes the Cargo.lock file belong to the root user, meaning the file will have restricted permissions and won't be writable.

### Grant permissions
(optional)

cd into the repo containing your Solana on-chain `program-rust` and run

```bash
ls -l         # <-- with a whitespace at the end, like this 'ls -l '. This prints current permissions for each file
```

 which returns something like this
 
 ```
 -rw-rw-rw-  1 irenefabris  staff  77749 Mar 29 14:59 Cargo.lock
 ```
 
 Run `chmod` to grant full permissions
 
 ```bash
 chmod <NUM OPTION> <FILENAME>            # generic chmod command
 chmod 777 Cargo.lock                     # example command you should run
 ```
 
 which returns
 
 ```
 -rwxrwxrwx  1 irenefabris  staff  77749 Mar 29 14:59 Cargo.lock
```

### Install npm dependencies

```bash
npm install
```

### Build the on-chain program

There is both a Rust and C version of the on-chain program, whichever is built
last will be the one used when running the example.

```bash
npm run build:program-rust

npm run build:program-c
```
> **Important**: You should run the command above in _parallel_ with the `solana-test-validator`. Both commands should be executed concurrently in two different terminal windows. 

### Grant more permissions
(optional)

The `npm run build:program-rust` command may return an error of this type

```bash
> helloworld@0.0.1 build:program-rust
> cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program

BPF SDK: /Users/irenefabris/.local/share/solana/install/releases/1.10.5/solana-release/bin/sdk/bpf
Failed to install bpf-tools: Permission denied (os error 13)
```

If this is your case, it means the rust program could not be built because some permissions were denied. To fix it, cd into the path of the folder with restricted permissions and grant more permissions using the chmod command on _every_ restricted files. If the `chmod` command returns an OperationNotPermitted error, then use `sudo chmod` instead. In our case, we would have to

```bash
cd /Users/irenefabris/.local/share/solana/install/releases/1.10.5/solana-release/bin/sdk/bpf
ls -l 
sudo chmod 777 filename1
sudo chmod 777 filename2
sudo chmod 777 filename3
ls -l 
```
You have just granted any user full permissions to read, write, execute (drwxrwxrwx). 

Now exit all running `cd ~` and cd into the repo with your entire smart contract codebase. Now run again

```bash
npm run build:program-rust
 ```


### Deploy the on-chain program

```bash
solana program deploy <PATH TO THE helloworld.so FILE>
```

If it returns the Error: _Dynamic program error: missing signature for supplied pubkey..._, then it means you should specify the `--keypair` flag in the command you executed. If you're using a system file wallet, then add after `--keypair` the path to the json file where your pubkey is stored. If you own a paper wallet, you might have to add the actual `<ADDRESS>`

```bash
solana program deploy <PATH TO THE helloworld.so > --keypair <PATH TO solana/id.json>               # general command 
solana program deploy /.helloworld.so --keypair /Users/irenefabris/.config/solana/id.json           # example
```


### Run the JavaScript client

```bash
npm run start
```

### Expected output

Public key values will differ:

```bash
Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 2045430982, 'solana-core': '1.7.8' }
Using account AiT1QgeYaK86Lf9kudqKthQPCWwpG8vFA1bAAioBoF4X containing 0.00141872 SOL to pay for fees
Using program Dro9uk45fxMcKWGb1eWALujbTssh6DW8mb4x8x3Eq5h6
Creating account 8MBmHtJvxpKdYhdw6yPpedp6X6y2U9dCpdYaZJdmwV3A to say hello to
Saying hello to 8MBmHtJvxpKdYhdw6yPpedp6X6y2U9dCpdYaZJdmwV3A
8MBmHtJvxpKdYhdw6yPpedp6X6y2U9dCpdYaZJdmwV3A has been greeted 1 times
Success
```

#### Not seeing the expected output?

- Ensure you've [started the local cluster](#start-local-solana-cluster),
  [built the on-chain program](#build-the-on-chain-program) and [deployed the program to the cluster](#deploy-the-on-chain-program).
- Inspect the program logs by running `solana logs` to see why the program failed.
  - ```bash
    Transaction executed in slot 5621:
    Signature: 4pya5iyvNfAZj9sVWHzByrxdKB84uA5sCxLceBwr9UyuETX2QwnKg56MgBKWSM4breVRzHmpb1EZQXFPPmJnEtsJ
    Status: Error processing Instruction 0: Program failed to complete
    Log Messages:
      Program G5bbS1ipWzqQhekkiCLn6u7Y1jJdnGK85ceSYLx2kKbA invoke [1]
      Program log: Hello World Rust program entrypoint
      Program G5bbS1ipWzqQhekkiCLn6u7Y1jJdnGK85ceSYLx2kKbA consumed 200000 of 200000 compute units
      Program failed to complete: exceeded maximum number of instructions allowed (200000) at instruction #334
      Program G5bbS1ipWzqQhekkiCLn6u7Y1jJdnGK85ceSYLx2kKbA failed: Program failed to complete

### Customizing the Program

To customize the example, make changes to the files under `/src`.  If you change
any files under `/src/program-rust` or `/src/program-c` you will need to
[rebuild the on-chain program](#build-the-on-chain-program) and [redeploy the program](#deploy-the-on-chain-program).

Now when you rerun `npm run start`, you should see the results of your changes.

## Learn about Solana

More information about how Solana works is available in the [Solana
documentation](https://docs.solana.com/) and all the source code is available on
[github](https://github.com/solana-labs/solana)

Further questions? Visit us on [Discord](https://discordapp.com/invite/pquxPsq)

## Learn about the client

The client in this example is written in TypeScript using:
- [Solana web3.js SDK](https://github.com/solana-labs/solana-web3.js)
- [Solana web3 API](https://solana-labs.github.io/solana-web3.js)

### Entrypoint

The [client's
entrypoint](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/main.ts#L13)
does five things.

### Establish a connection to the cluster

The client establishes a connection with the cluster by calling
[`establishConnection`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L92).

### Establish an account to pay for transactions

The client ensures there is an account available to pay for transactions,
and creates one if there is not, by calling
[`establishPayer`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L102).

### Check if the helloworld on-chain program has been deployed

In [`checkProgram`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L144),
the client loads the keypair of the deployed program from `./dist/program/helloworld-keypair.json` and uses
the public key for the keypair to fetch the program account. If the program doesn't exist, the client halts
with an error. If the program does exist, it will create a new account with the program assigned as its owner
to store program state (number of hello's processed).

### Send a "Hello" transaction to the on-chain program

The client then constructs and sends a "Hello" transaction to the program by
calling
[`sayHello`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L209).
The transaction contains a single very simple instruction that primarily carries
the public key of the helloworld program account to call and the "greeter"
account to which the client wishes to say "Hello" to.

### Query the Solana account used in the "Hello" transaction

Each time the client says "Hello" to an account, the program increments a
numerical count in the "greeter" account's data.  The client queries the
"greeter" account's data to discover the current number of times the account has
been greeted by calling
[`reportGreetings`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L226).


## Pointing to a public Solana cluster

Solana maintains three public clusters:
- `devnet` - Development cluster with airdrops enabled
- `testnet` - Tour De Sol test cluster without airdrops enabled
- `mainnet-beta` -  Main cluster

Use the Solana CLI to configure which cluster to connect to.

To point to `devnet`:
```bash
solana config set --url devnet
```

To point back to the local cluster:
```bash
solana config set --url localhost
```

## Writing the client in Rust

This example details writing the client code in typescript; however
the Solana client program can be written in any language. For an
example client written in Rust and an accompanying write up see [this
repo](https://github.com/ezekiiel/simple-solana-program).