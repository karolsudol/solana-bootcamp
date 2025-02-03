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


## 4. create, build, test anchor program

```sh
anchor init --test-template rust {program-name}
anchor build
anchor test
```

## 5. add - update dependencies
```sh
yarn
```
