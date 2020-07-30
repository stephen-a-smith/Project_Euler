fn main(){
    let mut a: i64 = 1;
    let mut b: i64 = 1;
    let mut t: i64;
    let mut sum: i64 = 0;

    //Iterative Fibbonacci algorithm
    while b < 4000000{
        t = a;
        a = b;
        b = a + t;

        if b % 2 == 0 {
            sum += b;
        }
    }

    println!("{}", sum);
}
