use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

pub struct Trader{
    public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    id: String,
}

struct Transaction<'a>{
    sender: &'a Trader,
    receiver: &'a Trader,
    amount: i32,
}

struct SignedTransaction<'a>{
    transaction: &'a Transaction<'a>,
    signature: Vec<u8>,
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
    fn sign(&self, t: &'a Transaction<'a>) -> SignedTransaction<'a>{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&t) };
        let hashed = Sha256::digest(&bytes).to_vec(); 
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = t.sender.private_key.sign(padding, &hashed).unwrap();
        
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }
}

impl<'a> SignedTransaction<'a>{
    pub fn verify(&self) -> bool{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&self.transaction) };
        let hashed = Sha256::digest(&bytes).to_vec();

        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let signature  = self.transaction.sender.private_key.sign(padding, &hashed).unwrap();
        
        signature.iter()
            .zip(&self.signature)
            .all(|(a, b)| a == b)
    }
}

impl<'a> Transaction<'a>{
    pub fn new(s: &'a Trader, r: &'a Trader, amount: i32) -> Transaction<'a>{
        Transaction{
            sender: s,
            receiver: r,
            amount: amount,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8]{
    // This is dangerous - do this better
    //::std::mem::transmute<T, &[u8]>
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}
fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();

    // Perform an a Transaction
    let mut t = Transaction::new(&t1, &t2, 100);
    let st: SignedTransaction = t1.sign(&t);
    println!("{:?}", st.verify());
}
