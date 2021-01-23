readfile = open("avt.txt", "r")
newfile = open("newavg.txt", "w")

for number, line in enumerate(readfile.readlines()):
    print(line)
    line_value = line.split(" ")[0]
    newfile.write("{0} {1}".format(number, line_value))

readfile.close()
newfile.close()