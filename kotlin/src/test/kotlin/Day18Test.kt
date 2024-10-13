import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test


class Day18Test {
    @Test
    fun testParseInstruction() {
        assertEquals(
            Instruction('R', 6),
            parseInstruction("R 6 (#70c710)")
        );
    }

    @Test
    fun testParseInstructionHex() {
        assertEquals(
            Instruction('R', 461937),
            parseInstructionHex("R 6 (#70c710)")
        );
    }
}
