extern crate primal_count;
use primal_count::PrimeCounter;
// mod prime_count;
// use prime_count::PrimeCounter;

fn main(){
    let mut pc = PrimeCounter::new(1_000_000_000_000);

    let ppi = pc.prime_pi(1_000_000_000);
    assert_eq!(ppi, 50_847_534);
    println!("{}", ppi);
    
    let ppi = pc.prime_pi(1_000_000_000_000);
    assert_eq!(ppi, 37_607_912_018);
    println!("{}", ppi);

}