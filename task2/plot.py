import matplotlib.pyplot as plt
from datetime import datetime

xs = open("/data/connections.txt", "r").read().strip().split("\n")
xs = [int(x) for x in xs]

basic = open("/data/data-18080.txt", "r").read().strip().split("\n")
basic = [float(x) for x in basic]

epoll = open("/data/data-18081.txt", "r").read().strip().split("\n")
epoll = [float(x) for x in epoll]

plt.plot(xs, basic)
plt.plot(xs, epoll)
plt.ylabel("Requests/Second")
plt.xlabel("Connections")

plt.legend(["basic", "epoll"])

dto = datetime.now()
timestr = dto.strftime("%Y%m%d%H%M%S")

plt.savefig("/data/plot-{}.png".format(timestr))

plt.show()

