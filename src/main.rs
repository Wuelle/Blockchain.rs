use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{distributions::Alphanumeric, Rng};
use byteorder::{LittleEndian, WriteBytesExt};

struct Trader{
    public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    id: String,
}

#[derive(Hash)]
struct Transaction<'a>{
    sender: &'a Trader,
    receiver: &'a Trader,
    amount: i32,
}

#[derive(Hash)]
struct SignedTransaction<'c>{
    transaction: Transaction<'c>,
    signature: Vec<u8>,
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
}
impl Hash for Trader{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.id.hash(state);
    }
}

impl SignedTransaction<'_>{
    pub fn new<'b>(sender: &'b Trader, receiver: &'b Trader, amount: i32) -> Self{
        let t = Transaction{
            sender: sender,
            receiver: receiver,
            amount: amount,
        };
        // Sign the hashed transaction
        let hashed = calculate_hash(&t);
        let padding = PaddingScheme::new_pkcs1v15_encrypt(); 
        let s = sender.private_key.sign(padding, &hashed).unwrap();

        SignedTransaction{
            transaction: t,
            signature: s
        }
    }
}
fn calculate_hash<T: Hash>(t: &T) -> [u8; 8]{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let hash = s.finish();
    //let mut bs = [0u8; 64];
    //bs.as_mut()
    //    .write_u64::<LittleEndian>(hash)
    //    .expect("Unable to convert hash to bytes");
    //return bs
    return hash.to_ne_bytes()
}

//static t1: Trader = Trader::new();
//static t2: Trader = Trader::new();

fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();
    let sta = SignedTransaction::new(&t1, &t2, 100);
    println!("{}", t1.id);
}
