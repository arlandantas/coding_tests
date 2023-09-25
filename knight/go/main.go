package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Position struct {
	y int
	x int
}

const KNIGHT_CHAR = "K"
const BLOCKED_CHAR = "#"

var TARGET_POSITION = Position{0, 0}

func getBoard() (board [][]string, blockedPositions []Position, knightPosition Position) {
	var size int
	fmt.Scan(&size)
	reader := bufio.NewReader(os.Stdin)

	blockedPositions = make([]Position, 0)

	board = make([][]string, size)
	for i := 0; i < size; i++ {
		line, _ := reader.ReadString('\n')
		line = strings.Replace(line, "\n", "", -1)
		if (len(line) - 1) != size {
			line = line + strings.Repeat(".", size-len(line))
		}
		board[i] = strings.Split(line, "")[:size]
		for j := 0; j < size; j++ {
			switch board[i][j] {
			case KNIGHT_CHAR:
				knightPosition = Position{i, j}
			case BLOCKED_CHAR:
				blockedPositions = append(blockedPositions, Position{i, j})
			}
		}
	}

	return
}

func getTargetSteps(board [][]string, blockedPositions []Position, knightPosition Position) (steps int) {
	queue := make([]Position, 0)
	queue = append(queue, knightPosition)
	positionMinSteps := make(map[Position]int)
	positionMinSteps[knightPosition] = 0
	for len(queue) > 0 {
		currentPosition := queue[0]
		steps = positionMinSteps[currentPosition]
		queue = queue[1:]
		if currentPosition == TARGET_POSITION {
			return
		}
		for _, direction := range []Position{
			{currentPosition.y + 2, currentPosition.x + 1},
			{currentPosition.y + 2, currentPosition.x - 1},
			{currentPosition.y - 2, currentPosition.x + 1},
			{currentPosition.y - 2, currentPosition.x - 1},
			{currentPosition.y + 1, currentPosition.x + 2},
			{currentPosition.y + 1, currentPosition.x - 2},
			{currentPosition.y - 1, currentPosition.x + 2},
			{currentPosition.y - 1, currentPosition.x - 2},
		} {
			if direction.y >= 0 && direction.y < len(board) {
				if direction.x >= 0 && direction.x < len(board) {
					if _, alreadyVisited := positionMinSteps[direction]; !alreadyVisited {
						if board[direction.y][direction.x] != BLOCKED_CHAR {
							queue = append(queue, direction)
							positionMinSteps[direction] = steps + 1
						}
					}
				}
			}
		}
	}
	steps = -1
	return
}

func main() {
	board, blockedPositions, knightPosition := getBoard()
	targetSteps := getTargetSteps(board, blockedPositions, knightPosition)
	fmt.Println(targetSteps)
}
