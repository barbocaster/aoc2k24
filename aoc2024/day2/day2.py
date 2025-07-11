safes = 0
unsafes = 0
file = open("sample2.txt")


for lines in file:
    splitted = lines.split()

    order = None

    if int(splitted[0]) > int(splitted[1]):
        order = -1
    else:
        order = 1

    
    for idx in range(0, len(splitted) - 1):
        left = int(splitted[idx])
        right = int(splitted[idx+1])

        level_decreasing = abs(int(splitted[idx]) - int(splitted[idx+1]))

        # ugly solve
        if (left > right and order != None and order == 1) or (left < right and order != None and order == -1) or left == right:
            unsafes+=1
            break;
    
        if level_decreasing > 3:
            print(level_decreasing)
            unsafes+=1
            break

    safes+=1

print(safes-unsafes)



        


        
