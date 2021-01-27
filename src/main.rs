use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{distributions::Alphanumeric, Rng}; 
struct Trader{
    public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    id: String,
}

#[derive(Hash)]
struct Transaction{
    sender: Trader,
    receiver: Trader,
    amount: i32,
}

struct SignedTransaction{
    transaction: Transaction,
    signature: String,
}

impl Trader{
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
//    /// Send currency to another Trader
//    pub fn send() -> Transaction{
//        
//    }
}
impl Hash for Trader{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
    }
}
fn calculate_hash<T: Hash>(t: &T) -> u64{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    let t1: Trader = Trader::new();
    let t2: Trader = Trader::new();
    println!("{:?}", t1.id);
    assert!(calculate_hash(&t1) != calculate_hash(&t2));
}
