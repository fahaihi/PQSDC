#!/bin/bash
# /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_without_partition.sh [mode] [fileName] [threads]
echo "pqsdc algorithm without partition"
mode=$1
fileName=$2
threads=$3

echo $mode
echo $fileName
echo $threads

if [ "${mode}" = "c" ]; then
  echo "compression mode"

  echo "1 进行分区文件进行游程预测映射"
  pwdPath=$(pwd)

  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} ${fileName}

  echo "2 使用ZPAQ算法进行级联压缩"
  zpaq a ${fileName}.PQVRC.zpaq ${fileName}.PQVRC -method 5 -threads ${threads}

  echo "3 计算压缩后文件大小"
  ls -l --block-size=1 ${fileName}.PQVRC.zpaq

  cd ${pwdPath}
  #rm -rf ${fileName}.partition
fi

if [ "${mode}" = "d" ]; then # 输入文件夹
  echo "de-compression mode"

  pwdPath=$(pwd)



  echo " 1 使用zpaq算法解压缩文件"
  aa=${fileName%.zpaq}
  rm -rf $aa
  echo "===> $aa"
  zpaq x ${fileName} -method 5 -threads ${threads}

  echo " 2 进行分区文件进行游程预测映射恢复"
  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -d ${threads} ${aa}


fi
