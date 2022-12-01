# https://adventofcode.com/2022/day/1
# 1. Find the largest sum of numbers
# 2. Find the sum of the three largest sum of numbers

lines = open('data.txt').readlines()
counts = []
count = 0
for line in lines:
    if not line.strip():
        counts.append(count)
        count = 0
    else:
        count += int(line)

ordered_counts = sorted(counts, reverse=True)
# solution for #1
print(ordered_counts[0])

# solution for #2
print(sum(ordered_counts[:3]))