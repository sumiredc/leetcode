const board = [
    [ 0, 0, 0, 0, 0, 0, 0, 0],
    [ 0, 0, 0, 2, 1, 0, 1, 0],
    [ 0, 0, 0, 2, 1, 1, 0, 0],
    [ 0, 0, 0, 2, 1, 1, 0, 0],
    [ 0, 0, 0, 2, 1, 0, 0, 0],
    [ 0, 0, 0, 0, 0, 0, 0, 0],
    [ 0, 0, 0, 0, 0, 0, 0, 0],
    [ 0, 0, 0, 0, 0, 0, 0, 0]
];

const BLACK = 1;
const WHITE = 2;
const EMPTY = 0;

const around = [
    -1, 0, 1
];


for (let y = 0; y < board.length; y++) {
    const row = board[y];
    for (let x = 0; x < row.length; x++) {
        const cell = row[x];
        if (cell !== EMPTY) {
            continue;
        }
        for (const dirX of around) {
            for (const dirY of around) {
                if (dirX === 0 && dirY === 0) {
                    continue;
                }
                if (BLACK === getCell(board, x + dirX, y + dirY)) {
                    if (check(board, x + dirX, y + dirY, dirX, dirY)) {
                        console.log("x: %d, y: %d", x, y);
                        break;
                    }
                }
            }
        }
    }
}

function getCell(board: number[][], x: number, y: number) {
    if (isOut(board, x, y)) {
        return -1;
    }
    return board[y][x];
}

function check(board: number[][], x: number, y: number, dirX: number, dirY: number) {
    if (isOut(board, x, y)) {
        return false;
    }
    const cell = board[y][x];
    if (cell === BLACK) {
        return check(board, x + dirX, y + dirY, dirX, dirY);
    }
    return cell === WHITE;
}

function isOut(board: number[][], x: number, y: number) {
    return !(y in board && x in board[y]);
}