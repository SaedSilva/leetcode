import java.util.Arrays;

class Solution {
    public long pickGifts(int[] gifts, int k) {
        int maiorIndice = 0;
        int iteracoes = 0;
        long total = 0;

        while (iteracoes != k) {
            for (int i = 0; i < gifts.length; i++) {
                if (gifts[i] > gifts[maiorIndice]) {
                    maiorIndice = i;
                }
            }
            gifts[maiorIndice] = (int) Math.sqrt(gifts[maiorIndice]);
            iteracoes++;
        }
        for (int gift : gifts) {
            total += gift;
        }
        return total;
    }
}