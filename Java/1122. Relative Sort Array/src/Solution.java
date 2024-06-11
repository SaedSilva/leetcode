import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

class Solution {
    public int[] relativeSortArray(int[] arr1, int[] arr2) {
        List<Integer> result = new ArrayList<>();
        List<Integer> diff = new ArrayList<>();

        for (int i = 0; i < arr2.length; i++) {
            for (int j = 0; j < arr1.length; j++) {
                if(arr2[i] == arr1[j]) {
                    result.add(arr1[j]);
                    arr1[j] = -1;
                }
            }
        }
        for (int i = 0; i < arr1.length; i++) {
            if(arr1[i] != -1) {
                diff.add(arr1[i]);
            }
        }
        Collections.sort(diff);
        result.addAll(diff);
        int[] result2 = new int[result.size()];
        for (int i = 0; i < result.size(); i++) {
            result2[i] = result.get(i);
        }

        return result2;
    }
}