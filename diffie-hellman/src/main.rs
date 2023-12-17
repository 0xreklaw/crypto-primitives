use num_bigint::{BigUint};
use num_primes::Generator;
use num_traits::cast::ToPrimitive;
use std::str::FromStr;

// a simple implementation of the diffie hellman key exchange
fn main() {
    let p = Generator::new_prime(512);
    let p = BigUint::from_str(&p.to_string()).expect("Failed to parse prime number");

    let a = Generator::new_prime(256);
    let a = BigUint::from_str(&a.to_string()).expect("Failed to parse alpha");

    let alice_priv = Generator::new_prime(256);
    let alice_priv = BigUint::from_str(&alice_priv.to_string()).expect("Failed to parse Alice's prime");

    let bob_priv = Generator::new_prime(256);
    let bob_priv = BigUint::from_str(&bob_priv.to_string()).expect("Failed to parse Bob's prime");

    let alice_proof = a.modpow(&alice_priv, &p);
    let bob_proof = a.modpow(&bob_priv, &p);

    let bob_session = alice_proof.modpow(&bob_priv, &p);
    let alice_session = bob_proof.modpow(&alice_priv, &p);

    assert_eq!(alice_session, bob_session, "Session keys do not match");

    println!("\nAlice Session Key: {} \n", alice_session);
    println!("\nBob Session Key: {}\n", bob_session);
}
