#!/bin/bash

# 对于双端文件、采用cat file_1.fastq file_2.fastq > file.fastq 进行合并压缩
for index in SRR921889 SRR377645 SRR948355 SRR12175235; do
  echo "${index}............................."
  prefetch ${index}
  fastq-dump --split-files ${index}
  rm -rf ${index}
done