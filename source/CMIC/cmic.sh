#!/bin/bash
# /public/home/jd_sunhui/genCompressor/PQSDC/source/CMIC/cmic.sh [mode] [fileName] [threads]
echo "cmic algorithm"
mode=$1
fileName=$2
threads=$3

echo $mode
echo $fileName
echo $threads

if [ "${mode}" = "c" ]; then
  echo "compression mode"
  # 使用CMIC进行映射打包
  /public/home/jd_sunhui/genCompressor/PQSDC/source/CMIC/CMIC -c ${fileName}
  # 使用ZPAQ算法进行压缩
  zpaq a ${fileName}.CMIC.zpaq ${fileName}.CMIC -method 5 -threads ${threads}
  # 删除文件
  rm -rf ${fileName}.CMIC
fi

if [ "${mode}" = "d" ]; then
  echo "de-compression mode"
  # 使用ZPAQ解包文件
  zpaq x ${fileName} -method 5 -threads ${threads}
  # 使用CMIC算法进行解压缩
  result=$(echo "$fileName" | sed 's/\.zpaq$//')
  echo $result
  /public/home/jd_sunhui/genCompressor/PQSDC/source/CMIC/CMIC -d ${result}
  rm -rf ${result}
fi
