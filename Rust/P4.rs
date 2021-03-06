fn main(){
    let mut largest: i64 = 0;
    for i in 100..1000 {
        for j in i..1000{ //optimization, reduces duplicate calculations
            let t = i*j;
            if is_palindrome(t) && t > largest {
                largest = t;
            }
        }
    }

    println!("{}", largest);
}

fn is_palindrome(mut n: i64) -> bool {
    let mut divisor: i64 = 1;
    while n / divisor >= 10 { divisor *= 10; }

    while n != 0 {
        let msd = n / divisor; //Most Signifigant Digit
        let lsd = n % 10; //Lease Signifigant Digit

        if msd != lsd {return false;}

        n = (n % divisor) / 10; //Strip LSD and MSD
        divisor /= 100; //We stripped 2 digits, so divisor loses factor of 100
    }
    true
}
