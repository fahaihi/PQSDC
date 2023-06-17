#!/bin/bash

# 对于双端文件、采用cat file_1.fastq file_2.fastq > file.fastq 进行合并压缩
for index in SRR8386204 SRR8386224 SRR8386225 ERR7091256 ERR7091268 SRR013951 SRR027520 SRR554369 SRR17794741 SRR17794724 SRR12175235; do
  echo "${index}............................."
  prefetch ${index}
  fastq-dump --split-files ${index}
  rm -rf ${index}
done