const fs = require('fs');
const input = fs.readFileSync("input").toString();
const [first, ...rest] = input.split("\n\n");
const numbers = first.split(',').map(Number);
const boards = rest.filter(Boolean).map(board => board.split("\n")
    .filter(Boolean)
    .map(line => line.split(' ')
        .filter(number => number.length > 0)
        .map(Number)
        .map(n => ({val: n, marked: false}))));

function mark(board, number) {
    for (let row of board)
        for (let cell of row)
            if (cell.val === number)
                cell.marked = true;
}

function winner(board) {
    for (let i = 0; i < 5; i++) {
        let marked_row = true;
        let marked_col = true;

        for (let j = 0; j < 5; j++) {
            if (board[i][j].marked === false)
                marked_row = false;

            if (board[j][i].marked === false)
                marked_col = false;
        }

        if (marked_row || marked_col)
            return true;
    }
    return false;
}

function score(board, number) {
    const sum = board.flat()
        .filter(e => !e.marked)
        .map(e => e.val)
        .reduce((sum, e) => sum + e, 0);
    return sum * number;
}

// part 1
for (let number of numbers) {
    let found = false;
    for (let board of boards) {
        mark(board, number);
        if (winner(board)) {
            console.log(score(board, number));
            found = true;
            break;
        }
    }
    if (found)
        break;
}

// part2
function find_last(numbers, boards) {
    if (boards.length == 1) {
        // draw numbers until it wins
        const last = boards[0];
        let number = numbers[0];
        while (!winner(last)) {
            mark(last, number);
            number = numbers.shift();
        }

        return {last, number};
    }
    const current = numbers.shift();
    boards.forEach(e => mark(e, current));
    const new_boards = boards.filter(e => !winner(e));
    return find_last(numbers, new_boards);
}

const {last, number} = find_last(numbers, boards);
console.log(score(last, number));
