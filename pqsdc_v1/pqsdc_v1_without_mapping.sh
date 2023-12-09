#!/bin/bash
# /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_without_mapping.sh [mode] [fileName] [threads]
echo "pqsdc algorithm for para (no shell wait)"
mode=$1
fileName=$2
threads=$3

echo $mode
echo $fileName
echo $threads

if [ "${mode}" = "c" ]; then
  echo "compression mode"

  echo "1 序列分区 生成${fileName}.partition文件"
  echo "/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -c ${threads} ${fileName}"
  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -c ${threads} ${fileName}
  #echo "2 进行分区文件进行游程预测映射"
  pwdPath=$(pwd)
  cd ${fileName}.partition
  #/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} data_1.dat
  #/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} data_2.dat

  echo "2 使用ZPAQ算法进行级联压缩"
  zpaq a partition_dat.zpaq partition_dat -method 5 -threads ${threads}
  zpaq a data_1.dat.zpaq data_1.dat -method 5 -threads ${threads}
  zpaq a data_2.dat.zpaq data_2.dat -method 5 -threads ${threads}

  echo "3 打包为压缩文件"
  tar -cf result.pqsdc_v1 data_1.dat.zpaq data_2.dat.zpaq partition_dat.zpaq
  ls -l --block-size=1 result.pqsdc_v1
  echo "4 删除产生的中间文件"
  #mv ${fileName%%.qualities}.pqsrc_v1 ${pwdPath}
  rm -rf *dat*
  cd ${pwdPath}
  #rm -rf ${fileName}.partition
fi

if [ "${mode}" = "d" ]; then # 输入文件夹
  echo "de-compression mode"
  echo "1 使用tar解包文件"
  pwdPath=$(pwd)
  cd ${fileName}
  tar -xvf result.pqsdc_v1

  echo "2 使用zpaq算法解压缩文件"
  zpaq x partition_dat.zpaq -method 5 -threads ${threads}
  zpaq x data_1.dat.zpaq -method 5 -threads ${threads}
  zpaq x data_2.dat.zpaq -method 5 -threads ${threads}

  #echo "3 进行分区文件进行游程预测映射"
  #/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -d ${threads} data_1.dat.PQVRC
  #/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -d ${threads} data_2.dat.PQVRC
  echo "3 合并分区恢复原始文件"

  mv data_1.dat data_1.dat.PQVRC_de
  mv data_2.dat data_2.dat.PQVRC_de
  cd ..
  pwd
  /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -d ${threads} ${fileName}
  #rm -rf ${fileName}
  cd ${pwdPath}

fi
