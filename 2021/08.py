#!/bin/python

# ======== setup ===========
data = open("08.data").read().split('\n')

# ======== code =======

def mapping(pattern) -> list:
    pattern = [0, 0] + [set.intersection(*[set(x) for x in pattern if len(x) == i]) for i in range(2, 8)]
    a = pattern[3] - pattern[2]
    e = pattern[7] - (pattern[3] | pattern[4] | pattern[5])
    c = pattern[7] - (pattern[5] | pattern[6] | e)
    d = pattern[7] - (pattern[6] | e | c)
    g = pattern[5] - a - d
    f = pattern[2] - c
    b = pattern[4] - c - d - f
    return [x.pop() for x in [a, b, c, d, e, f, g]]

def decode(mapping, digit):
    digit = {mapping.index(x) for x in digit}
    if digit == {0, 1, 2, 4, 5, 6}: return "0"
    elif digit == {2, 5}: return "1"
    elif digit == {0, 2, 3, 4, 6}: return "2"
    elif digit == {0, 2, 3, 5, 6}: return "3"
    elif digit == {1, 2, 3, 5}: return "4"
    elif digit == {0, 1, 3, 5, 6}: return "5"
    elif digit == {0, 1, 3, 4, 5, 6}: return "6"
    elif digit == {0, 2, 5}: return "7"
    elif digit == {0, 1, 2, 3, 4, 5, 6}: return "8"
    elif digit == {0, 1, 2, 3, 5, 6}: return "9"



patterns = [line.split('|')[0].strip().split(' ') for line in data]
digits = [line.split('|')[1].strip().split(' ') for line in data]
print(sum([len([x for x in line if len(x) in [2, 3, 4, 7]]) for line in digits]))
print(sum([int(''.join([decode(mapping(patterns[i]), digit) for digit in digits[i]])) for i in range(len(data))]))