import subprocess
import json
import argparse
import os
from datetime import datetime

mss_values = [128, 256, 512, 536, 1024, 2048, 4096, 8192, 9216]

def run_swiss_client(host, time, interface=None, window_size=None, zerocopy=False, mss=None):

    client_cmd = f"bin/swiss-iperf client {host} --time {time} --json"

    if window_size:
        client_cmd += f" -w {window_size}"

    if zerocopy:
        client_cmd += " -Z"

    if mss:
        client_cmd += f" -M {mss}"

    if interface:
        client_cmd += f" --interface {interface}"


    print("cmd=", client_cmd)
    client = subprocess.Popen(client_cmd.split(" "), stdout=subprocess.PIPE)

    client_res, _ = client.communicate()
    client_res = json.loads(client_res)

    bytes_written = client_res["bytes_written"] 
    bits = bytes_written * 8
    bps = bits / client_res["time"] / 1000000000
    print(f"Gbits_per_second= {bps}")
    print()
    client.kill()

    return bps

def run_iperf_client(host, time, interface=None, window_size=None, zerocopy=False, mss=None):
    if interface:
        host = f"{host}%{interface}"

    client_cmd = f"iperf3 -c {host} --time {time} --json"

    if window_size:
        client_cmd += f" -w {window_size}"

    if zerocopy:
        client_cmd += " -Z"

    if mss:
        client_cmd += f" -M {mss}"


    print("cmd=", client_cmd)
    client = subprocess.Popen(client_cmd.split(" "), stdout=subprocess.PIPE)

    client_res, _ = client.communicate()
    client_res = json.loads(client_res)

    bps = client_res["end"]["sum_sent"]["bits_per_second"] / 1000000000
    print(f"Gbits_per_second= {bps}")
    print()
    client.kill()

    return bps

def main():

    parser = argparse.ArgumentParser(description="iperf3 benchmark")
    parser.add_argument("--host", type=str, default="::1")
    parser.add_argument("--interface", type=str)
    parser.add_argument("--time", type=int, default=10)

    args = parser.parse_args()

    host = args.host
    time = args.time
    interface = args.interface

    os.makedirs("result", exist_ok=True)

    xs = [] 
    xs_mss = []
    y_basic, y_zerocopy, y_mss = [], [], []
    y_swiss_basic, y_swiss_zerocopy, y_swiss_mss = [], [], []

    for i in range(9):
        x = 2**i 
        window_size = x * 1024
        mss = mss_values[i]

        bps_iperf_basic = run_iperf_client(host, time, interface=interface, window_size=window_size)
        bps_iperf_zerocopy = run_iperf_client(host, time, interface=interface, window_size=window_size, zerocopy=True)

        bps_swiss_basic = run_swiss_client(host, time, interface=interface, window_size=window_size)
        bps_swiss_zerocopy = run_swiss_client(host, time, interface=interface, window_size=window_size, zerocopy=True)

        bps_iperf_mss = run_iperf_client(host, time, interface=interface, mss=mss)
        bps_swiss_mss = run_swiss_client(host, time, interface=interface, mss=mss)

        xs.append(x)
        xs_mss.append(mss)

        y_basic.append(bps_iperf_basic)
        y_zerocopy.append(bps_iperf_zerocopy)

        y_swiss_basic.append(bps_swiss_basic)
        y_swiss_zerocopy.append(bps_swiss_zerocopy)

        y_mss.append(bps_iperf_mss)
        y_swiss_mss.append(bps_swiss_mss)

    dto = datetime.now()
    timestr = dto.strftime("%Y-%m-%d-%H-%M-%S")

    for filename in ["data_latest", f"data_{timestr}"]:

        filename = f"result/{filename}.txt"
        print("generated data:", filename)
        with open(filename, "w") as f:
            f.write(" ".join(map(str, xs)) + "\n")
            f.write(" ".join(map(str, y_basic)) + "\n")
            f.write(" ".join(map(str, y_zerocopy)) + "\n")
            f.write(" ".join(map(str, y_swiss_basic)) + "\n")
            f.write(" ".join(map(str, y_swiss_zerocopy)) + "\n")
            f.write(" ".join(map(str, xs_mss)) + "\n")
            f.write(" ".join(map(str, y_mss)) + "\n")
            f.write(" ".join(map(str, y_swiss_mss)) + "\n")

    # gen_plot(timestr, xs, y_basic, y_zerocopy, y_swiss_basic, y_swiss_zerocopy)


if __name__ == "__main__":
    main()
