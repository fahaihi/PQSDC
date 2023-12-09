function fileMerge() {
  # 指定要拼接的文件名
  local filename=$1
  # 指定要拼接的块数目
  # 创建一个空文件
  cat /dev/null >"$filename"
  # 使用 cat 命令将分割后的文件拼接起来
  files=$(ls -l $filename.part* | awk '{print $9}')
  #ls -l $filename.part* | awk '{print $9}'
  for file in $files; do
    cat "$file" >>"$filename"
  done
  # 删除分割后的文件
  rm -rf ${filename}.part*
}
fileMerge $1
