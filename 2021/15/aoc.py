#!/bin/python

# ======== setup ===========
import sys
data = open(sys.path[0] + "/data").read()
# ======== code =======

def dijkstra(data, line):

    target = len(data) - 1
    visited = set()
    shortest = {0: 0}

    while shortest:

        current = min(shortest, key=shortest.get)

        if current > line - 1 and current - line not in visited and current - line not in shortest: shortest[current - line] = shortest[current] + data[current - line]
        if current % line and current - 1 not in visited and current - 1 not in shortest: shortest[current - 1] = shortest[current] + data[current - 1]
        if (current + 1) % line and current + 1 not in visited and current + 1 not in shortest: shortest[current + 1] = shortest[current] + data[current + 1]
        if current <= target - line and current + line not in visited and current + line not in shortest: shortest[current + line] = shortest[current] + data[current + line]

        if current == target: return shortest[target]

        visited.add(current)
        del shortest[current]


line = data.index('\n')
data = [int(x) for x in data if x != '\n']
print(dijkstra(data, line))
newdata = []
for y in range(line * 5):
        for x in range(line * 5):
                newdata.append(((data[(y % line) * line + (x % line)] + x // line + y // line) -1 ) % 9 + 1)
print(dijkstra(newdata, line * 5))