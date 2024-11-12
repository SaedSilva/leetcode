import java.util.*;

class Solution {
    public String shortestCompletingWord(String licensePlate, String[] words) {
        licensePlate = RemoveAndLowerCase(licensePlate);
        if(licensePlate.length() == 1) {
            
        }
        String[] wordsOrder = words.clone();
        for (int i = 0; i < wordsOrder.length; i++) {
            wordsOrder[i] = RemoveAndLowerCase(wordsOrder[i]);
        }

        Map<String, Integer> map = new HashMap<>();
        char[] compare = licensePlate.toCharArray();
        int count = 0;
        for (int i = 0; i < wordsOrder.length; i++) {
            count = 0;
            char[] wordsOrderChar = wordsOrder[i].toCharArray();
            for (int j = 0; j < compare.length; j++) {
                for (int k = 0; k < wordsOrderChar.length; k++) {
                    if (compare[j] == wordsOrderChar[k]) {
                        wordsOrderChar[k] = '0';
                        count++;
                        break;
                    }
                }
            }
            map.put(words[i], count);
        }
        String key = Collections.max(map.entrySet(), Map.Entry.comparingByValue()).getKey();
        return key;
    }

    private String RemoveAndLowerCase(String licensePlate) {
        licensePlate = licensePlate
                .toLowerCase()
                .replace(" ", "");
        for (int i = 0; i < 10; i++) {
            licensePlate = licensePlate.replace(Integer.toString(i), "");
        }
        List<Character> characterList = new ArrayList<>();
        for (char c : licensePlate.toCharArray()) {
            characterList.add(c);
        }
        Collections.sort(characterList);
        StringBuilder result = new StringBuilder();
        for (Character character : characterList) {
            result.append(character);
        }
        return result.toString();
    }
}