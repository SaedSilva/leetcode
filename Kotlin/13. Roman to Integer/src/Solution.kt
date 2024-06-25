class Solution {
    fun romanToInt(s: String): Int {
        if (s.isEmpty()) return 0
        if (s.length == 1) return getIntInRoman(s.toCharArray().get(0))

        val array = s.toCharArray()
        var value = 0
        var valueNext = 0
        var result = 0
        array.forEachIndexed { index, char ->
            value = getIntInRoman(char)
            if (index < array.size - 1) valueNext = getIntInRoman(array[index+1])
            if (value < valueNext) result -= value
            else result += value
        }
        return result
    }

    private fun getIntInRoman(char: Char): Int {
        when (char) {
            'I' -> return 1
            'V' -> return 5
            'X' -> return 10
            'L' -> return 50
            'C' -> return 100
            'D' -> return 500
            'M' -> return 1000
        }
        return 0
    }
}