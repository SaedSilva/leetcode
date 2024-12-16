class Solution {
    public int[] getFinalState(int[] nums, int k, int multiplier) {
        if (k == 0) {
            return nums;
        }
        int menorIndice = getMenorIndice(nums);
        k--;
        nums[menorIndice] *= multiplier;
        return getFinalState(nums, k, multiplier);
    }

    private int getMenorIndice(int[] nums) {
        int indice = 0;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] < nums[indice]) {
                indice = i;
            }
        }
        return indice;
    }
}