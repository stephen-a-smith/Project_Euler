public class P3 {
    public static void main(String[] args){
        long number = 600851475143L;
        long p = 1;

        while(p < number){
            p = smallest_factor(number);
            number /= p;
        }

        System.out.println(p);
    }

    //Smallest factor will always be prime
    public static long smallest_factor(long n){
        int limit = (int) Math.ceil(Math.sqrt(n));
        for(int i = 3; i < limit; i+=2){
            if(n % i == 0) {
                return i;
            }
        }
        return n; // n itself is prime
    }
}
