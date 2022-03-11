#!/bin/python

# ======== setup ===========
data = open("13.data").read().split('\n')

# ======== code =======

def run(folds, dots):
    for fold in folds:
        dots = {(dot[0] - 2 * max(dot[0] - fold[1], 0), dot[1]) for dot in dots} if fold[0] == 'x' else {(dot[0], dot[1] - 2 * max(dot[1] - fold[1], 0)) for dot in dots}
    return dots

folds = [(fold.split('=')[0][-1], int(fold.split('=')[1])) for fold in data[data.index('') + 1:]]
dots = {tuple(map(int, line.split(','))) for line in data[:data.index('')]}

print(len(run(folds[:1], dots)))
dots = run(folds, dots)
for y in range(6):
    for x in range(60):
        print('#' if (x, y) in dots else ' ', end='')
    print('')