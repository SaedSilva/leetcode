//TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or
// click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
fun main() {
    val solution = Solution()

    val result = solution.haveConflict(
        event1 = arrayOf("01:15", "02:00"),
        event2 = arrayOf("02:00", "03:00")
    )

    val result2 = solution.haveConflict(
        event1 = arrayOf("01:00", "02:00"),
        event2 = arrayOf("01:20", "03:00")
    )

    val result3 = solution.haveConflict(
        event1 = arrayOf("10:00", "11:00"),
        event2 = arrayOf("14:00", "15:00")
    )
    val result4 = solution.haveConflict(
        event1 = arrayOf("14:13", "22:08"),
        event2 = arrayOf("02:40", "08:08")
    )
    val result5 = solution.haveConflict(
        event1 = arrayOf("15:19","17:56"),
        event2 = arrayOf("14:11","20:02")
    )

    println(result)
    println(result2)
    println(result3)
    println(result4)
    println(result5)
}

class Solution {
    fun haveConflict(event1: Array<String>, event2: Array<String>): Boolean {
        val eventOne: Pair<Int, Int> = Pair(stringToMinutes(event1[0]), stringToMinutes(event1[1]))
        val eventTwo: Pair<Int, Int> = Pair(stringToMinutes(event2[0]), stringToMinutes(event2[1]))

        println(eventOne)
        println(eventTwo)

        return (eventTwo.first <= eventOne.second && eventTwo.first >= eventOne.first) ||
                (eventTwo.second >= eventOne.first && eventTwo.second <= eventOne.second) ||
                (eventTwo.first <= eventOne.first && eventTwo.second >= eventOne.second)
    }

    fun stringToMinutes(value: String): Int {
        val values = value.split(":").map { it.toInt() }
        val totalMinutesFromHours = values[0] * 60
        return totalMinutesFromHours + values[1]
    }
}