import kotlin.math.pow
import kotlin.math.sqrt

class Solution {
    //Não solucionado | Superou o tempo limite

    fun judgeSquareSum(c: Int): Boolean {
        if (c in 1..2) {
            return true
        }
        var result = c.toDouble()
        var a = 0.0
        var b = 0.0

        for (i in 0..sqrt(c.toDouble()).toInt()+1) {
            for (j in 0..sqrt(c.toDouble()).toInt()+1) {
                a = a.pow(2.0)
                b = b.pow(2.0)

                if (a + b == result) {
                    return true
                }
                a = i.toDouble()
                b = j.toDouble()
            }
        }

        return false
    }
}