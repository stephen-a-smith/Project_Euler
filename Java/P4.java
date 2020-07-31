public class P4 {
    public static void main(String[] args){
        int largest = 0;    
    
        for(int i = 100; i < 1000; i++){
            for(int j = i; j < 1000; j++){// optimization, reduces calcuations
                int temp = i*j;
                if(is_palindrome(temp) && temp > largest){
                    largest = temp;
                }
            }
        }
        System.out.println(largest);
    }

    static boolean is_palindrome(int n){
        int divisor = 1;
        
        while(n / divisor >= 10){ divisor *= 10; }

        while(n != 0){
            int msd = n / divisor; // most signifigant digit
            int lsd = n % 10; //least signifigant digit

            if(msd != lsd){ return false; }

            n = (n % divisor) / 10; //strip msd and lsd
            divisor /= 100; //drop divisor by factor of 100 (2 digits)
        }
        return true;
    }
}
