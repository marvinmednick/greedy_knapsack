#!/usr/bin/env python
import sys
import argparse
import re
import logging
import os


logging.basicConfig(level=os.environ.get("LOGLEVEL", "INFO"))
log = logging.getLogger("knap")
#log.setLevel(level)
log.info("Info enabled")
log.debug("Debug enabled")

parser = argparse.ArgumentParser()
parser.add_argument("file",type=argparse.FileType('r'))
args = parser.parse_args()


file1 = args.file
# read the first line
line1 = file1.readline().strip()
line1_values = re.match("(?P<size>\d+)\s+(?P<num_vertex>\d+)",line1)
knapsack_size = int(line1_values.group('size'))+1
num_vertex = int(line1_values.group('num_vertex'))

print(f"size {num_vertex} {knapsack_size}")
Lines = file1.readlines()

vertex = []

vertex_regex = re.compile("(?P<value>\d+)\s+(?P<weight>\d+)")
count = 0
# Strips the newline character
for line in Lines:
    count += 1
    m = vertex_regex.match(line.strip())
    vertex.append({'id': count,  'value': int(m.group('value')), 'weight': int(m.group('weight'))})



for v in vertex:
    print(v)


results = [[0 for i in range(num_vertex)] for w in range(knapsack_size)]
results_valid = [[0 for i in range(num_vertex)] for w in range(knapsack_size)]

#for j in reversed(range(0,knapsack_size)):
#    print(f"{j}",[f"{results[j][i]:.2f}" for i in range(num_vertex)])


for i in range(0,num_vertex):
    for x in range(0,knapsack_size):
        log.debug(f"i{i} x{x} w {vertex[i]['weight']}")

        if x >= 0 and i-1 >= 0:
            result1 = results[x][i-1] 
            if not results_valid[x][i-1]: print(f"Result x{x} i-1{i-1} not yet set") 
        else:
            result1 = 0
        if i > 0 and x >= vertex[i]['weight']:
            x_index = x-vertex[i]['weight'] 
            result2 = results[x_index][i-1]+vertex[i]['value']
            if not results_valid[x_index][i-1]: print(f"Result  x-weight(i) {x_index} {i-1} not yet set")
        else:
            result2 = vertex[i]['value']

        log.debug(f"{i} {x} R1 {result1}, R2 {result2}")
        results[x][i] = max(result1,result2)
        results_valid[x][i] = True 
    print(f"Result for i={i}")
    for j in reversed(range(0,knapsack_size)):
        loginfo = f"{j}",[f"{results[j][i]:.2f}" for i in range(num_vertex)]
        log.debug(loginfo)


print("Final")
#for j in reversed(range(0,knapsack_size)):
#    print(f"{j}",[f"{results[j][i]:.2f}" for i in range(num_vertex)])


print(f"Max Value is {results[knapsack_size-1][num_vertex-1]}")


