#include <stdio.h>

int is_palindrome(int n);

int main(){
    int i, j;
    int largest = 0;
    int t;
    for(i = 100; i < 1000; i++){
        for(j = i; j < 1000; j++){ //optimization, reduce duplicate calcuations
            t = i*j;
            if(is_palindrome(t) == 1 & t > largest){
                largest = t;
            }
        }
    }
    printf("%d\n", largest);

return 0;
}

int is_palindrome(int n){
    int divisor = 1;

    while(n/divisor >= 10) { divisor *= 10; } //set divisor to be appropriate

    while(n != 0){
        int msd = n / divisor; //Most Signifigant Digit
        int lsd = n % 10; //Least Signifigant Digit

        if (msd != lsd){
            return 0;
        }

        n = (n % divisor) / 10; //remove MSD and LSD
        divisor /= 100; //because we stripped 2 digits, we need to reduce by 100
    }    

return 1;
}
