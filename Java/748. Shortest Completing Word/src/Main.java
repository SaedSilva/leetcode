public class Main {
    public static void main(String[] args) {
        Solution solution = new Solution();
        String[] words = {"step", "steps", "stripe", "stepple"};
        String licensePlate = "1s3 PSt";
        String resul = solution.shortestCompletingWord(licensePlate, words);
        System.out.println(resul);
    }
}