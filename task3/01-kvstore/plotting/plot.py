import numpy as np
import matplotlib.pyplot as plt
from datetime import datetime


def Extract(lst):
        return [ int(float(item[2])) for item in lst]

output1 = open("/data/output1.txt")
lines = [x.split(",") for x in output1.readlines()]
values1 = Extract(lines)

output2 = open("/data/output2.txt")
lines = [x.split(",") for x in output2.readlines()]
values2 = Extract(lines)

output3 = open("/data/output3.txt")
lines = [x.split(",") for x in output3.readlines()]
values3 = Extract(lines)

output4 = open("/data/output4.txt")
lines = [x.split(",") for x in output4.readlines()]
values4 = Extract(lines)

output5 = open("/data/output5.txt")
lines = [x.split(",") for x in output5.readlines()]
values5 = Extract(lines)

output6 = open("/data/output6.txt")
lines = [x.split(",") for x in output6.readlines()]
values6 = Extract(lines)

output7 = open("/data/output7.txt")
lines = [x.split(",") for x in output7.readlines()]
values7 = Extract(lines)

output8 = open("/data/output8.txt")
lines = [x.split(",") for x in output8.readlines()]
values8 = Extract(lines)

output9 = open("/data/output9.txt")
lines = [x.split(",") for x in output9.readlines()]
values9 = Extract(lines)

output10 = open("/data/output10.txt")
lines = [x.split(",") for x in output10.readlines()]
values10 = Extract(lines)

#
#plotread
#


x = [values1[1],values2[1],values3[1],values4[1],values5[1]]  
y = [values1[12],values2[12],values3[12],values4[12],values5[12]]
z = [values6[1],values7[1],values8[1],values9[1],values10[1]]
k = [values6[12],values7[12],values8[12],values9[12],values10[12]]  
plt.plot(x, y) 
plt.plot(z, k)
plt.xlabel('Throughput (ops/sec)') 
plt.ylabel('Average read latency')  
plt.title('Read latency') 
plt.legend(["redis", "rocksdb"])

dto = datetime.now()
timestr = dto.strftime("%Y%m%d%H%M%S")

plt.savefig("/data/plotread.png".format(timestr))

#
#plotupdate
#

x = [values1[1],values2[1],values3[1],values4[1],values5[1]]
y = [values1[25],values2[25],values3[25],values4[25],values5[25]]
z = [values6[1],values7[1],values8[1],values9[1],values10[1]]
k = [values6[25],values7[25],values8[25],values9[25],values10[25]]  

plt.plot(x, y) 
plt.plot(z, k)
plt.xlabel('Throughput (ops/sec)') 
plt.ylabel('Average update latency')  
plt.title('Update latency') 
plt.legend(["redis", "rocksdb"])

dto = datetime.now()
timestr = dto.strftime("%Y%m%d%H%M%S")

plt.savefig("/data/plotupdate.png".format(timestr))



