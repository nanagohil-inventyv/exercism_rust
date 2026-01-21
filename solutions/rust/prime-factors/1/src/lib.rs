pub fn factors(n: u64) -> Vec<u64> {
    let mut n_cp = n;
    let mut prime_factor:Vec<u64> = Vec::new();

    while n_cp % 2 == 0 {
        prime_factor.push(2);
        n_cp  >>=1;
    }
    let mut i = 3;
    while i * i <= n_cp {
        while n_cp % i == 0 {
             prime_factor.push(i);
             n_cp /= i;
        }
        i+=2;
    }
    if(n_cp > 1){
        prime_factor.push(n_cp);
    }
    prime_factor
    
}

