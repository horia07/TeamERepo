import matplotlib.pyplot as plt
import sys
from datetime import datetime

def gen_plot(timestr, xs, y_basic, y_zerocopy, y_swiss_basic, y_swiss_zerocopy, xs_mss, y_mss, y_swiss_mss):
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

    filename = "/data/plot_window_{}.png".format(timestr)
    print("generated plot:", filename)
    fig.savefig(filename)
    fig.savefig("/data/plot_window_latest.png")

    plt.clf()

    plt.plot(xs_mss, y_mss, "o-", label="iperf3")
    plt.plot(xs_mss, y_swiss_mss, "s-", label="swiss-iperf")
    plt.xlabel("MSS (in Bytes)")
    plt.ylabel("Throughput (in Gbits/sec)")

    filename = "/data/plot_mss_{}.png".format(timestr)
    print("generated plot:", filename)
    plt.savefig(filename)
    plt.savefig("/data/plot_mss_latest.png")

if __name__ == "__main__":
    dto = datetime.now()
    timestr = dto.strftime("%Y-%m-%d-%H-%M-%S")

    xs = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_basic = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_zerocopy = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_swiss_basic = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_swiss_zerocopy = list(map(float, sys.stdin.readline().strip().split(" ")))
    xs_mss = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_mss = list(map(float, sys.stdin.readline().strip().split(" ")))
    y_swiss_mss = list(map(float, sys.stdin.readline().strip().split(" ")))

    # print(xs, y_basic, y_swiss_zerocopy)

    gen_plot(timestr, xs, y_basic, y_zerocopy, y_swiss_basic, y_swiss_zerocopy, xs_mss, y_mss, y_swiss_mss)

