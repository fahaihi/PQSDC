#!/bin/bash
# srun -p gpu1 -N 2 -c 2 /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/test.sh [filename]
# GPU集群节点最多支持30核心每个作业，集群扩展设置单节点启用6核心、最多启用5个节点
fileName=$1
echo "fileName: ${fileName}"
#echo "Number of tasks: $SLURM_NTASKS"
#echo "Partition: $SLURM_JOB_PARTITION"
#echo "CPUs per task: $SLURM_CPUS_ON_NODE"
#echo "Tasks per node: $SLURM_NTASKS_PER_NODE"

# 判断是否是一个节点
if [ ${SLURM_NNODES} == '1' ]; then
  echo "Number of nodes: $SLURM_NNODES"
  threads=$SLURM_CPUS_ON_NODE
  case $SLURM_NODEID in
  0)
    echo "Running on $(hostname)"
    PwdPath=$(pwd)
    echo "PwdPath:${PwdPath}"
    exit 0
    ;;
  esac
fi

# 判断是否是2个节点
# srun -p gpu1 -N 2 -c 2 /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/test.sh test.qualities
if [ ${SLURM_NNODES} == '2' ]; then
  threads=$SLURM_CPUS_ON_NODE
  nodes=( $(scontrol show hostname) )
  {
    case $SLURM_NODEID in
  0)
    echo "Running on $(hostname)"
    PwdPath=$(pwd)
    echo "PwdPath:${PwdPath}"
    echo "*******************${nodes[0]}"
    echo "*******************${nodes[1]}"
    exit
    echo "1: $(hostname)执行并行序列分区"
    /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/partition.out -c ${threads} ${fileName}
  ;;
  esac
  } &
  wait

  case $SLURM_NODEID in
  0)
    echo "2.1: $(hostname)执行并行程预测映射"
    cd ${fileName}.partition
    /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} data_1.dat
    ;;
  1)
    #ssh $(hostname) "wait" &
    echo "2.2: $(hostname)执行并行程预测映射"
    cd ${fileName}.partition
    /public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc.out -c ${threads} data_2.dat
    ;;
  esac

fi

# 判断是否是3个节点
if [ ${SLURM_NNODES} == '3' ]; then
  echo "Number of nodes: $SLURM_NNODES"
fi

# 判断是否是4个节点
if [ ${SLURM_NNODES} == '3' ]; then
  echo "Number of nodes: $SLURM_NNODES"
fi

# 判断是否是5个节点
if [ ${SLURM_NNODES} == '3' ]; then
  echo "Number of nodes: $SLURM_NNODES"
fi

# 判断是否是大于5个节点
if [ ${SLURM_NNODES} ] >'5'; then
  echo "Number of nodes: $SLURM_NNODES"
  exit
fi
