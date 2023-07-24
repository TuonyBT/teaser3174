use std::collections::BTreeSet;

fn main() {

    let pfs = sieve_of_eratothsenes_factors(99).into_iter().collect::<Vec<BTreeSet<usize>>>();
    let avail_odds = (65..100).filter(|age| age%11 != 0 && age%2 != 0).collect::<Vec<usize>>();
    let avail_evens = (65..100).filter(|age| age%11 != 0 && age%2 == 0).collect::<Vec<usize>>();

    for p in &avail_odds {
        let odds_p = filter_ages(&avail_odds, p, &pfs);

        println!("First age: {} contains digits: {:?}", p, age_digits(p));
        println!("Leaving allowed odd ages: {:?}", odds_p);

        for q in &odds_p {
            let odds_q = filter_ages(&odds_p, q, &pfs);
    
            println!("Second age: {} contains digits: {:?}", q, age_digits(q));
            println!("Leaving allowed odd ages: {:?}", odds_q);
        }
        println!();
    }

    println!();
    println!("Odd ages initially possible {:?}", avail_odds);
    println!("Even ages initially possible {:?}", avail_evens);
}



//  Adapt basic Sieve of Eratothsenes to return the prime factors of compound numbers as well
fn sieve_of_eratothsenes_factors(x: usize) -> Vec<BTreeSet<usize>> {
    let mut sieve = vec![true; x + 1];
    let mut prime_factors = vec![BTreeSet::<usize>::new(); x + 1]; // add somewhere to store factors
    sieve[0] = false;
    sieve[1] = false;
    let mut lp: usize = 2;
//    while lp <= (x as f64).sqrt().floor() as usize {
    while lp < x {                                                      // need to record all multiples of each prime
//        let fnp = lp.pow(2);                                
        let fnp = lp;                                //  including the first
        for idx in (fnp..sieve.len()).step_by(lp) {
            sieve[idx] = false;
            prime_factors[idx].insert(lp);
        }
        lp = match sieve[lp + 1..].iter().position(|z| z == &true) {
            Some(y) => y + lp + 1,
            None => x,
        };
    }
    //let primes = sieve.iter().enumerate().filter(|z| z.1 == &true).map(|z| z.0).collect::<Vec<usize>>();
    prime_factors
}

fn age_digits(age: &usize) -> BTreeSet<usize> {
    vec![age / 10, age % 10].into_iter().collect::<BTreeSet::<usize>>()
}

fn common_digits(x: &usize, y: &usize) -> bool {
    let x_dig = vec![x / 10, x % 10].into_iter().collect::<BTreeSet::<usize>>();
    let y_dig = vec![y / 10, y % 10].into_iter().collect::<BTreeSet::<usize>>();

    x_dig.is_disjoint(&y_dig)
}

fn common_primes(x: &usize, y: &usize, pfs: &Vec<BTreeSet<usize>>) -> bool {
    let x_facs = &pfs[*x];
    let y_facs = &pfs[*y];

    x_facs.is_disjoint(&y_facs)
}

fn filter_ages(ages: &Vec<usize>, p: &usize, pfs: &Vec<BTreeSet<usize>>) -> Vec<usize> {

    ages.to_owned().into_iter()
    .filter(|age| common_digits(age, p))
    .filter(|age| common_primes(age, p, &pfs))
    .collect::<Vec<usize>>()

}