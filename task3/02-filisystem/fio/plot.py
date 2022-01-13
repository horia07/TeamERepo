import numpy as np
import matplotlib.pyplot as plt
from datetime import datetime


def Extract(lst):
	return [int(int(item[1])/(1000)) for item in lst]

xs = [4, 8, 16, 32]

ext4r = open("/data/ext4rand_read_bw.log")
lines = [x.split(",") for x in ext4r.readlines()]
valuesextr = Extract(lines)

ext4w = open("/data/ext4rand_write_bw.log")
lines = [x.split(",") for x in ext4w.readlines()]
valuesextw = Extract(lines)

btrfsw = open("/data/btrfsrand_write_bw.log")
lines = [x.split(",") for x in btrfsw.readlines()]
valuesbtrfsw = Extract(lines)

btrfsr = open("/data/btrfsrand_read_bw.log")
lines = [x.split(",") for x in btrfsr.readlines()]
valuesbtrfsr = Extract(lines)


#_______________________________________________________________________
#plotread

labels = ['4', '8', '16', '32']
x = np.arange(len(labels))
width = 0.35

fig, ax = plt.subplots()

rects1 = ax.bar(x - width/2, valuesextr, width, label='extr4', color='r')
rects2 = ax.bar(x + width/2, valuesbtrfsr, width, label='btrFS', color='b')

ax.set_ylabel('MB/s')
ax.set_xlabel('IO depth')
ax.set_title('ext4 vs btrFS read benchmark')
ax.set_xticks(x, labels)


ax.bar_label(rects1, padding=3)
ax.bar_label(rects2, padding=3)
fig.tight_layout()

ax.legend()


dto = datetime.now()
timestr = dto.strftime("%Y%m%d%H%M%S")

plt.savefig("/data/plotread.png".format(timestr))

#____________________________________________________________________________________
#plotwrite

labels = ['4', '8', '16', '32']
x = np.arange(len(labels))
width = 0.35

fig, ax = plt.subplots()

rects1 = ax.bar(x - width/2, valuesextw, width, label='extr4', color='r')
rects2 = ax.bar(x + width/2, valuesbtrfsw, width, label='btrFS', color='b')

ax.set_ylabel('MB/s')
ax.set_xlabel('Block Size')
ax.set_title('ext4 vs btrFS write benchmark')
ax.set_xticks(x, labels)


ax.bar_label(rects1, padding=3)
ax.bar_label(rects2, padding=3)
fig.tight_layout()

ax.legend()


dto = datetime.now()
timestr = dto.strftime("%Y%m%d%H%M%S")

plt.savefig("/data/plotwrite.png".format(timestr))


