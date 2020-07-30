fn main(){
    let mut t: i64;
    let mut largest: i64 = 0;
    for i in 100..1000 {
        for j in 100..1000{
            t = i*j;
            if is_palindrome(&t) && t > largest {
                largest = t;
            }
        }
    }

    println!("{}", largest);
}

fn is_palindrome(num: &i64) -> bool {
    let mut divisor: i64 = 1;
    let mut n = *num;
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
