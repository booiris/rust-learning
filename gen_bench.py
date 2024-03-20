import random

with open("bench_in", "w") as f:
    f.write("1000\n")
    for i in range(1000):
        f.write("5000\n")
        for _ in range(5000):
            rand_data = random.randint(1, 10000)
            f.write(str(rand_data) + " ")
        f.write("\n")
