public class index {
  public static void main(String[] args) {
   System.out.println(invertiNumeroRecursivo(254, 0)); 
  }

  public static int invertiNumeroRecursivo(int num, int pos) {
     if (num < 10) {
      return num;
     } else {
       return (num%10) * (int)Math.pow(10, pos) + invertiNumeroRecursivo(num/10, pos - 1);
     }
  }
}
