from collections import Counter

def string_to_line(string):
    points = map(lambda l: tuple(map(int, l.split(","))), string.split(" -> "))
    points = list(points)
    return points

def get_input():
    with open("input", "r") as file:
        raw_input = file.read()
        not_empty = lambda str: str != ''
        lines_str = filter(not_empty, raw_input.split("\n"))
        return list(map(string_to_line, lines_str))

def is_horizontal(line):
    return line[0][0] == line[1][0]

def is_vertical(line):
    return line[0][1] == line[1][1]

def is_diagonal(line):
    x1 = line[0][0]
    y1 = line[0][1]
    x2 = line[1][0]
    y2 = line[1][1]

    if (x2 - x1) == 0:
        return False
    else:
        s = (y2 - y1) // (x2 - x1)
        return s == -1 or s == 1


def get_points(line):
    points = []

    if is_horizontal(line):
        x = line[0][0]
        y1 = line[0][1]
        y2 = line[1][1]
        for i in range(min(y1, y2), max(y1, y2) + 1):
             points.append((x, i))

    if is_vertical(line):
        y = line[0][1]
        x1 = line[0][0]
        x2 = line[1][0]
        for i in range(min(x1, x2), max(x1, x2) + 1):
             points.append((i, y))

    if is_diagonal(line):
        x1 = line[0][0]
        y1 = line[0][1]
        x2 = line[1][0]
        y2 = line[1][1]

        # to facilitate working with ranges
        # range(x1, x2) where x1 > x2 is empty
        if x1 > x2:
            x1, x2, y1, y2 = x2, x1, y2, y1

        s = (y2 - y1) // (x2 - x1)
        for i, j in zip(range(x1, x2), range(y1, y2, s)):
            points.append((i, j))

        # ranges strike again
        points.append((x2, y2))

    return points

def solve(lines):
    points = []

    for line in lines:
        points += get_points(line)

    counted = Counter(points)
    result = len([v for (_,v) in counted.items() if v >= 2])
    print(result)

if __name__ == "__main__":
    ranges = get_input()
    solve(ranges)
