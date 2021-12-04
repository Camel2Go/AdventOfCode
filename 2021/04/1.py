#!/bin/python

# ======== setup ===========

data = open("data").read().split('\n')

# ======== code =======

def won(board, input):
    for i in range(len(board)):
        if sum([x in input for x in board[i]]) == len(board) or sum([x in input for x in [board[n][i] for n in range(len(board))]]) == len(board):
            return True
    return False

def eval(board, input, n):
    score = 0
    for i in range(len(board)):
        for j in range(len(board)):
            if board[i][j] not in input: score += int(board[i][j])
    return score * int(n)


boards = []
board = []
for line in data[2:]:
    if not line: 
        boards.append(board)
        board = []
    else: board.append(line.strip().replace("  ", ' ').split(' '))

input = []
for n in data[0].split(','):
    finished = False
    input.append(n)
    for board in boards:
        if won(board, input): 
            print(eval(board, input, n))
            finished = True
            break
    if finished: break
