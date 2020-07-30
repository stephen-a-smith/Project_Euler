public class P2{
    public static void main(String[] args){
        int a = 1;
        int b = 1;
        int t;
        int sum = 0;

        while(b < 4000000){
            t = a;
            a = b;
            b = a + t;

            if (b % 2 == 0){ sum += b; }
        }

        System.out.println(sum);
    }
}
