public class Main {
    public static void main(String[] args) {
        Solution solution = new Solution();

        String s = solution.longestCommonPrefix(new String[]{"flower", "flow", "flight"});
        System.out.println(s);
        String s2 = solution.longestCommonPrefix(new String[]{"ab", "a"});
        System.out.println(s2);
    }
}