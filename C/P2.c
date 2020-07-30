#include <stdio.h>

int main(){
    int a = 1;
    int b = 1;
    int t;
    int sum = 0;

    //iterative fibbonacci algorithm 
    while(b < 4000000){
        t = a;
        a = b;
        b = a + t;

        if(b % 2 == 0){
            sum += b;            
        }    
    }

    printf("%d\n", sum);
return 0;
}
