import matplotlib.pyplot as plt

ys = list(map(float, open("bench.out", "r").read().strip().split("\n")))
xs = [10, 100, 1000, 10000]
plt.xlabel("Concurrent connections")
plt.ylabel("Requests/Second")

plt.plot(xs,ys)

plt.savefig("plot.png")

plt.show()



