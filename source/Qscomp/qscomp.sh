#!/bin/bash
# /public/home/jd_sunhui/genCompressor/PQSDC/source/Qscomp/qscomp.sh [mode] [fileName] [threads]
echo "qscomp algorithm"
mode=$1
fileName=$2
threads=$3

echo $mode
echo $fileName
echo $threads

if [ "${mode}" = "c" ]; then
  echo "compression mode"
  # 使用Qscomp进行映射打包
  /public/home/jd_sunhui/genCompressor/PQSDC/source/Qscomp/Qscomp -c ${fileName}
  # 使用tar打包文件夹
  tar -cvf ${fileName}.out.tar ${fileName}.out
  #tar cf ${fileName}.out.tar.bz2 --use-compress-prog=pbzip2 ${fileName}.out
  # 使用ZPAQ算法压缩打包文件
  zpaq a ${fileName}.out.tar.zpaq ${fileName}.out.tar -method 5 -threads ${threads}
  #pbzip2 -9 -m2000 -p${threads} -c ${fileName}.out.tar >${fileName}.out.tar.bz2
  #pbzip2 -9 -m2000 -p${threads} -c ${fileName} >${directory}/${prefix}.bz2
  #tar -cvf ${fileName}.out.tar ${fileName}.out
  #tar -cvf ${fileName}.out.tar ${fileName}.out && pbzip2 -9 -m2000 -p${threads} ${fileName}.out.tar
  # 删除文件夹
  rm -rf ${fileName}.out
  rm -rf ${fileName}.out.tar
fi

if [ "${mode}" = "d" ]; then
  echo "de-compression mode"
  # 1 使用bzip2算法解包文件
  zpaq x ${fileName} -method 5 -threads ${threads}
  #pbzip2 -dc -9 -p8 -m2000 ${fileName} > ${fileName%%.tar}
  # 2 使用tar解包文件
  tar -xvf ${fileName%%.zpaq}
  #tar -xvf ${fileName} --use-compress-prog=pbzip2
  #3 使用Qscomp算法进行解压缩
  /public/home/jd_sunhui/genCompressor/PQSDC/source/Qscomp/Qscomp -d ${fileName%%.tar.zpaq}
  rm -rf ${fileName%%.zpaq}
fi
