import java.util.*;

public class Solution {
    public boolean isValid(String s) {
        if (s.isEmpty()) {
            return false;
        }
        Stack<Character> characters = new Stack<>();
        char[] charArray = s.toCharArray();
        for (char c : charArray) {
            switch (c) {
                case '(', '{', '[' -> characters.add(c);
                case ')' -> {
                    if (characters.isEmpty() || characters.pop() != '(') return false;
                }
                case '}' -> {
                    if (characters.isEmpty() || characters.pop() != '{') return false;
                }
                case ']' -> {
                    if (characters.isEmpty() || characters.pop() != '[') return false;
                }
                default -> {
                    return false;
                }
            }
        }
        return characters.isEmpty();
    }
}
