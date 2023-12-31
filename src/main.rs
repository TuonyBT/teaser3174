use std::collections::BTreeSet;
use itertools::Itertools;

fn main() {

//  Set out the prime factors of each age, as a means to testing for coprimes
    let pfs = sieve_of_eratothsenes_factors(99).into_iter().collect::<Vec<BTreeSet<usize>>>();

//  Each set of eleven can only contain up to one even age, so we can save a bit of trial and error by dividing the ages into odds and evens
    let avail_odds = (65..100).filter(|age| age%11 != 0 && age%2 != 0).collect::<Vec<usize>>();
    let avail_evens = (65..100).filter(|age| age%11 != 0 && age%2 == 0).collect::<Vec<usize>>();

//  Start by calculating all possible sets of three ages, with and without an even member
    let mut even_triplets = Vec::<BTreeSet<usize>>::new();
    let mut odd_triplets = Vec::<BTreeSet<usize>>::new();

//  Complement each of these with one set of two ages, avoiding pairings with more than one even age
    let mut even_twins = Vec::<BTreeSet<usize>>::new();
    let mut odd_twins = Vec::<BTreeSet<usize>>::new();

//  This loop builds the pairs and triplets, removing potential clashes as we build
    for p in &avail_odds {
        let odds_p = filter_ages(&avail_odds, p, &pfs);
        let evens_p = filter_ages(&avail_evens, p, &pfs);

        for q in &evens_p {
            even_twins.push([*p, *q].into_iter().collect::<BTreeSet::<usize>>());
        }

        for q in &odds_p {
            if q > p {
                odd_twins.push([*p, *q].into_iter().collect::<BTreeSet::<usize>>());
            }

            let odds_q = filter_ages(&odds_p, q, &pfs);
            let evens_q = filter_ages(&evens_p, q, &pfs);

            for r in &evens_q {
                even_triplets.push([*p, *q, *r].into_iter().collect::<BTreeSet::<usize>>());
            }

            for r in &odds_q {
                odd_triplets.push([*p, *q, *r].into_iter().collect::<BTreeSet::<usize>>());
            }
        }
    }

    println!("There are {:?} pairings with one even member", even_twins.len());
    println!("There are {:?} pairings with no even members", odd_twins.len());
    println!("There are {:?} triplets with one even member", even_triplets.len());
    println!("There are {:?} triplets with no even members", odd_triplets.len());

//  Build allowed pairings of one bench of three with one bench of two
    let mut twin_triplet_pairs = Vec::<(&BTreeSet<usize>, &BTreeSet<usize>)>::new();

//  Triplets with no even age can be combined with pairs that have an even age...
    for (twin, triplet) in odd_triplets.iter().cartesian_product(&even_twins) {
        if twin.is_disjoint(triplet) {
            if twin.iter().cartesian_product(triplet)
                                                .all(|t| coprime(t.0, t.1, &pfs)) {
                twin_triplet_pairs.push((twin, triplet));    
            }
        }
    }

//  ... or with pairs that only have odd ages...
    for (twin, triplet) in odd_triplets.iter().cartesian_product(&odd_twins) {
        if twin.is_disjoint(triplet) {
            if twin.iter().cartesian_product(triplet)
                                                .all(|t| coprime(t.0, t.1, &pfs)) {
                twin_triplet_pairs.push((twin, triplet));    
            }
        }
    }

//  ... but triplets with an even age can only be combined with pairs of odd ages
    for (twin, triplet) in even_triplets.iter().cartesian_product(&odd_twins) {
        if twin.is_disjoint(triplet) {
            if twin.iter().cartesian_product(triplet)
                                                .all(|t| coprime(t.0, t.1, &pfs)) {
                twin_triplet_pairs.push((twin, triplet));    
            }
        }
    }

//  To find the highest possible value of age sum that matches the puzzle requirements, start with the largest
//  possible sum and search downwards until requirements are met
    twin_triplet_pairs.sort_by_key(|(triplet, _twin)| triplet.iter().sum::<usize>());

    println!("There are {:?} twin-triplet pairings with no more than one even member", twin_triplet_pairs.len());

    let mut age_sum = 0_usize;

    for t_t in twin_triplet_pairs.iter().rev() {

        if age_sum > t_t.0.iter().sum::<usize>() {break}
        age_sum = t_t.0.iter().sum::<usize>();

        let useable_odds = avail_odds.iter().filter(|age| !t_t.0.contains(age) && !t_t.1.contains(age))
                                                        .filter(|age| t_t.0.iter().all(|t| coprime(t, age, &pfs)))
                                                        .filter(|age| t_t.1.iter().all(|t| coprime(t, age, &pfs)))
                                                        .collect::<Vec<&usize>>();

        let useable_pairs = useable_odds.into_iter().combinations(2)
                                            .filter(|p| coprime(p[0], p[1], &pfs))
                                            .filter(|p| no_common_digits(p[0], p[1]))
                                            .collect::<Vec<Vec<&usize>>>();

        if useable_pairs.iter().flatten().collect::<BTreeSet<&&usize>>().len() < 6 {continue;}

        let three_benches = useable_pairs.into_iter().combinations(3)
                                                    .filter(|v| v.iter().flatten().collect::<BTreeSet<&&usize>>().len() > 5)
                                                    .collect::<Vec<Vec<Vec<&usize>>>>();


        if three_benches.len() > 0 {
            println!();
            println!("The largest possible sum of ages on the bench of three is {}", age_sum);
            println!("The eldest age on that bench is {:?}", t_t.0.iter().last().unwrap());
//            break;
        }
    }

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

//  None of the ages on any given bench can share a digit
fn no_common_digits(x: &usize, y: &usize) -> bool {
    let x_dig = [x / 10, x % 10].into_iter().collect::<BTreeSet::<usize>>();
    let y_dig = [y / 10, y % 10].into_iter().collect::<BTreeSet::<usize>>();

    x_dig.is_disjoint(&y_dig)
}

//  None of the ages on any given bench share a prime factor
fn coprime(x: &usize, y: &usize, pfs: &Vec<BTreeSet<usize>>) -> bool {
    let x_facs = &pfs[*x];
    let y_facs = &pfs[*y];

    x_facs.is_disjoint(&y_facs)
}

//  Filter any given set of ages to avoid clashes with a subset we have already built,
//  according to the rules for ages on any given bench
fn filter_ages(ages: &Vec<usize>, p: &usize, pfs: &Vec<BTreeSet<usize>>) -> Vec<usize> {

    ages.to_owned().into_iter()
    .filter(|age| no_common_digits(age, p))
    .filter(|age| coprime(age, p, &pfs))
    .collect::<Vec<usize>>()

}