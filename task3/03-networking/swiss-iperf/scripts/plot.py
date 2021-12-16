import matplotlib.pyplot as plt
import sys
from datetime import datetime

def gen_plot(timestr, xs, y_basic, y_zerocopy, y_swiss_basic, y_swiss_zerocopy):
    fig, (ax1, ax2) = plt.subplots(1, 2, constrained_layout=True, sharey=True)

    ax1.plot(xs, y_basic, "o-", label="basic")
    ax1.plot(xs, y_zerocopy, "s-", label="zerocopy")
    ax1.set_xlabel("Window size (in KB)")
    ax1.set_ylabel("Throughput (in Gbits/sec)")
    ax1.set_title("iperf3")
    ax1.legend()

    ax2.plot(xs, y_swiss_basic, "o-", label="basic")
    ax2.plot(xs, y_swiss_zerocopy, "s-", label="zerocopy")
    ax2.set_xlabel("Window size (in KB)")
    ax2.set_title("swiss-iperf")#
    ax2.legend()

    filename = "/data/plot_{}.png".format(timestr)
    print("generated plot:", filename)
    fig.savefig(filename)
    fig.savefig("/data/plot_latest.png")


if __name__ == "__main__":
    dto = datetime.now()
    timestr = dto.strftime("%Y-%m-%d-%H-%M-%S")

    xs = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_basic = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_zerocopy = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_swiss_basic = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_swiss_zerocopy = list(map(float, sys.stdin.readline().strip().split(" ")))

    # print(xs, y_basic, y_swiss_zerocopy)

    gen_plot(timestr, xs, y_basic, y_zerocopy, y_swiss_basic, y_swiss_zerocopy)

