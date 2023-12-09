#!/bin/bash
echo "test PQSDC CPU CLUSTER parallel efficiency"
# /public/home/jd_sunhui/genCompressor/PQSDC/script/test_parallel_Pr.sh



threads=8
Nu=0
for Nu in 1 2 3 4 5 6 7 8; do

done

fileNameC="SRR027520.qualities"
Nu=5
threads=4
G=6
sbatch -p gpu1 -N ${Nu} -c ${threads} -n ${Nu} -e cluster/${fileNameC}_${Nu}_c.err -o cluster/${fileNameC}_${Nu}_c.out /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_cluster.slurm -c ${fileNameC} ${threads} ${G}

fileNameD="SRR027520.qualities.partition"
Nu=5
threads=4
G=6
sbatch -p gpu1 -N ${Nu} -c ${threads} -n ${Nu} -e cluster/${fileNameD}_${Nu}_d.err -o cluster/${fileNameD}_${Nu}_d.out /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_cluster.slurm -d ${fileNameD} ${threads} ${G}



for file in "SRR12175235" "ERR7091256_1" "SRR8386204" "SRR8386224" "SRR554369"; do
  fileNameC="${file}.qualities"
  Nu=7
  threads=4
  G=6
  sbatch -p gpu1 -N ${Nu} -c ${threads} -n ${Nu} -e cluster/${fileNameC}_${Nu}_c.err -o cluster/${fileNameC}_${Nu}_c.out /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_cluster.slurm -c ${fileNameC} ${threads} ${G}
done


for file in "SRR12175235" "ERR7091256_1" "SRR8386204" "SRR8386224" "SRR554369"; do
  fileNameD="${file}.qualities.partition"
  Nu=7
  threads=4
  G=6
  sbatch -p gpu1 -N ${Nu} -c ${threads} -n ${Nu} -e cluster/${fileNameD}_${Nu}_d.err -o cluster/${fileNameD}_${Nu}_d.out /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_cluster.slurm -d ${fileNameD} ${threads} ${G}
done