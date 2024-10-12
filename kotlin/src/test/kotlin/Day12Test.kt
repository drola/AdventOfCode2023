import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertAll


class Day12Test {
    @Test
    fun testSolveLine() {
        assertAll(
            { assertEquals(1, solveLine(Line("", listOf()))) },
            { assertEquals(1, solveLine(Line("???.###", listOf(1, 1, 3)))) },
            { assertEquals(4, solveLine(Line(".??..??...?##.", listOf(1, 1, 3)))) },
            { assertEquals(1, solveLine(Line("?#?#?#?#?#?#?#?", listOf(1, 3, 1, 6)))) },
            { assertEquals(1, solveLine(Line("????.#...#...", listOf(4, 1, 1)))) },
            { assertEquals(4, solveLine(Line("????.######..#####.", listOf(1, 6, 5)))) },
            { assertEquals(10, solveLine(Line("?###????????", listOf(3, 2, 1)))) }
        )
    }
}
