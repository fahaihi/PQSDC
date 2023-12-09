#!/bin/bash
echo "test PQSDC ablation experiment"
# /public/home/jd_sunhui/genCompressor/PQSDC/script/test_ablation_experiment.sh [fileName] [threads]
fileName=$1
threads="$2"
# 每执行一组算法暂停sleep_time秒
sleep_time=10
prefix=${fileName%.*}

directory="ablation"
if [ ! -d "$directory" ]; then
  mkdir -p "$directory"
  echo "Created directory: $directory"
else
  echo "Directory already exists: $directory"
fi
# 将时间戳转换为秒
function timer_reans() {
  if [[ $1 == *"."* ]]; then
    #echo "The string contains a dot. ==> m:ss"
    local min=$(echo "$1" | cut -d ':' -f 1)
    local sec=$(echo "$1" | cut -d ':' -f 2 | cut -d '.' -f 1)
    local ms=$(echo "$1" | cut -d '.' -f 2)
    #echo "Minutes: $min"
    #echo "Seconds: $sec"
    #echo "Milliseconds: $ms"
    local result=$(echo "scale=3; 60*${min}+${sec}+$ms/1000+1" | bc)
    #echo "result: $result"
    echo $result
  else
    #echo "The string does not contain a dot. ==> h:mm:ss"
    local hour=$(echo "$1" | cut -d ':' -f 1)
    local min=$(echo "$1" | cut -d ':' -f 2)
    local sec=$(echo "$1" | cut -d ':' -f 3)
    local result=$(echo "scale=3; 3600*${hour}+60*${min}+$sec+1.001" | bc)
    echo $result
  fi
}

# 记录处理后所在位置
saveName=${prefix}.sum_result
# 和上面文件一样，知识将结果保存成CSV格式
saveCSV=${prefix}.sum_result.csv
# 以字节为单位计算文件的大小
file_sizes=$(ls -lah --block-size=1 ${fileName} | awk '/^[-d]/ {print $5}')
# 计算文件的长度
nums=$(wc -l ${fileName} | awk '{print $1}')
# 计算序列数量
length=$(head -10 ${fileName} | wc -L)
# 计算总的碱基数量
file_bases=$((${length} * ${nums}))
echo "print info**************************************************************************************"
echo "test fileName:${fileName}"
echo "prefix:${prefix}"
echo "file_bases:${file_bases}"
echo "nums:${nums}"
echo "length:${length}"
echo "file_sizes:${file_sizes}"
echo "saveName:${saveName}"
name1="Experiment-Type"
name2="CTime(S)"
name3="Cmem(KB)"
name4="CFsize(B)"
name5="bit/base"
name6="ratio"
name7="DTime(S)"
name8="Dmem(KB)"
echo "prefix:${prefix}" >${directory}/${saveName}
echo "file_bases:${file_bases}" >>${directory}/${saveName}
echo "nums(tiao):${nums}" >>${directory}/${saveName}
echo "length(bp):${length}" >>${directory}/${saveName}
echo "file_sizes(B):${file_sizes}" >>${directory}/${saveName}
printf "%-30s %-15s %-10s %-10s %-10s %-10s %-10s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >${directory}/${saveCSV}

function PQSDC_With_Partition_Mapping() {
  echo "*********************************************************"
  local fileName=$1
  local threads=$2
  echo ${fileName}
  echo ${threads}
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
  local name1="with-partition-&-mapping"
  local PQSDC_PATH=/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1.sh
  (/bin/time -v -p ${PQSDC_PATH} c ${fileName} ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name4=$(ls -lah --block-size=1 ${fileName}.partition/result.pqsdc_v1 | awk '/^[-d]/ {print $5}')
  local name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
  local name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
  rm -rf ${directory}/${name1}_${prefix}.log
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
  (/bin/time -v -p ${PQSDC_PATH} d ${fileName}.partition ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name2=$(timer_reans $name2)
  local name7=$(timer_reans $name7)
  printf "%-30s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
  echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
  PLRC -verify ${prefix}.fastq qualities ${fileName}.pqsdc_de_v1
  echo
  echo
  echo
  cat ${directory}/${saveName}
  sleep ${sleep_time}
}

function PQSDC_With_Partition() {
  echo "*********************************************************"
  local fileName=$1
  local threads=$2
  echo ${fileName}
  echo ${threads}
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
  local name1="with-partition"
  local PQSDC_PATH=/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_without_mapping.sh
  (/bin/time -v -p ${PQSDC_PATH} c ${fileName} ${threads}) > ${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name4=$(ls -lah --block-size=1 ${fileName}.partition/result.pqsdc_v1 | awk '/^[-d]/ {print $5}')
  local name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
  local name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
  rm -rf ${directory}/${name1}_${prefix}.log
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
  (/bin/time -v -p ${PQSDC_PATH} d ${fileName}.partition ${threads}) > ${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name2=$(timer_reans $name2)
  local name7=$(timer_reans $name7)
  printf "%-30s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
  echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
  PLRC -verify ${prefix}.fastq qualities ${fileName}.pqsdc_de_v1
  echo
  echo
  echo
  cat ${directory}/${saveName}
  sleep ${sleep_time}
}

function PQSDC_With_Mapping() {
  echo "*********************************************************"
  local fileName=$1
  local threads=$2
  echo ${fileName}
  echo ${threads}
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
  local name1="with-mapping"
  local PQSDC_PATH=/public/home/jd_sunhui/genCompressor/PQSDC/pqsdc_v1/pqsdc_v1_without_partition.sh
  cp ${fileName} ${prefix}_copy.qualities
  rm -rf ${prefix}_copy.qualities.PQVRC.zpaq
  local fileName=${prefix}_copy.qualities
  (/bin/time -v -p ${PQSDC_PATH} c ${fileName} ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name4=$(ls -lah --block-size=1 ${prefix}_copy.qualities.PQVRC.zpaq | awk '/^[-d]/ {print $5}')
  local name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
  local name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
  rm -rf ${directory}/${name1}_${prefix}.log
  echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
  (/bin/time -v -p ${PQSDC_PATH} d ${prefix}_copy.qualities.PQVRC.zpaq ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
  cat ${directory}/${name1}_${prefix}.log
  local name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
  local name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
  local name2=$(timer_reans $name2)
  local name7=$(timer_reans $name7)
  printf "%-30s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
  echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
  PLRC -verify ${prefix}.fastq qualities ${prefix}_copy.qualities.PQVRC_de
  echo
  echo
  echo
  cat ${directory}/${saveName}
  sleep ${sleep_time}
}

cho "PQSDC_With_Mapping ${fileName} ${threads}"
PQSDC_With_Mapping ${fileName} ${threads}
echo
echo
echo "PQSDC_With_Partition ${fileName} ${threads}"
PQSDC_With_Partition ${fileName} ${threads}
echo
echo
echo
echo
echo "PQSDC_With_Partition_Mapping ${fileName} ${threads}"
PQSDC_With_Partition_Mapping ${fileName} ${threads}

