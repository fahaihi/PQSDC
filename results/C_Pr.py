# 测试单节点上的压缩并行效率

import pandas as pd
import matplotlib.pyplot as plt
import re
import math
import numpy as np
pathName = "/public/home/jd_sunhui/genCompressor/PQSDC/data/TestPara/parallel/"
sFiles = ["SRR12175235", "SRR027520", "ERR7091256_1", "SRR8386204", "SRR8386224","SRR554369",]
lName = ["SRR12175235 (5249MB)",
         "SRR027520 (3734MB)",
         "ERR7091256 (3143MB)",
         "SRR8386204 (2021MB)",
         "SRR8386224 (1836MB)",
         "SRR554369 (335MB)",]

AlgorithmList = ["Fname", "Fsize(MB)", "Fwight", "Pr=1", "Pr=2", "Pr=4", "Pr=6", "Pr=8",
                 "Pr=10", "Pr=12", "Pr=14", "Pr=16", "Pr=18", "Pr=20", "Pr=22", "Pr=24", "Pr=26", "Pr=28"]

dataList = []
baseList = []
fileSize = []
weight = []
for name in sFiles:
    da = pd.read_csv(pathName + name + ".sum_result.csv")
    #da = da.loc[:,["bit/base"]]
    dataList.append(da)
    with open(pathName + name + ".sum_result") as f:
        lines = f.readlines()
        lineBase = lines[1].strip()
        lineFSize = lines[4].strip()
        print(lineBase)
        baseList.append(float(re.findall(r'\d+', lineBase)[0]))
        print(lineFSize)
        fileSize.append(float(re.findall(r'\d+', lineFSize)[0]))
        #print(lines)

result = []
result_s = []
# 计算每个文件的权重
for i in range(len(sFiles)):
    weight.append(fileSize[i] / sum(fileSize))
for i in range(len(dataList)):
    #print("*"*50)
    #print(dataList[i])
    temp = []
    temp.append(sFiles[i])
    temp.append(round(float(fileSize[i]/1024/1024), 3))
    temp.append(round(weight[i], 3))
    for j in range(len(dataList[i])):
        data = dataList[i].iloc[j]
        temp.append(float(data["CTime(S)"]))
    result.append(temp)
result = pd.DataFrame(result)
result.columns = AlgorithmList
print(result)
def getAvgeMean(result):
    temp = []
    temp.append("avg-mean")
    for name in AlgorithmList:
        print(name)
        sum = 0
        if name == "Fname":
            continue
        else:
            dat = result[name]
            for i in range(len(dat)):
                sum = sum + dat.iloc[i]
        #print(sum/len(sFiles))
        temp.append(round(sum/len(sFiles),3))
    #result.loc[len(result)] = temp
    return temp

avg = getAvgeMean(result)

def getWeightAvg(result):
    # 1 计算总文件大小
    sumSize = 0
    for i in fileSize:
        sumSize = sumSize + i;
    # 2 计算加群平均
    temp = []
    temp.append("weight-mean")
    for name in AlgorithmList:
        sum = 0
        if name == "Fname":
            continue
        else:
            dat = result[name]
            for i in range(len(dat)):
                sum = sum + dat.iloc[i] * weight[i]
        # print(sum/len(sFiles))
        temp.append(round(sum, 3))
    #result.loc[len(result)] = temp
    return temp

wavg = getWeightAvg(result)
# 计算提升百分比


avg_up=["avg_up(%)", round(float(0), 3), round(float(0), 3)]
wavg_up=["wavg_up(%)", round(float(0), 3), round(float(0), 3)]
baseline_avg=avg[3]
baseline_wavg=wavg[3]
for i in range(11):
    avg_up.append(round(float(100 * (avg[i+3] - baseline_avg) / avg[i+3]), 3))
    wavg_up.append(round(float(100 *(wavg[i + 3] - baseline_wavg) / wavg[i + 3]), 3))
#result.loc[len(result)] = avg
#result.loc[len(result)] = avg_up
#result.loc[len(result)] = wavg
#result.loc[len(result)] = wavg_up
print(result)

result.to_csv("C_Pr.csv")


# 绘制压缩时间图
result = result.loc[:, ["Pr=1", "Pr=2", "Pr=4", "Pr=6", "Pr=8", "Pr=10", "Pr=12", "Pr=14", "Pr=16", "Pr=18", "Pr=20", "Pr=22", "Pr=24", "Pr=26", "Pr=28"]]
print(result)

colors = ['#F27970', '#BB9727', '#54B345', '#05B9E2', '#8983BF','#C76DA2','#F27970', '#BB9727', '#54B345', '#05B9E2', '#8983BF','#C76DA2'] #, ,
markers = ['H', 'D', 's', 'p', 'h', 'o', 'D', 's', 'p', 'h', 'H']
lines = ['-', '--', '-.', ':', "--", '-.', '--', '-.', ':', '-', "--"]

for i in range(len(result)):
    x = [1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]
    y = result.iloc[i].tolist()
    y = [math.log(j, 10) for j in y]
    print(x)
    print(y)
    plt.plot(x, y, c=colors[i], marker=markers[i], label=lName[i], linestyle=lines[i], linewidth=1.5, markersize=8)
#plt.xlabel("cpu cores number")
#plt.ylabel("time consumption / log(y,10)")
#plt.title("compression scatter plot")
plt.xticks(np.arange(0, 30, 2))
plt.legend()
#plt.show()
plt.savefig("CTime_Pr.svg")

# 绘制加速比曲线图
plt.clf()
for i in range(len(result)):
    x = [1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28]
    y = result.iloc[i].tolist()
    chuanxing = y[0]
    y = [chuanxing/j for j in y]
    print(x)
    print(y)
    plt.plot(x, y, c=colors[i], marker=markers[i], label=lName[i], linestyle=lines[i], linewidth=1.5, markersize=8)
#plt.xlabel("cpu cores number")
#plt.ylabel("speedup")
#plt.title("compression scatter plot")
plt.xticks(np.arange(0, 30, 2))
plt.legend()
#plt.show()
plt.savefig("CTime_Speedup_Pr.svg")
