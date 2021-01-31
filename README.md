# Blockchain.rs
## Description
A toy implementation of the Blockchain Algorithm, implemented in Rust.

## Structs
* Trader(Trades Bitcoin with other Traders)
  * private key
  * public key
  * ID

* Block (Merkle Tree of all past transactions)
  * Prev Hash
  * Secret Number
  * Transactions(balanced Merkle tree)
  * Miner

* Transaction
  * Sender
  * Receiver
  * Amount

* SignedTransaction
  * Transaction
  * Signature

* Miner

## Todo
* find better way to hash a transaction (just using raw bytes rn, unsafe!)

## Additional Ressources
* [3b1b Video on Blockchain](https://www.youtube.com/watch?v=bBC-nXj3Ng4)
* [Original Bitcoin Paper](https://bitcoin.org/en/bitcoin-paper)
* [Merkle Trees](https://www.youtube.com/watch?v=s0fruNfgW30)



