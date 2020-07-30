fn main(){
    let mut number: i64= 600851475143;
    let mut p: i64 = 1;

    while p < number {
        p = smallest_factor(number);
        number /= p;
    }

    println!("{}", p);
}

// The smallest factor will always be prime

fn smallest_factor(n: i64) -> i64{
    let limit = (n as f64).sqrt().ceil() as i64;
    
    for x in (3..limit).step_by(2) {
        if n % x == 0  {
            return x;
        }
    }
    n //n itself is prime
}
