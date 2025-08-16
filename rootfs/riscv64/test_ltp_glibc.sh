#!/bin/bash

echo "#### OS COMP TEST GROUP START ltp-glibc ####"

# 定义目标目录
target_dir="ltp/testcases/bin"

export PATH=/ts/glibc/$target_dir:$PATH

# 遍历目录下的所有文件
while read test_case_name; do
  # 跳过目录，仅处理文件
  file=$test_case_name
  if [ -f "$file" ]; then
    # 输出文件名
    echo "RUN LTP CASE $test_case_name"

    $file

    ret=$?

    # 输出文件名和返回值
    echo "FAIL LTP CASE $test_case_name : $ret"
  fi
done < /ltp-ok-rv-glibc.txt


echo "#### OS COMP TEST GROUP END ltp-glibc ####"
