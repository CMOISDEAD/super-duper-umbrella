public class index {
  public static void main(String[] args) {
    System.out.println(invertirpalabra("hola"));
  }

  public static String invertirpalabra(String palabra) {
    if (palabra.length() == 0) {
      return palabra;
    } else {
      return invertirpalabra(palabra.substring(1) + palabra.charAt(0));
    }
  }
}
