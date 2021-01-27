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

# Disclaimer 
To keep it simple, i am using 32bit Encryption instead of 256bit.
This doesnt change the core algorithm but it makes it more feasable
to simulate.
