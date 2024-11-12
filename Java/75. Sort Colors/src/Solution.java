class Solution {
    public void sortColors(int[] nums) {
        for (int i = 1; i < nums.length; i++) {
            for (int j = 0; j < nums.length-1; j++) {
                if(nums[j] > nums[i]) {
                    var temp = nums[j];
                    nums[j] = nums[i];
                    nums[i] = temp;
                }
            }
        }
        int para = 0;
    }
}