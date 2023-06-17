#!/bin/bash

# 1 安装配置环境依赖https://github.com/zpaq/zpaq.git
# git clone https://github.com/zpaq/zpaq.git 这里已经下载完毕
cd zpaq
make
cd ..

# 2 编译PQSDC
g++ partition.cpp -std=c++11 -fopenmp -O3 -o partition.out
g++ PQVRC.cpp -std=c++11 -fopenmp -O3 -o pqsdc.out
g++ -std=c++11 -Wall -Werror -Wno-unused-variable -O3 -o pqsdc_tools pqsdc_tools.cpp tools.cpp message.cpp

# 3 配置环境(需要外部全局环境执行)
#export PATH=$PATH:`pwd`/
#export PQSDC_V2_PATH="`pwd`/"
#source ~/.bashrc

# 4 激活运行脚本
chmod +x pqsdc_v2