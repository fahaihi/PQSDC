#!/bin/bash
function traverse_folder {
    local folder_path=$1

    for file in "$folder_path"/*; do
        if [ -f "$file" ]; then
            echo "File: $file"
        elif [ -d "$file" ]; then
            echo "Directory: $file"
            traverse_folder "$file"
        fi
    done
}
/bin/time -v -p `traverse_folder "/public/home/jd_sunhui/genCompressor/PQSDC"` > test.log 2>&1

cat test.log

name2=$(cat test.log | grep -o 'Elapsed (wall clock) time (h:mm:ss or m:ss):.*' | awk '{print $8}')
name2="1:23:45"
echo $name2



result=$(timer_reans $name2)
echo "timer_reans: $result"