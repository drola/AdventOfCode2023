import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import java.util.stream.Collectors.toList

class Day19Test {
    @Test
    fun testToWorkflow() {
        assertEquals(
            Workflow(
                "qqz", listOf(
                    Rule.GoToIf(Operand.S, Operator.GreaterThan, 2770, "qs"),
                    Rule.GoToIf(Operand.M, Operator.LessThan, 1801, "hdj"),
                    Rule.GoTo("R")
                )
            ), "qqz{s>2770:qs,m<1801:hdj,R}".toWorkflow()
        )
    }

    @Test
    fun testToCase() {
        assertEquals(Case(787, 2655, 1222, 2876), "{x=787,m=2655,a=1222,s=2876}".toCase())
    }
}
