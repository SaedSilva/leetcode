public class Solution {
    public String longestCommonPrefix(String[] strs) {
        StringBuilder result = new StringBuilder();
        int length = strs.length;
        String prefix = strs[0];
        for (int i = 0; i < prefix.length(); i++) { // Ponteiro dos caracteres
            char result1 = prefix.charAt(i);
            for (int j = 1; j < length; j++) { // Ponteiro das strings
                if (i == strs[j].length() || result1 != strs[j].charAt(i)) {
                    return result.toString();
                }
            }
            result.append(result1);
        }
        return result.toString();
    }
}
