import java.util.Arrays;

class Solution {
    public int heightChecker(int[] heights) {
        int[] ordened = heights.clone();
        Arrays.sort(ordened);
        int numberOfErrors = 0;
        for (int i = 0; i < heights.length; i++) {
            if(ordened[i] != heights[i]) {
                numberOfErrors++;
            }
        }
        return numberOfErrors;
    }
}