import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] teste1 = new int[] {2,1,3,5,6};
        System.out.println(Arrays.toString(solution.getFinalState(teste1, 5, 2)));
    }
}