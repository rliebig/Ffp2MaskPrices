readfile = open("avg.txt", "r")
newfile = open("hourly.txt", "w")


value_dict = {}
amount_dict = {}
for i in range(24):
    value_dict[i] = 0.0
    amount_dict[i] = 0.0


for line in readfile.readlines():
    hour = line.split("T")[1].split(":")[0]
    value = line.split(" ")[1]

    value_dict[int(hour)] += float(value)
    amount_dict[int(hour)] += 1.0

for j in value_dict:
    value_dict[j] = value_dict[j] / amount_dict[j]

for j in value_dict:
    print(str(j) + " " + str(value_dict[j]))
    newfile.write(str(j) + " " + str(value_dict[j]) + "\n")

readfile.close()
newfile.close()