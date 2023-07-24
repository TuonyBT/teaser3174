fn main() {
    println!("{:?}", sieve_of_eratothsenes_factors(100).1);
}



//  Adapt basic Sieve of Eratothsenes to return the prime factors of compound numbers as well
fn sieve_of_eratothsenes_factors(x: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut sieve = vec![true; x + 1];
    let mut prime_factors = vec![Vec::<usize>::new(); x + 1]; // add somewhere to store factors
    sieve[0] = false;
    sieve[1] = false;
    let mut lp: usize = 2;
//    while lp <= (x as f64).sqrt().floor() as usize {
    while lp < x {                                                      // need to record all multiples of each prime
//        let fnp = lp.pow(2);                                
        let fnp = lp;                                //  including the first
        for idx in (fnp..sieve.len()).step_by(lp) {
            sieve[idx] = false;
            prime_factors[idx].push(lp);
        }
        lp = match sieve[lp + 1..].iter().position(|z| z == &true) {
            Some(y) => y + lp + 1,
            None => x,
        };
    }
    let primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0).collect::<Vec<usize>>();
    (primes, prime_factors)
}