#!/bin/bash
echo "tes universal compressors"
# /public/home/jd_sunhui/genCompressor/PQSDC/script/test_universal_compressors.sh
fileName=$1
threads=$2
# 每执行一组算法暂停sleep_time秒
sleep_time=5
prefix=${fileName%.*}

directory="result"
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
name1="Algorithm"
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
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-10s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >${directory}/${saveCSV}



echo "Algorithm10: test QVZ Algorithm ********************************************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="QVZ"
QVZ_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/qvz/bin"
pwdPath=$(pwd)
# 1 切换到QVZ路径并配置文件并将测序数据暂时存放在QVZ目录
cp ${fileName} ${QVZ_PATH}
cd ${QVZ_PATH}
# 2 调用QVZ算法执行压缩
(/bin/time -v -p ./qvz -q -f 1 ${fileName} ${prefix}.qvz) > ${name1}_${prefix}.log 2>&1
cat ${name1}_${prefix}.log
# 3 计算压缩率、时间和内存等
name2=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${prefix}.qvz | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
# 4 解压缩
(/bin/time -v -p ./qvz -x ${prefix}.qvz ${prefix}.qvz_recover) > ${name1}_${prefix}.log 2>&1
cat ${name1}_${prefix}.log
# 5 统计解压缩信息等
name7=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
# 6 将解压缩结果传到主脚本,验证是否解压缩成功
mv ${prefix}.qvz_recover ${pwdPath}
rm -rf ${fileName}
cd ${pwdPath}
PLRC -verify ${prefix}.fastq qualities ${prefix}.qvz_recover
rm -rf ${prefix}.qvz_recover
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
cat ${directory}/${saveName}




echo "Algorithm9: test AQUa Algorithm ********************************************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="AQUa"
AQUa_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/AQUa"
pwdPath=$(pwd)
# 1 切换到AQUa路径并配置文件并将测序数据暂时存放在AQUa目录
cp ${fileName} ${AQUa_PATH}
cd ${AQUa_PATH}
echo >${prefix}_${name1}.properties
echo "inputfile=${fileName}" >>${prefix}_${name1}.properties                   # 算法处理的文件
echo "outputfile=${prefix}.aqua" >>${prefix}_${name1}.properties               # 压缩后写入的文件
echo "metadatafile=${prefix}_metadata.xml" >>${prefix}_${name1}.properties     # 随机访问元数据文件
echo "sequenceindexfile=sequences.xml" >>${prefix}_${name1}.properties         # 这个目前应该用不到
echo "statisticsfile=${prefix}_statistics.txt" >>${prefix}_${name1}.properties # 记录一些属性信息
echo "inputsize=${file_bases}" >>${prefix}_${name1}.properties                 # 记录总的碱基数目
echo "mode=ENCODE" >>${prefix}_${name1}.properties                             # 告诉算法当前是编码压缩模式
echo "dnaformat=PLAIN" >>${prefix}_${name1}.properties                         # 告诉算法当前输入的是纯Quality文件
echo "blocksize=${length}" >>${prefix}_${name1}.properties                     # 告诉算法当前输入的长度
echo "windowsize=4" >>${prefix}_${name1}.properties                            # 告诉算法当前输入的长度
echo "cabac=true" >>${prefix}_${name1}.properties
echo "cabacgrouping=${nums}" >>${prefix}_${name1}.properties
echo "hardreset=true" >>${prefix}_${name1}.properties
echo "hardresetboundary=${nums}" >>${prefix}_${name1}.properties
echo "alphabets=QUAL" >>${prefix}_${name1}.properties
echo "tools=DFC,NSP,CVP,SRP,AVP,HNSP,DFT" >>${prefix}_${name1}.properties
cat ${prefix}_${name1}.properties
# 2 调用AQUa算法执行压缩,并记录压缩结果
(/bin/time -v -p java -jar AQUa.jar ${prefix}_${name1}.properties) > ${name1}_${prefix}.log 2>&1
echo "***********************************************************************"
tail -50 ${name1}_${prefix}.log
name2=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${prefix}.aqua | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
# 3 编辑配置解压缩信息
echo >${prefix}_${name1}_de_.properties
echo "inputfile=${prefix}.aqua" >>${prefix}_${name1}_de_.properties
echo "outputfile=${prefix}.aqua_recov" >>${prefix}_${name1}_de_.properties
echo "metadatafile=${prefix}_metadata.xml" >>${prefix}_${name1}_de_.properties
echo "sequenceindexfile=sequences.xml" >>${prefix}_${name1}_de_.properties
echo "statisticsfile=${prefix}_statistics.txt" >>${prefix}_${name1}_de_.properties
echo "inputsize=${file_bases}" >>${prefix}_${name1}_de_.properties
echo "mode=DECODE" >>${prefix}_${name1}_de_.properties
echo "dnaformat=PLAIN" >>${prefix}_${name1}_de_.properties
echo "alphabets=QUAL" >>${prefix}_${name1}_de_.properties
echo "tools=DFC,NSP,CVP,SRP,AVP,HNSP,DFT" >>${prefix}_${name1}_de_.properties
cat ${prefix}_${name1}_de_.properties
# 3 调用AQUa算法解压缩，并记录解压缩结果
(/bin/time -v -p java -jar AQUa.jar ${prefix}_${name1}_de_.properties) >${name1}_${prefix}.log 2>&1
echo "************************************************************"
tail -50 ${name1}_${prefix}.log
name7=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
# 4 删除多余文件，并将解压缩结果传到主脚本
rm -rf ${fileName} ${prefix}_${name1}_de_.properties ${prefix}_${name1}.properties ${name1}_${prefix}.log
rm -rf ${fileName}.aqua ${prefix}_metadata.xml ${prefix}_statistics.txt
mv ${prefix}.aqua_recov ${pwdPath}
# 5 验证是否解压缩成功
cd ${pwdPath}
PLRC -verify ${prefix}.fastq qualities ${prefix}.aqua_recov
rm -rf ${prefix}.aqua_recov
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
cat ${directory}/${saveName}




