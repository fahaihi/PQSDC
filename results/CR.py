import pandas as pd
import re
pathName = "/public/home/jd_sunhui/genCompressor/PQSDC/data/TestSame/result/"
sFiles = [
    "SRR8386204", "SRR8386224", "SRR8386225",
    "ERR7091256_1", "ERR7091268_1", "SRR013951",
    "SRR027520", "SRR554369", "SRR17794741", "SRR17794724", "SRR12175235"
]

AlgorithmList = ["Fname", "Fsize(MB)", "Fwight", "PQSDC", "7-Zip", "PIGZ", "PBzip2", "ZPAQ", "CMIC", "LCQS", "AQUa", "FCLQC", "Qscomp", "QVZ"]

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
    result.append(temp)
result = pd.DataFrame(result)
result.columns = AlgorithmList
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
for i in range(11):
    avg_up.append(round(float(100 * (avg[i+3] - baseline_avg) / avg[i+3]), 3))
    wavg_up.append(round(float(100 *(wavg[i + 3] - baseline_wavg) / wavg[i + 3]), 3))
result.loc[len(result)] = avg
result.loc[len(result)] = avg_up
result.loc[len(result)] = wavg
result.loc[len(result)] = wavg_up
print(result)

result.to_csv("CR.csv")