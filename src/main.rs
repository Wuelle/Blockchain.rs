use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

pub struct Trader{
    public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    id: String,
}

#[derive(Hash)]
struct Transaction<'a>{
    sender: &'a Trader,
    receiver: &'a Trader,
    amount: i32,
    signature: Option<Vec<u8>>,
}

impl<'a> Trader{
    pub fn new() -> Self{
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RSAPrivateKey::new(&mut rng, bits)
            .expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric) 
            .take(16)
            .map(char::from)
            .collect();

        Trader{
            public_key: public_key,
            private_key: private_key,
            id: id,
        }
    }
    /// Sign a given Transaction with the RSA private key
    fn sign(&self, t: &'a mut Transaction<'a>) -> &mut Transaction<'a>{
        println!("signing a transaction!");
        t
    }
}

impl<'a> Transaction<'a>{
    pub fn new(s: &'a Trader, r: &'a Trader, amount: i32) -> Transaction<'a>{
        Transaction{
            sender: s,
            receiver: r,
            amount: amount,
            signature: None,
        }
    }
}
impl Hash for Trader{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
    }
}

//impl<'c> SignedTransaction<'c>{
//    fn new<'a, 'b>(sender: &'a Trader, receiver: &'b Trader, amount: i32) -> Self{
//        let t = Transaction{
//            sender: sender,
//            receiver: receiver,
//            amount: amount,
//        };
//        // Sign the hashed transaction
//        let hashed = calculate_hash(&t);
//        let padding = PaddingScheme::new_pkcs1v15_encrypt(); 
//        let s = sender.private_key.sign(padding, &hashed).unwrap();
//
//        SignedTransaction{
//            transaction: t,
//            signature: s,
//        }
//    }
//}
fn calculate_hash<T: Hash>(t: &T) -> [u8; 8]{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let hash = s.finish();
    return hash.to_ne_bytes()
}

fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();

    // Perform an a Transaction
    let mut t = Transaction::new(&t1, &t2, 100);
    t1.sign(&mut t);
    println!("{:?}\n----\n {:?}", t1.private_key, t1.public_key);
}
