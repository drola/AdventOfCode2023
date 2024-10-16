fun rotateCW(data: Array<CharArray>): Array<CharArray> {
    // transpose
    for (x in 0..<data.size) {
        for (y in x..<data[x].size) {
            var t = data[x][y];
            data[x][y] = data[y][x];
            data[y][x] = t;
        }
    }

    // mirror Y
    for (y in 0..<data.size) {
        data[y].reverse();
    }

    return data;
}

fun fromString(data: String): Array<CharArray> =
    data.split("\n").map { line -> line.toCharArray() }.toTypedArray()

fun toString(data: Array<CharArray>): String = data.map { row -> row.joinToString("") }.joinToString("\n")

fun slideRight(data: Array<CharArray>) {
    for (line in data) {
        var lastWallIndex = -1;
        while (lastWallIndex < line.size - 1) {
            // go to next wall index
            var countRound = 0;
            var nextWallIndex = lastWallIndex;
            for (i in lastWallIndex + 1..line.size) {
                if (i == line.size || line[i] == '#') {
                    nextWallIndex = i;
                    break;
                } else if (line[i] == 'O') {
                    countRound++;
                }
            }

            line.fill('.', lastWallIndex + 1, nextWallIndex - countRound);
            line.fill('O', nextWallIndex - countRound, nextWallIndex);

//            println("lastWallIndex: $lastWallIndex, nextWallIndex: $nextWallIndex");
//            for (i in lastWallIndex + 1..<(nextWallIndex - countRound)) {
//                println("line[$i] = '.'");
//                line[i] = '.';
//            }
//            for (i in nextWallIndex - countRound..<nextWallIndex) {
//                line[i] = 'O';
//            }

            lastWallIndex = nextWallIndex;
        }
    }
}

fun slideNorth(data: Array<CharArray>): Array<CharArray> {
    rotateCW(data);
    slideRight(data);
    rotateCW(data);
    rotateCW(data);
    rotateCW(data);
    return data;
}

fun quarterCycle(data: Array<CharArray>): Array<CharArray> {
    rotateCW(data);
    slideRight(data);
    return data;
}

fun calculateLoad(data: Array<CharArray>): Long {
    var load: Long = 0;
    for (line in data) {
        for (i in 0..<line.size) {
            if (line[i] == 'O') {
                load += i + 1;
            }
        }
    }
    return load;
}

