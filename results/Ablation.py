# 获得消融实验结果

import pandas as pd
import re
pathName = "/public/home/jd_sunhui/genCompressor/PQSDC/data/TestAblation/ablation/"
sFiles = [
    "SRR8386204", "SRR8386224", "SRR8386225",
    "ERR7091256_1", "ERR7091268_1", "SRR013951",
    "SRR027520", "SRR554369", "SRR17794741", "SRR17794724", "SRR12175235"
]

AlgorithmList = ["Fname", "Fsize(MB)", "Fwight", "wm-cr", "wm-st", "wm-mm", "wp-cr", "wp-st", "wp-mm", "pm-cr", "pm-st", "pm-mm"]

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
# 计算每个文件的权重
for i in range(len(sFiles)):
    weight.append(fileSize[i] / sum(fileSize))
for i in range(len(dataList)):
    temp = []
    temp.append(sFiles[i])
    temp.append(round(float(fileSize[i]/1024/1024), 3))
    temp.append(round(weight[i], 3))
    for j in range(len(dataList[i])):
        data = dataList[i].iloc[j]
        temp.append(float(data["bit/base"]))
        A = float(data["CTime(S)"])
        B = float(data["DTime(S)"])
        C = A + B
        temp.append(C)
        A = float(data["Cmem(KB)"])
        B = float(data["Cmem(KB)"])
        C = max(A, B)
        temp.append(C)
        #temp.append(float(data["CTime(S)"]))
        #temp.append(float(data["DTime(S)"]))
        #temp.append(float(data["Cmem(KB)"]))
        #temp.append(float(data["Dmem(KB)"]))
    result.append(temp)
result = pd.DataFrame(result)
result.columns = AlgorithmList
result[["wm-mm", "wp-mm", "pm-mm"]] = result[["wm-mm", "wp-mm", "pm-mm"]].div(1024) # 转换为MB
#result[["wm-st", "wp-st", "pm-st"]] = result[["wm-st", "wp-st", "pm-st"]].div(60) # 转换为分钟
#print(result)

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
for i in range(3):
    avg_up.append(round(float(100 * (avg[i+3] - baseline_avg) / avg[i+3]), 3))
    wavg_up.append(round(float(100 *(wavg[i + 3] - baseline_wavg) / wavg[i + 3]), 3))
result.loc[len(result)] = avg
#result.loc[len(result)] = avg_up
result.loc[len(result)] = wavg
#result.loc[len(result)] = wavg_up
result = result.round(3)
print(result)

result.to_csv("Ablation.csv")