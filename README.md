# Blockchain.rs
## Description
A toy implementation of the Blockchain Algorithm, implemented in Rust.

## Structs
* Trader(Trades Bitcoin with other Traders)
  * private key
  * public key

* Block
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

## Additional Ressources
* [3b1b Video on Blockchain](https://www.youtube.com/watch?v=bBC-nXj3Ng4)
* [Original Bitcoin Paper](https://bitcoin.org/en/bitcoin-paper)
* [Merkle Trees](https://www.youtube.com/watch?v=s0fruNfgW30)

## Differences to 'traditional' Blockchain
* P2P Network is replaced by concurrent threads(with seperate memory)

## TODO
* use arrays for storing hashes (fixed size, no need to be on heap)