fun main() {
    var data = fromString(
        """#....#O##.....O.#...##...#OO.....##...#O...O..........OO......#..OO..O.O...O.O#O#..O...#.......O...#
O.....O#..O#O...OO.#.....O#.....O.O..O.O..O.O#.O.#...#...O.#.........#..#O.#......#O.O...#..#.##.##.
......##..#.O#..#.O.O......#.O#..O....#.#..#...#.O.#....#....O.#...#O#OOO.##...O............OO#.#..#
.OO.................#.#.O.#O.O.#...#..........##.O..#.O##..##O......#O...#.#...O...#......O.........
#.......O#...#.OO.##.O.......#...O......O#......#..#.#..O...##..O#..#O.O.....#..OO..O.O.....O.....O#
.O..O.OO...#...#O####...##.O.#..O......#O.O..O......O.......O....O....#..O...O.....OOO..#...O.....OO
OO.#O.....#.O.#O.#OOO...OO#.........#..O.#O.#O..##O...#.##..#.....#...#.....O...O...............O.OO
...OO..#....O.#O.....#.#.#..O...O.#....#....#O...O.........O..##O....O#.O.......OO#.O.O..O......OO..
...O....#....#.....O...O.#.O.O...O..#..#.O.....O.#....O#O#.##O....O...#....#......#.....#..O....#..O
OOOO#..##...##.....OO..O##.O.OOO#..O...O.#....O..OO##......O..#.O.#.....O.O#.#.#.O....#..........#O.
#.OO#O.O..#.O....#..O.OO#..OO...O.O...O.#.O...##.O.....O..OO.#O..#...O.##..#......O#O#OO#...##O.....
..#..##...O#.OO#.O.O....###O.O......#....#O..O..O.....O#O.#.....O#....O...#OO.....O###O.....O.OOO.O#
O.........OO..O...#.O.#O.#..O...O#.....O#.O...O.OO#.#.#...#...##...O#.OO.OO....O.OOO........#..OO.##
.....O..O........OO...OO#O..O.................#...#..#....#.O..#..O#..O..#....O##..#...#O.O.OO#O...#
....OO.#OO...O.O.OO.##..OO....O#....O......O..O....O......O...OOO.OO...#.......O.OO....O...OOO#...#.
....O......##..O.#O...O.O.O#.##..O.....OO.#..O..O..#.#.O..OO#..#O..#..O##.##..O#.O.O#.O#O#.O.#....O.
..O#.O.O#.##....##.O#..O....#O.O#...O....O..O###..#..OO.O..O.....O##..#...#.##.............OOO...OO.
O....##.#OOO...#....#...#....#.OO..#.O....O....O.....O##..O.....#....O.OO.O#.O.O.#.#.O#..##.O.O...O.
..#.O......#..O.....O.O.......O...O#O.O...O..O......#......##..##....OO......OO..#..O#.O#O#O.#..#...
...#O...O..O......O.#......OO..O..#O..#O#O...O.......OO#.#OO..#O....O.......#O.....#....OO....O....#
...O.OOOOO..#..O.O.O#..#....O#...##.#..#.O.O.O.......O.....#....#O.O.#O......#.OO...OOO...#....##O..
.#.......#.O...##..O.OO....O..#.#..#..O..O...##O.OO#.#.........O........O#.#..#O.....OOO##.#..O...OO
...O.....#..O#..##OOO....O.#..O.....OO.##..#OO.#...#O#..O#.OO..#.O....#OO.......O.OO.O.#O##..O.O.O#.
...#...###..#..#....#O#..#O......##.O.####.#.....O.O.......O#.#.O#.O..#..#.#...O...#...#O.OO...O....
#OO.#O..........#.OOO.....O...##..O.O..O#O.O##O...#...#....#....O.O.#.#O...O.#..O#.OOO..#..#.O...O..
.O....#OOO..O..#.#.O.#OOO.......O.#..#...#OO.##.#..OOOO...#.#..#....#..#....O.....#.#.OO....O.......
.OO...O#..#............O...#.#.OO.O#OO....#O..#....O.#O.OO......O.O.....O...O.#.....OO.##..#...#O.O.
..O..O#O.#OOO....#.OO....O....O..#.#.......#...............O...O....OO.O..#O...OO...#OO........OO...
#OO.O...O#.O.O.O#O.#...O.O......O#.O....#O.....O..#O#..O...O.OO.O.#O...O..O..#...O.##.O...OO..O.#O..
#..O...#O...OO..O..O.O.O...#.......#.....O.O....#...#..O....#.......##..O.O#.....O........#.....#..O
.#....O..O.........#..O...#....#O#..#...O.O.OOO...O...#.....##...##.#...OO#O..O....O..O...O#........
...O...O.O..#.#..#O........#####.O#....#O##OOO.O..O.O#.O.......OOOOO....#.##O..#...#.........O..O.OO
..#..#...OO#.OO..O....##.O..OO.O.O.#.#O#.O#...O...O.O.#O......OO..O.#.#..O.OO.......###OO.##O....O..
........##.O##O#..#......#.....O.O...#..#O#.........#....O...#..O..#....#..#.....O...O.O.O.#..##O.O.
.....O...O##...#.#...#O#....OO.....#O.O.O.#.#.#.#.....O..#..O.O.#..#.OOO....OO#...OO.O..O#O...O.#.#O
###....O.O..#O......OO.#.OO.##.O...#O.#...O.#.O....O....O....#..#...O..##.#..O.O...O...#.#.#.#.O#...
....O#..OO#..#...#.O.OO.O##.O..O...O...#..#.OO.O.O....##....O..O.O.OO...##O..O.#O...O#..#....O#O....
#....#....O.#.O...OO....O....#......#O###O##.......O..OO#O...#O#..O...O....##...#..###.O.#..........
#O#.#.#.....O.....#.#.O....O....OO#O......O#.#.OO#.....#.OO.O...OO##.O....O.O##...#.............#O..
..#.#..###.....O.O..O......O....O....#O.OO.O....OO..#.#O#......O..#..O.##.#.##..O.OO...O......##...#
...#.O..#.OO.........O..O.#O......#O..#..O#.O#......O#..OOO.O##....O....#.##O...O.........#O.##...O.
.....#O..O.#OO.O....#..OO..O#..O..O#..#.OO.#..O.O...O.....#.#O....O....O##.O.O..##O.O.O..O....#.O#..
##.O.O...##..O..O..#O..#.#..O#.#.O.....#..O.##.....O##..#O.#.....O....O....#.OOO#.....O...O....O..#.
.....#.O#.O#.O....O.#...#....#.#.....#..#O..#.O......O.O.O...##..OO..O.#..#..#.#...#.#...........OO.
....O#.#.O..#....O....#O.O.OO##O.........##O..#O.OO.OO..#....O.O#..###.O......##.O..#.#O..#.....#...
....O.O.#.O....#O.....##....#.O......O.......#..#.O.#..#O.#.OO.#.OO..##..O..#O...O.O.OO......O#O....
OO..O..O...#.......##OO.........#....O.#.OOO.O..OO..O.....#...O..#..O.....O..#...O.OO.....O#.O.O.O.#
.OO#......#O.#......O....#.#..#O..O...#.#.........OO..........O..O.....O.O..O##.O...O.O...#.........
..#.#OO.OO........#O..O...#O#OO...#..O..#.##.OO.#.....O..O.OOO..#..#.#....OO...##.OO..O#....O....O.O
.#.#.#..O.OO..O......O...O#O.OO.......#...O...O.#O...OO..O#.O.#.##............##.OOO#.OOO..#O.O#...O
......O...##OO.....#.##O#....#OO.O....#.#....#.....O.O......#...O.O#.##...O#............#....O#O.O.O
.O....#...O....#.##O.....O##.......#...O.....#....O.#O#.O..OO#O...#.........#...OO.O##..#..##....#.#
##O.#O......O..#.#...##.##.O##.#.....#OO#..#.........#O.O...#......#..O..O.O##..O.OO......O#O#O#..#.
......#.O#O.O...#O.OO.....O...O##.#.OO#..OO....#O..#....#.....O.#O.O.#O#..OO..O..#O.#.....O#......##
....###.......OO#O...O..#..#.#.....#.#...O..OO#.O..#OO#.O..O....OO.O.#.##O..#..O...OO.#.O.#......#..
#O.O....#...#.O..O..####....O#........O....O#..O..O..OO..OO...OOO..O..OOO......#.#....O..##.O#..#O##
..........O.#.O.O#..#.O..O#.#....#...#.##OO..OOO.....O##O#....#O.#....#.O...#O#.O.#.##..#O.O#.......
.....#...#O#......O#.O..........#O.O.OO..O..OO.#.............O#.#........O#.OOO##O...O.O#.#...#.....
O..O..O.........OO#...O.O...O....#.#.#...OO#...OOO...O........#.#O.OO#.O.....O.O..O#.OOO.O...O.OO...
OO#....#..O.#.O.O##O##.........#..O..#.O.##..O.##.#.OO..O.....##..#......OO..#...#...O..O..#.OO.#.O.
#.O....#..#.....#.##.#...O.O...O.O...O#...#...O..O...#...#..O.#.O#.OO.#....O....O#..#.O##...#...#..#
##...O..#.#......#O.O...#O.....#......OO...#OO..O......#.#..........O#..O..O..O.O.O....O...O...#O..#
..##.#O.#.OOO..O...O..O.O..O...##.....OOO#..O......#..O...........#O......O..#OO#..O.O.##....OO..O#.
.#..............O#..#..O#O...OO###.##....O.#.##..###.#O.#....#...#.OO.O#....O..OO..#OO#.O..........O
...#.#...#OO........#.O..#.O...#.O.O.#.....#......O#......#....#.##O.###.O....##.....O#OO.O....O##..
.O.O......O..#.....O......##O#..#..OO..#.#..O...#.O..#O.O#OO..#..O#.O....#...#O..O..#..O#.O.....O.OO
O.#.O.OO.OO.O......O..#.O...O#O..OO......O...#......O..O#O..##O.#OO....O......OO......#O......#...O.
.#O..O.#O..O.O#..O.#...#...O##O....#O.O##..#OO.....O..OO#.O##...#....#..##..OO.OO#..O...OO.....#.#.#
O#.OOO.#O..O.O#.O#O......OO...#O....O............#..#........#.O##..#O.O###.OO..#...#.....O........O
O......O.OO..#.#..#.......OOO.#.#........................O#O.#..O..........##.#..O#.......#.#....O..
O.#O...O#.#O.......O##O..#..O.#.#O.O.O...#OO..O.O##O.....O....OOO...##..OO....#.#..##...#..O.#.O.#.O
#..#.#O.#..##....OOO.....O......O.#O..#O..O.#O.O.#....O...O#.OO##..#...O###....###..#..O.#.O...#....
...O.......O.O.......#.#OOO....O.......#..O...O.........##..#.O.#O.O......#..OO.O.O......#.....OO...
O.....#O.#.#....#..#.O.OOO...#.O###......OO...O.O.O.###...O.O.......##..#......#..OO.O.....OO...O.OO
O.O.....#.#.#....O#OOO.....O.....#O...##O..#OO.O.......O.O.......#.O.#.........#..........O...#.OOO.
O....OO.#.#O##.O....#O#.OO..O...OO.#.#O.#.#.O...O#O...OO.#.O...##...OO..#OO.#......#O......#....O.O.
O...O......O..#..#.#...##..O..#.O.O......#....#.....O#O.O....O.#..O..............#.O...#...OO.....O#
....OO.......O...##O..#.O#..O#..O..O#.#.O#.#..O.O..#.......#O..OO.O.#....#..#.......OO..OOO.#O#...O.
.#.....O......O..O...O....#...OOO#...O#.#.#.#.OO.O.O...#..O..O#......#.O.....O.O.O....#...O#.....#.O
...O#........O......OOO..#.OO#.O.#.O..##......#.#O...O.O...O.#O.OO.OO..#..O...O.........#.O##..OO##.
..O..O.#O..#O..##....O.O.#O...O#.O.....##.....O......#...#.....OOO..O##.O#..#.#.O.......#O...O#...##
...O.OO..O.#.OO#....#.O#O..O...#O.O.###...##...O.O..#..O..#.O....#........#...#....O...O....#O#O.O.#
...OO.#...O...OOO..O.....#..#.O.#OO...O...O..#..O..O.O...O..#.#O.#OO.O.....O.O..#...O..O..O#.O#O...#
O...#...##.OO..OO.......#..O.O..OO.OO.O#......#.O#O.O.#..O.#.O.O.O#OO...#.....##O.OO.#OO....O...###.
O.............#.#..#.O....#....O...#..O..O#.O...OO.O.O........O#.O........OO..#....O...#O#O#.O.#OO.O
OO.........O...#.O.O...#.....O...#.OO#O.##............#....O....##.##..#.O..O....OO...OOO..O........
O.....#....O.#O..OO.O#.......#.....#....#.....##OO#..O..#OO.....#.....#..OO#..O#..O.O#OO.O.O...O.OO.
OO#....O....O#....#O#.....#.....O.#O#..OO.OO.O..#..O....#O..O#.#O#..OOO...##...#..#...O..O...O#..O#.
O......O#...O.#......O.O..#..O.#.....O...O...OO###...#.OO......O..O....#..O#.O....#..#.#..#.OO..O...
...#...#O#O.......#.O#.O....OOO.O..OO#.O.O..O#....O....#OO..O.......O.#..#...#.....O...O.O..O..O.#..
.O#..#.O#.#O#.O#..O..#....#O#..#.O.##.O.OOOO.#O..O#....O..OOO.......OO.#...O..O#O..O#...#O...O.#....
O.#.O.#.O...O.#.......#O.O#O.##..#...#...O#...O#O#O.#O...O..O.....#.O.##OO.O...O#.#...O.O#...#..O###
O.......O#..#......#....OO.O.#O..##O...OOO.O....O..#O....#.#..##OOO.OO#.....#..O.O...#O...O..O.O..OO
...O..O#O.O..O...##.....O.O.#O........O.###O#..#...#.OO.#.O.....OOO.#.......O..O.OO..O#...O.....O#..
........#..O#..#O.#OO.O.O..#.##..#O..#.O......O.O.#O.O.......#O#..O..##..O#.......O.#..O.#.O.O.O#O..
#..O....O.....#.O....O..##....O...O........#O....#...#..O....#....#..#O.O.......O.OOO..#...OO...#.#.
.O.O..#OO..O..O.O.........##..O...........OO..O.O.....OO...O#.#.......#...O#O...#..O.O...O.O#...#...
O.....#O.......O#..O..O..##O..#.......#...#.O......O...O..#...O..O..OO.#O..O..O.#......OO...#...OO#O
..###O.........O..#.#.#.#.O..O.#....#...#..O##O#..#O#.O#OO.O.O....O.....#O...#..O..#O..O....#.....#.
....O#.O..#......#..OOO..#.O.O...O.#O.......O.O.#####..#.....#.#.#....#...O#..O.O.O.O...O..#.......O
"""
    );

    var map = HashMap<String, Long>();
    var i: Long = 0;
    while (i < 4_000_000_000) {
        var s = toString(data);
        if (map.contains(s)) {
            var previousStep = map.getValue(s);
            println("State in step $i was seen in step $previousStep");
            var cycle = i - previousStep;
            var remainingSteps = 4_000_000_000 - i - 1;
            var stepsToSkip = (remainingSteps / cycle) * cycle;
            i += stepsToSkip;
        } else {
            map.set(s, i);
        }

        i++;
        quarterCycle(data);
    }
    rotateCW(data);
    var load = calculateLoad(data);
    println("Load: $load");
}

