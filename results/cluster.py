# 测试单节点上的压缩并行效率

import pandas as pd
import matplotlib.pyplot as plt
import re
import math
import numpy as np
lName = ["SRR12175235 (5249MB)",
         "SRR027520 (3734MB)",
         "ERR7091256 (3143MB)",
         "SRR8386204 (2021MB)",
         "SRR8386224 (1836MB)",
         "SRR554369 (335MB)",]
colors = ['#F27970', '#BB9727', '#54B345', '#05B9E2', '#8983BF','#C76DA2','#F27970', '#BB9727', '#54B345', '#05B9E2', '#8983BF','#C76DA2'] #, ,
markers = ['H', 'D', 's', 'p', 'h', 'o', 'D', 's', 'p', 'h', 'H']
lines = ['-', '--', '-.', ':', "--", '-.', '--', '-.', ':', '-', "--"]


data = pd.read_csv("cluster.csv")
ct = data.loc[:,["ct1", "ct2", "ct3", "ct4", "ct5", "ct6", "ct7"]]
dt = data.loc[:,["dt1", "dt2", "dt3", "dt4", "dt5", "dt6", "dt7"]]

ct_speedup = ct.copy()
for i in range(6):
    index = "ct" + str(i + 2)
    ct_speedup[index] = ct_speedup["ct1"].div(ct_speedup[index])
ct_speedup["ct1"] = ct_speedup["ct1"].div(ct_speedup["ct1"])
print(ct_speedup)

dt_speedup = dt.copy()
for i in range(6):
    index = "dt" + str(i + 2)
    dt_speedup[index] = dt_speedup["dt1"].div(dt_speedup[index])
dt_speedup["dt1"] = dt_speedup["dt1"].div(dt_speedup["dt1"])
print(dt_speedup)

def pic(name, data):
    print("---------------------------------------------")
    print(name)
    plt.clf()
    for i in range(len(data)):
        x = [1, 2, 3, 4, 5, 6, 7]
        y = data.iloc[i].tolist()
        if name == "cluster_ct" or name == "cluster_dt":
            y = [math.log(j, 10) for j in y]
        print(x)
        print(y)
        plt.plot(x, y, c=colors[i], marker=markers[i], label=lName[i], linestyle=lines[i], linewidth=1.5, markersize=8)
        plt.xticks(np.arange(0, 8, 1))
        plt.gca().yaxis.set_major_formatter(plt.FormatStrFormatter('%.1f'))
        plt.legend()
        plt.savefig(name + ".svg")

pic("cluster_ct", ct)
pic("cluster_dt", dt)
pic("cluster_dt_speedup", dt_speedup)
pic("cluster_ct_speedup", ct_speedup)