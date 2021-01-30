# Blockchain.rs
A toy implementation of the Blockchain Algorithm, implemented in Rust.

# Structs
* Trader(Trades Bitcoin with other Traders)
  * private key
  * public key
  * ID

* Block (list of all past transactions)
  * Prev Hash
  * Secret Number
  * Transactions
  * Miner

* Transaction
  * Sender
  * Receiver
  * Amount
  * Signature

* Miner

# Todo
* find better way to hash a transaction (just using raw bytes rn, unsafe!)

# Additional Ressources
* ![3b1b Video on Blockchain](https://www.youtube.com/watch?v=bBC-nXj3Ng5)
* ![Original Bitcoin Paper](https://bitcoin.org/en/bitcoin-paper)

# Disclaimer 
To keep it simple, i am using 32bit Encryption instead of 256bit.
This doesnt change the core algorithm but it makes it more feasable
to simulate.
