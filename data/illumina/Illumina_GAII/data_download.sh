#!/bin/bash
echo "bash file for Illumina_GAII"

# 对于双端文件、采用cat file_1.fastq file_2.fastq > file.fastq 进行合并压缩
for index in SRR554369 SRR2463661 SRR870667; do
  echo "${index}............................."
  prefetch ${index}
  fastq-dump --split-files ${index}
  rm -rf ${index}
done

