#!/bin/bash
function fileSplite() {
    local filename=$1
    local num_blocks=$2
    # 获取文件大小，单位为字节
    filesize=$(stat -c %s "$filename")
    # 计算每个块的大小，向上取整到最接近的整数
    block_size=$(( ($filesize + $num_blocks - 1) / $num_blocks ))
    split --bytes="$block_size" "$filename" "$filename.part"
}
fileSplite $1 $2