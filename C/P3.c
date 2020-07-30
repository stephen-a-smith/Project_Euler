#include <stdio.h>
#include <math.h>

long smallest_factor(long n);
int main(){
    long p = 1;
    long number = 600851475143;
    while(p < number){
        p = smallest_factor(number);
        number /= p;
    }        

    printf("%ld\n", p);
return 0;
}

//The smallest found factor of a number will always be prime
long smallest_factor(long n){
    int i;
    int limit = floor(sqrt(n));
    for(i = 3; i < limit; i+=2){
        if(n % i == 0){return i;}
    }
    return n; // Means that N is a prime
}




