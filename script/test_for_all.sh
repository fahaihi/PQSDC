#!/bin/bash

threads=16
function same_algorithm() {

  echo "1 测试所有算法的压缩率、时间和内存开销"
  for prefix in SRR8386204 SRR8386224 SRR8386225 ERR7091256_1 ERR7091268_1 SRR013951 SRR027520 SRR554369 SRR17794741 SRR17794724 SRR12175235; do
    fileName=${prefix}.qualities
    threads=28
    nohup srun -p gpu1 -N 1 -c ${threads} /public/home/jd_sunhui/genCompressor/PQSDC/script/test_universal_compressors.sh ${fileName} ${threads} > result/${prefix}.test_log &
  done
}
same_algorithm

function ablation() {

  echo "2 测试PQSDC算法消融实验"
  for prefix in SRR8386204 SRR8386224 SRR8386225 ERR7091256_1 ERR7091268_1 SRR013951 SRR027520 SRR554369 SRR17794741 SRR17794724 SRR12175235; do
    fileName=${prefix}.qualities
    threads=16
    nohup srun -p gpu1 -N 1 -c ${threads} /public/home/jd_sunhui/genCompressor/PQSDC/script/test_ablation_experiment.sh ${fileName} ${threads} > ablation/${prefix}.test_log &
  done
}

function parallel() {

  echo "3 测试PQSDC算法单节点并行效率"
  for prefix in SRR8386204 SRR8386224 SRR8386225 ERR7091256_1 ERR7091268_1 SRR013951 SRR027520 SRR554369 SRR17794741 SRR17794724 SRR12175235; do
    fileName=${prefix}.qualities
    threads=28
    nohup srun -p gpu1 -N 1 -c ${threads} /public/home/jd_sunhui/genCompressor/PQSDC/script/test_parallel_Pr.sh ${fileName} > parallel/${prefix}.test_log &
  done
}
