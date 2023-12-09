#!/bin/bash
# /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1.sh [mode] [fileName] [threads]
echo "pqsdc algorithm"
mode=$1
fileName=$2
threads=$3

echo $mode
echo $fileName
echo $threads

if [ "${mode}" = "c" ]; then
  echo "compression mode"

  # 1 序列分区 生成${fileName}.partition文件
  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -c ${threads} ${fileName}
  # 2 进行分区文件进行游程预测映射
  pwdPath=$(pwd)
  cd ${fileName}.partition
  for ((i = 1; i < 3; i++)); do
    {
      /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} data_${i}.dat
    } &
  done
  wait

  # 3 使用ZPAQ算法进行级联压缩
  for ((i = 0; i < 3; i++)); do
    {
      if (( i == 0 )); then
        zpaq a partition_dat.zpaq partition_dat -method 5 -threads ${threads}
      fi
      if (( i != 0 )); then
        zpaq a data_${i}.dat.PQVRC.zpaq data_${i}.dat.PQVRC -method 5 -threads ${threads}
      fi
    } &
  done
  wait

  # 4 打包为压缩文件
  tar -cf result.pqsdc_v1 data_1.dat.PQVRC.zpaq data_2.dat.PQVRC.zpaq partition_dat.zpaq
  ls -l --block-size=1 result.pqsdc_v1
  # 5 删除所有文件
  #mv ${fileName%%.qualities}.pqsrc_v1 ${pwdPath}
  rm -rf *dat*
  cd ${pwdPath}
  #rm -rf ${fileName}.partition
fi

if [ "${mode}" = "d" ]; then # 输入文件夹
  echo "de-compression mode"
  # 1 使用tar解包文件
  pwdPath=$(pwd)
  cd ${fileName}
  tar -xvf result.pqsdc_v1

  # 2 使用zpaq算法解压缩文件
  for ((i = 0; i < 3; i++)); do
    {
      if (( i == 0 )); then
        zpaq x partition_dat.zpaq -method 5 -threads ${threads}
      fi
      if (( i != 0 )); then
        zpaq x data_${i}.dat.PQVRC.zpaq -method 5 -threads ${threads}
      fi
    } &
  done
  wait

  # 3 进行分区文件进行游程预测映射
  for ((i = 1; i < 3; i++)); do
    {
      /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -d ${threads} data_${i}.dat.PQVRC
    } &
  done
  wait
  # 4 合并分区恢复原始文件
  cd ..
  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -d ${threads} ${fileName}
  rm -rf ${fileName}
  cd ${pwdPath}

fi
