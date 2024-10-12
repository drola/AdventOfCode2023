import java.io.File
import java.util.Scanner
import kotlin.math.max

data class Line(val springs: String, val groups: List<Int>)

data class State(val stringPosition: Int, val groupPosition: Int) {
    fun isValid(): Boolean {
        return stringPosition >= groupPosition;
    }
}

fun trim(line: Line, stringPosition: Int, groupPosition: Int): State {
    var currentStringPosition = stringPosition;
    while (currentStringPosition >= 0 && line.springs[currentStringPosition] == '.') {
        currentStringPosition--;
    }
    return State(currentStringPosition, groupPosition);
}

fun consumeGroup(line: Line, stringPosition: Int, groupPosition: Int): State? {
    if (groupPosition < 0 || line.groups.isEmpty()) {
        return null;
    }
    var remainingGroupLength = line.groups[groupPosition];
    var currentStringPosition = stringPosition;
    while (remainingGroupLength > 0 && currentStringPosition >= 0) {
        if (line.springs[currentStringPosition] == '.') {
            return null;
        }
        remainingGroupLength--;
        currentStringPosition--;
    }
    if (remainingGroupLength > 0) {
        return null;
    }
    if (currentStringPosition >= 0 && line.springs[currentStringPosition] == '#') {
        return null;
    }

    return trim(line, max(-1, currentStringPosition - 1), groupPosition - 1);
}

fun recursion(line: Line, stringPosition: Int, groupsPosition: Int, cache: MutableMap<State, Long>): Long {
    var state: State = trim(line, stringPosition, groupsPosition);

    // Termination: Invalid state
    if (state.isValid() != true) {
        return 0;
    }
    // Termination: End of string
    if (state.stringPosition == -1 && state.groupPosition == -1) {
        return 1;
    }

    if (cache.contains(state)) {
        return cache.getValue(state)
    }

    var combinations: Long = 0;
    // a) make next one a "."
    if (state.stringPosition >= 0 && line.springs[state.stringPosition] != '#') {
        combinations += recursion(line, state.stringPosition - 1, groupsPosition, cache)
    }
    // b) make next on a "#"
    var consumedState = consumeGroup(line, state.stringPosition, state.groupPosition);
    if (consumedState != null) {
        combinations += recursion(line, consumedState.stringPosition, consumedState.groupPosition, cache);
    }
    cache.put(state, combinations);
    return combinations;
}


fun solveLine(line: Line): Long {
    var cache = mutableMapOf<State, Long>()
    return recursion(line, line.springs.length - 1, line.groups.size - 1, cache);
}

fun parseLines(filename: String): List<Line> {
    var lines = ArrayList<Line>()
    var scanner = Scanner(File(filename))
    scanner.useDelimiter("[, \n]")
    while (scanner.hasNextLine()) {
        var springs = scanner.next("[#.?]+")
        scanner.skip(" ")
        var groups = ArrayList<Int>()
        while (scanner.hasNextInt()) {
            groups.add(scanner.nextInt())
        }

        lines.add(Line(springs, groups))
    }
    return lines
}

fun unfoldLine(line: Line): Line =
    Line(
        generateSequence { line.springs }.take(5).joinToString("?"),
        generateSequence { line.groups }.take(5).flatten().toList()
    )


fun main(args: Array<String>) {
    var lines = parseLines(args[0])
    var totalCombinations: Long = 0;
    for (line in lines) {
        println(line);
        var combinationInLine = solveLine(line);
        println("Combinations: $combinationInLine");
        totalCombinations += combinationInLine;
    }

    println("Total combinations: $totalCombinations");


    totalCombinations = 0;
    for (line in lines) {
        var unfolded = unfoldLine(line);
        println(unfolded);
        var combinationInLine = solveLine(unfolded);
        println("Combinations: $combinationInLine");
        totalCombinations += combinationInLine;
    }

    println("Total combinations: $totalCombinations");
}
