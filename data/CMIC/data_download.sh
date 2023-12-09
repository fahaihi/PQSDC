#!/bin/bash

# 对于双端文件、采用cat file_1.fastq file_2.fastq > file.fastq 进行合并压缩
for index in ERR091571 SRR870667 SRR327342 SRR007215 SRR010712 SRR801793; do
  echo "${index}............................."
  prefetch ${index}
  fastq-dump --split-files ${index}
  rm -rf ${index}
done