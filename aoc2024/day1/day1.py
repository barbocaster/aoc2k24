file = open("sample.txt")

left = []
right = []

for lines in file:
    splitted = lines.split()
    left.append(splitted[0])
    right.append(splitted[1])

left.sort()
right.sort()

sum = 0

for idx in range(0, len(left)):
    sum += abs(int(left[idx]) - int(right[idx]))


print(sum)
