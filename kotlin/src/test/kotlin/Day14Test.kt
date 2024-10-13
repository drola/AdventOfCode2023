import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test


class Day14Test {
    @Test
    fun testSlideNorth() {
        var input = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....""".trimMargin();
        var expectedOutput = """OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....""".trimMargin();
        assertEquals(expectedOutput, toString(slideNorth(fromString(input))));
    }

    @Test
    fun testRotateCW() {
        var input = """
            |123
            |456
            |789""".trimMargin();
        var expectedOutput = """
            |741
            |852
            |963""".trimMargin()
        assertEquals(expectedOutput, toString(rotateCW(fromString(input))));
    }

    @Test
    fun testQuarterCycle() {
        var input = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
""".trimMargin();
        var expectedOutputFullCycle = """.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
""".trimMargin();
        assertEquals(
            expectedOutputFullCycle,
            toString(quarterCycle(quarterCycle(quarterCycle(quarterCycle(fromString(input))))))
        );
    }
}
