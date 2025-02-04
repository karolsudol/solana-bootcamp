# solana-bootcamp

## CLI:

## 1. get cluster config

```sh
solana config get
```

## 2. set cluster config

```sh
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```


## 3. set a wallet up

```sh
solana-keygen new
solana address
solana config set -ud
solana airdrop 2
solana balance
```


## 4. create, build, test and deploy anchor program

```sh
anchor init --test-template rust {program-name} # or with ts:
anchor init {program-name} 
anchor build # if issues with lockfile Change version 4 to 3 in your cargo.lock file and/or run: 
cargo build-sbf -- -Znext-lockfile-bump
anchor test # alternativelly as:
solana-test-validator # tb run in separate session and test as:
anchor test --skip-local-validator
anchor deploy
solana program close {program_id} --bypass-warning
```

## 5. add - update dependencies
```sh
yarn
```
