#include <stdio.h>

int main(){
    //This is pretty trivial to implement without any fancy algorithms

    int i;
    int sum = 0;

    for(i = 1; i < 1000; i++){
        if(i % 3 == 0 || i % 5 == 0){sum += i;}
    }

    printf("%d\n", sum);
}