echo "Algorithm6: test FCLQC Algorithm ********************************************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
echo Notion: FCLQC input Fastq-File
# 1 设置FCLQC目录和配置文件
name1="FCLQC"
FCLQC_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/FCLQC/target/release/main"
#CONFIG_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/FCLQC/sample_data/parameter.json"
FastqName=${prefix}.fastq
CONFIG_PATH=${prefix}_parameter.json
fz=$(expr ${nums} / ${threads})
echo ${FastqName}
echo "CONFIG:"
echo "{" >${CONFIG_PATH}
echo "\"precision\": 35," >>${CONFIG_PATH}
echo "\"file_size\": ${fz}," >>${CONFIG_PATH}
echo "\"thread_num\":${threads}," >>${CONFIG_PATH}
echo "\"first_line\":3," >>${CONFIG_PATH}
echo "\"last_line\": $(expr ${fz} - 3)" >>${CONFIG_PATH}
echo "}" >>${CONFIG_PATH}
cat ${CONFIG_PATH}
echo ${FastqName}
# 2 调用算法执行压缩且将记录打印至${name1}_${prefix}.log
(/bin/time -v -p ${FCLQC_PATH} -c ${FastqName} ${FastqName}.cflqc ${CONFIG_PATH}) >${directory}/${name1}_${prefix}.log 2>&1
#cargo run ${FCLQC_PATH} -c ${FastqName} ${FastqName}.cflqc ${CONFIG_PATH}
cat ${directory}/${name1}_${prefix}.log
# 3 统计记录压缩率、压缩时间、内存等信息
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'encoded size =.*' | grep -o '[0-9]*')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
# 4 解压缩
(/bin/time -v -p ${FCLQC_PATH} -d ${FastqName}.cflqc ${FastqName}.cflqc.dec ${CONFIG_PATH}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
cat ${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
rm -rf ${FastqName}.cflqc*
rm -rf ${CONFIG_PATH}
echo
echo
sleep ${sleep_time}

echo "Algorithm8: test LCQS Algorithm ********************************************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="LCQS"
LCQS_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/LCQS"
# 1 将数据copy至LCQS目录
module load compiler/gnu/gcc-compiler-8.4.0
cp ${fileName} ${LCQS_PATH}
# 2 切换到LCQS目录行
pwdPath=$(pwd)
pwd
cd ${LCQS_PATH}
#make clean
#make
pwd
# 3 运行lcqs，将时间内存等信息记录至当前路径下${name1}_${prefix}.log
#module load compiler/gnu/gcc-compiler-8.4.0
#echo "${LD_LIBRARY_PATH}"
/bin/time -v -p ./lcqs c ${fileName} ${prefix}.lcqs >${name1}_${prefix}.log 2>&1
# 4 打印lcqs运行信息
cat ${name1}_${prefix}.log
# 5 计算压缩率、时间和内存等信息
name2=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name2=$(timer_reans $name2)
name3=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${prefix}.lcqs | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
# 5 移除当前目录下的${name1}_${prefix}.log
rm -rf ${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
pwd
# 6 解压缩恢复文件
#module load compiler/gnu/gcc-compiler-8.4.0
/bin/time -v -p ./lcqs d ${prefix}.lcqs ${prefix}.lcqs_recover >${name1}_${prefix}.log 2>&1
# 7 打印lcqs运行信息
cat ${directory}/${name1}_${prefix}.log
# 8 计算、时间和内存等信息
name7=$(cat ${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name7=$(timer_reans $name7)
name8=$(cat ${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
# 9 将恢复文件转移至脚本文件
cp ${prefix}.lcqs_recover ${pwdPath}
# 10 删除多余文件并返回主目录
rm -rf ${prefix}.lcqs_recover ${prefix}.lcqs ${name1}_${prefix}.log
cd ${pwdPath}
pwd
# 11 验证是否完全恢复
PLRC -verify ${prefix}.fastq qualities ${prefix}.lcqs_recover
# 12 删除多余文件并保存记录
rm -rf ${prefix}.lcqs_recover
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
echo
echo
#cat  ${directory}/${saveName}
sleep ${sleep_time}

echo "Algorithm1: test 7Z Algorithm ********************************************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="7-Zip"
/bin/time -v -p 7zz a -mx9 -mmt${threads} ${directory}/${prefix}.7z ${fileName} >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name2=$(timer_reans $name2)
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${directory}/${prefix}.7z | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
cd ${directory}
/bin/time -v -p 7zz x -mx9 -mmt${threads} ${prefix}.7z >${name1}_${prefix}.log 2>&1
cd ..
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name7=$(timer_reans $name7)
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
PLRC -verify ${prefix}.fastq qualities ${directory}/${fileName}
rm -rf ${directory}/${fileName}
rm -rf ${directory}/${prefix}.7z
rm -rf ${directory}/${name1}_${prefix}.log
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
echo
echo
sleep ${sleep_time}

echo "Algorithm2:test PIGZ Algorithm *********************************************************"
name1="PIGZ"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
(/bin/time -v -p pigz -c -11 -p "${threads}" "${fileName}" >${directory}/${prefix}.gz) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name2=$(timer_reans $name2)
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${directory}/${prefix}.gz | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
(/bin/time -v -p pigz -dc -p ${threads} ${directory}/${prefix}.gz >${directory}/${prefix}.qualities) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name7=$(timer_reans $name7)
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
cat $directory/$saveName
#exit 0
PLRC -verify ${prefix}.fastq qualities ${directory}/${prefix}.qualities
rm -rf ${directory}/${prefix}.qualities
rm -rf ${directory}/${prefix}.gz
rm -rf ${directory}/${name1}_${prefix}.log
echo
echo
echo
sleep ${sleep_time}

echo "Algorithm3:test PBzip2 Algorithm *********************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="PBzip2"
(/bin/time -v -p pbzip2 -9 -m2000 -p${threads} -c ${fileName} >${directory}/${prefix}.bz2) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${directory}/${prefix}.bz2 | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
(/bin/time -v -p pbzip2 -dc -9 -p${threads} -m2000 ${directory}/${prefix}.bz2 >${directory}/${prefix}.qualities) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
PLRC -verify ${prefix}.fastq qualities ${directory}/${prefix}.qualities
rm -rf ${directory}/{prefix}.qualities
rm -rf ${directory}/${prefix}.bz2
rm -rf ${directory}/${name1}_${prefix}.log
echo
echo
echo
sleep ${sleep_time}

echo "Algorithm4: test ZPAQ Algorithm *********************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="ZPAQ"
(/bin/time -v -p zpaq a ${prefix}.zpaq ${fileName} -method 5 -threads ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${prefix}.zpaq | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
(/bin/time -v -p zpaq x ${prefix}.zpaq -method 5 -threads ${threads} -to ${directory}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
PLRC -verify ${prefix}.fastq qualities ${directory}/${fileName}
rm -rf ${directory}/${fileName}
rm -rf ${prefix}.zpaq
rm -rf ${directory}/${name1}_${prefix}.log
echo
echo
echo
sleep ${sleep_time}

echo "Algorithm5: test CMIC Algorithm *********************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="CMIC"
CMIC_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/CMIC/cmic.sh"
(/bin/time -v -p ${CMIC_PATH} c ${fileName} ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${fileName}.CMIC.zpaq | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
(/bin/time -v -p ${CMIC_PATH} d ${fileName}.CMIC.zpaq ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
#cat ${directory}/${saveName}
PLRC -verify ${prefix}.fastq qualities ${fileName}.CMIC_de
rm -rf ${fileName}.CMIC.zpaq
rm -rf ${fileName}.CMIC_de
rm ${directory}/${name1}_${prefix}.log
#exit 0

echo "Algorithm7: test Qscomp Algorithm *********************************************************"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++compression"
name1="Qscomp"
Qscomp_PATH="/public/home/jd_sunhui/genCompressor/PQSDC/source/Qscomp/qscomp.sh"
(/bin/time -v -p ${Qscomp_PATH} c ${fileName} ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name2=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name3=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name4=$(ls -lah --block-size=1 ${fileName}.out.tar.zpaq | awk '/^[-d]/ {print $5}')
name5=$(echo "scale=3; 8*${name4}/${file_bases}" | bc)
name6=$(echo "scale=3; ${file_sizes}/${name4}" | bc)
rm -rf ${directory}/${name1}_${prefix}.log
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++decompression"
(/bin/time -v -p ${Qscomp_PATH} d ${fileName}.out.tar.zpaq ${threads}) >${directory}/${name1}_${prefix}.log 2>&1
cat ${directory}/${name1}_${prefix}.log
name7=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name8=$(cat ${directory}/${name1}_${prefix}.log | grep -o 'Maximum resident set size.*' | grep -o '[0-9]*')
name2=$(timer_reans $name2)
name7=$(timer_reans $name7)
PLRC -verify ${prefix}.fastq qualities ${fileName}.de_comp
printf "%-10s %-15s %-10s %-10s %-10s %-10s %-15s %-10s\n" ${name1} ${name2} ${name3} ${name4} ${name5} ${name6} ${name7} ${name8} >>${directory}/${saveName}
echo "${name1},${name2},${name3},${name4},${name5},${name6},${name7},${name8}" >>${directory}/${saveCSV}
#cat ${directory}/${saveName}
rm -rf ${fileName}.out.tar.zpaq
rm -rf ${fileName}.de_comp
rm ${directory}/${name1}_${prefix}.log
cat ${directory}/${saveName}
#exit 0
