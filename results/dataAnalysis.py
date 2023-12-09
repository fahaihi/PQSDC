print("hello")
import pandas as pd
import os
import glob


csvFileList=[]
AlgorithmList = ["PQSDC", "ZPAQ", "QVZ", "AQUA", "FCLQC", "LCQS", "7-Zip", "PIGZ", "PBzip2", "CMIC", "Qscomp"]
TableHeadList = ["CTime(S)",	"Cmem(KB)",	"CFsize(B)"	,"bit/base",	"ratio",	"DTime(S)",	"Dmem(KB"]
# 获取当前路径
current_dir = os.getcwd()

# 查找当前路径下所有的 CSV 文件
csv_files = glob.glob(current_dir + "/*.csv")

# 打印所有找到的 CSV 文件路径
dataList = []
for file_path in csv_files:
    print(file_path)
    file_name = os.path.basename(file_path)
    csvFileList.append(file_name)
    data = pd.read_csv(file_path)
    dataList.append(data)

def getInfo(csv_files, dataList, type_info):
    result = []
    for i in range(len(dataList)):
        temp_result = []
        #temp_result.append(csv_files[i])
        for j in range(len(dataList[i])):
            data = dataList[i].iloc[j]
            temp_result.append(float(data[type_info]))
        result.append(temp_result)
    result = pd.DataFrame(result)
    result.columns = AlgorithmList
    return result

for info in ["CTime(S)",	"Cmem(KB)",	"bit/base",	"DTime(S)",	"Dmem(KB"]:
    print("*"*50)
    print(info)
    result = getInfo(csv_files, dataList, info)
    print(result)
