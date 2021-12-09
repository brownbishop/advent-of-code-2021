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
