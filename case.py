x = 200000
with open("./ini", "w") as f:
    f.write("1\n")
    f.write(f"{x}\n")
    for i in range(x):
        f.write(f"{i+2} ")
    f.write("\n")
