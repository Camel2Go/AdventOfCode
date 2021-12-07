#!/bin/python

# ======== setup ===========

import sys
data = open(sys.path[0] + "/data").read().split('\n')

# ======== code =======

def won(board, marked):
    for i in range(len(board)):
        if sum([x in marked for x in board[i]]) == len(board) or sum([x in marked for x in [board[n][i] for n in range(len(board))]]) == len(board):
            return True
    return False

def eval(board, marked, n):
    score = 0
    for i in range(len(board)):
        for j in range(len(board)):
            if board[i][j] not in marked: score += int(board[i][j])
    return score * int(n)

def run(boards, numbers, win=True):
    marked = []
    for n in numbers:
        marked.append(n)
        for board in boards:
            if won(board, marked):
                if win:
                    return(eval(board, marked, n))
                else:
                    if len(boards) == 1:
                        return(eval(board, marked, n))
                    boards.remove(board)
    return False


numbers = data[0].split(',')
boards = [[line.strip().replace("  ", ' ').split(' ') for line in data[i:i+5]] for i in range(2, len(data), 6)]

print(run(boards, numbers))
print(run(boards, numbers, False))