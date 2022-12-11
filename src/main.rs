
use num_primes::Generator;


fn generator(){

}

fn main(){
    println!("Hello, world!");

    // Generate two primes (p,q) of 512 bits each
    let p = Generator::new_prime(1024);
    let q = Generator::new_prime(160);
  
  
    // Generate Safe Prime of 64 bits | Uses (n-1)/2 to check
    let safe_prime = Generator::safe_prime(64);
    println!("{}", safe_prime);

    
}