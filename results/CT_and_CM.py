# 统计算法的压缩内存和时间开销，并绘制散点图
from sklearn.preprocessing import MinMaxScaler
import pandas as pd
import re
import pandas as pd
import matplotlib.pyplot as plt
# 1 导入路径、文件等相关信息
pathName = "/public/home/jd_sunhui/genCompressor/PQSDC/data/TestSame/result/"
sFiles = [
    "SRR8386204", "SRR8386224", "SRR8386225",
    "ERR7091256_1", "ERR7091268_1", "SRR013951",
    "SRR027520", "SRR554369", "SRR17794741", "SRR17794724", "SRR12175235"
]
SelectedAlgorithm = ["PQSDC", "ZPAQ", "CMIC", "LCQS", "Qscomp"]
AlgorithmList = ["Fname", "Fsize(MB)", "PQSDC", "7-Zip", "PIGZ", "PBzip2", "ZPAQ", "CMIC", "LCQS", "AQUa", "FCLQC", "Qscomp", "QVZ"]
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

result_Mem = []
result_Time = []
result_Size = []
#print("--------------", len(dataList))
#print(dataList)
for i in range(len(dataList)):
    temp_Mem = []
    temp_Time = []
    temp_Size = []
    temp_Mem.append(sFiles[i])
    temp_Mem.append(round(float(fileSize[i]/1024/1024), 3))
    temp_Time.append(sFiles[i])
    temp_Time.append(round(float(fileSize[i]/1024/1024), 3))
    temp_Size.append(sFiles[i])
    temp_Size.append(round(float(fileSize[i] / 1024 / 1024), 3))
    for j in range(len(dataList[i])):
        data = dataList[i].iloc[j]
        temp_Mem.append(float(data["Cmem(KB)"] / 1024 ))
        temp_Time.append(float(data["CTime(S)"]))
        temp_Size.append(float(data["ratio"]))
    result_Mem.append(temp_Mem)
    result_Time.append(temp_Time)
    result_Size.append(temp_Size)


result_Time = pd.DataFrame(result_Time)
result_Mem = pd.DataFrame(result_Mem)
result_Size = pd.DataFrame(result_Size)

result_Time.columns = AlgorithmList
result_Mem.columns = AlgorithmList
result_Size.columns = AlgorithmList

# 获取文件大小的列表
FS = result_Mem.loc[:, "Fsize(MB)"]

result_Time = result_Time.loc[:, SelectedAlgorithm]
#result_Time = result_Time.append(result_Time.mean(), ignore_index=True)
result_Mem = result_Mem.loc[:, SelectedAlgorithm]
result_Size = result_Size.loc[:, SelectedAlgorithm]
#result_Mem = result_Mem.append(result_Mem.mean(), ignore_index=True)
result_Time_normalized = result_Time.apply(lambda x: (x - x.min()) / (x.max() - x.min()), axis=1)

# 对 result_Mem 进行最大最小归一化处理
result_Mem_normalized = result_Mem.apply(lambda x: (x - x.min()) / (x.max() - x.min()), axis=1)
result_Size_normalized = result_Size.apply(lambda x: (x - x.min()) / (x.max() - x.min()), axis=1)

#  绘制散点图
#data = pd.concat([result_Time_normalized, result_Mem_normalized], axis=1)

colors = ['#F27970', '#BB9727', '#54B345', '#05B9E2', '#8983BF'] #, '#C76DA2'
markers = ['o', 'o', 'o', 'o', 'o']
for i in range(len(SelectedAlgorithm)):
    # 外层i循环算法
    x = result_Mem_normalized.loc[:, SelectedAlgorithm[i]].tolist()
    y = result_Time_normalized.loc[:, SelectedAlgorithm[i]].tolist()
    z = result_Size.loc[:, SelectedAlgorithm[i]].tolist()
    #FS = FS.tolist()
    print("*" * 20)
    print(SelectedAlgorithm[i])
    print(x)
    print(y)
    print(y)
    print(FS)
    plt.scatter(x, y, c=colors[i], marker=markers[i], label=SelectedAlgorithm[i], s=150, alpha=0.7)

# 添加平均值
x = result_Mem_normalized.mean().tolist()
y = result_Time_normalized.mean().tolist()
z = result_Size_normalized.mean().tolist()
z = [x*1500 + 200 for x in z]
plt.scatter(x, y, c=colors, marker='o', s=z, alpha=0.95, edgecolor='black', linestyle='dashed')
for i, txt in enumerate(zip(x, y)):
    plt.annotate(f'({txt[0]:.2f}, {txt[1]:.2f})', (x[i], y[i]),
                textcoords='offset points', xytext=(0,25), ha='center')
print(x)
print(y)
print(z)
#plt.xlabel("normalized compression peak-memory")
#plt.ylabel("normalized compression time")
#plt.title("compression scatter plot")
#plt.legend()
#plt.show()
plt.savefig("CT_and_CM_scatter_plot.svg")

#print(result_Size_normalized)


