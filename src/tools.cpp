#include <iostream>
#include "tools.h"
#include <string.h>
#include <string>
#include <fstream>
#include <cstdlib>
#include <cstdio>
#include <stdio.h>
#include <vector>
#include <algorithm>
#include <iomanip>
#include <functional>
#include <dirent.h>
using namespace std;


/* 判断文件是否是FastQ文件 */
bool isFastqFile(const std::string& fileName){
    std::string ext = fileName.substr(fileName.find_last_of(".") + 1);
    return ext == "fastq" || ext == "fq";
}

/* 获取输入文件的统计信息 */
int getStatisticsInfo(std::string fileName){
    uint64_t totalReadsNum = 0;        /* 总序列数目 */
    uint64_t totalReadsNumWithN = 0;   /* 含有碱基“N”的总序列数目 */
    uint64_t totalBases = 0;           /* 总的DNA碱基数目 */
    uint64_t totalBasesN = 0;          /* 总的N碱基数目 */
    uint64_t minReadLength = 1844674407370955161;        /* 最短序列长度 */
    uint64_t maxReadLength = 0;        /* 最长序列长度 */
    uint64_t avgReadLength = 0;        /* 平均序列长度 */

    /*判断输入文件是否是FastQ类型文件*/
    if (!isFastqFile(fileName)) {cout << "Please input *.fastq file." << endl; exit(1);}
    ifstream infile(fileName);
    if (!infile.is_open()) {cout << "Failed to open file: " << fileName << "." << endl; exit(1);}

    string line;
    int tempCounter = 0;
    while(getline(infile, line)){
    if (tempCounter == 1){ /*找到reads所在的行*/
        totalReadsNum ++;
        uint64_t readLen = line.length();
        int count = std::count(line.begin(), line.end(), 'N');
        if(count != 0) {
            totalReadsNumWithN ++;
            totalBasesN = totalBasesN + count;
        }
        if(readLen < minReadLength) minReadLength = readLen;
        if(readLen > maxReadLength) maxReadLength = readLen;
        totalBases = totalBases + readLen;
    }
    tempCounter ++;
    tempCounter = tempCounter % 4;
    }
    if(PrintFlag){
        avgReadLength = totalBases/totalReadsNum;
        cout << "**************************************************************" << endl;
        cout << "File:\t" << fileName << endl;
        cout << "totalReadsNum:\t" << totalReadsNum << endl;
        cout << "totalReadsNumWithN:\t" << totalReadsNumWithN << endl;
        cout << "totalBases:\t" << totalBases << endl;
        cout << "totalBasesN:\t" << totalBasesN << endl;
        cout << "minReadLength:\t" << minReadLength << endl;
        cout << "maxReadLength:\t" << maxReadLength << endl;
        cout << "avgReadLength:\t" << avgReadLength << endl;
        cout << "reads_raw_size(Bits):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum) * 8   << endl;
        cout << "reads_raw_size(B):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)   << endl;
        cout << "reads_raw_size(KB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024)  << endl;
        cout << "reads_raw_size(MB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024 * 1024)  << endl;
        cout << "reads_raw_size(GB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024 * 1024 * 1024) << endl;
        /* 获取测试文件大小 */
        const char* filePath = fileName.c_str();
        FILE* file = std::fopen(filePath, "rb");
        std::fseek(file, 0, SEEK_END);
        long fileSize = std::ftell(file) + 4*totalReadsNum;
        float fileSizeInKB = static_cast<float>(fileSize) / (1024); /* 获取文件大小（以KB为单位）*/
        float fileSizeInMB = static_cast<float>(fileSize) / (1024 * 1024); /* 获取文件大小（以MB为单位）*/
        float fileSizeInGB = static_cast<float>(fileSize) / (1024 * 1024 * 1024); /* 获取文件大小（以GB为单位）*/
        cout << "file_raw_size(Bits):\t" << std::fixed << std::setprecision(3) << fileSize * 8 << endl;
        cout << "file_raw_size(B):\t" << std::fixed << std::setprecision(3) << fileSize << endl;
        cout << "file_raw_size(KB):\t" << std::fixed << std::setprecision(3) << fileSizeInKB << endl;
        cout << "file_raw_size(MB):\t" << std::fixed << std::setprecision(3) << fileSizeInMB << endl;
        cout << "file_raw_size(GB):\t" << std::fixed << std::setprecision(3) << fileSizeInGB << endl;
        cout << "Plase using 'ls -l --block-size=KB *.fastq' for desk file sizes" << endl;

    }
    infile.close();
    return 0;
}

/* 解析命令行参数 */
std::string parseCommandLine(int argc, char* argv[], std::string parameter){
    std::string path;
    std::vector<std::string> args(argv, argv + argc);

    auto it = std::find(args.begin(), args.end(), parameter);
    if (it != args.end() && ++it != args.end()) {
        path = *it;
    }

    return path;
}

/* 获取字符串哈希值 */
std::size_t easyHash(std::string str){
    std::hash<std::string> hash_function;;
    std::size_t hash_value = hash_function(str);
    //std::cout << "Hash value: " << hash_value << std::endl;
    return hash_value;
}

/* 验证文件是否解压缩成功 */
int verify(std::string fileA, std::string type, std::string fileB){
    /*  fileA: 原始FastQ文件
    **  fileB: 解压缩恢复文件
    **  type : reads or qualities
    */
    ifstream infileA(fileA);
    if (!infileA.is_open()) {cout << "Failed to open file: " << fileA << "." << endl; exit(1);}
    if (!isFastqFile(fileA)) {cout << "Please input *.fastq file." << endl; exit(1);}
    ifstream infileB(fileB);
    if (!infileB.is_open()) {cout << "Failed to open file: " << fileB << "." << endl; exit(1);}
    if (isFastqFile(fileB)) {cout << "Please input pure *qualities or *reads file." << endl; exit(1);}
    string lineA, lineB;
    int tempCounter = 0;
    int unRecoverNum = 0;
    while(getline(infileA, lineA)){
        if(tempCounter%4 == 1 && type == "reads"){
            getline(infileB, lineB);
            if(easyHash(lineB)!=easyHash(lineA)){
                cout << "Wrong lossless recover reads at line: " << tempCounter << endl;
                cout << "lineA: " << lineA << endl;
                cout << "lineB: " << lineB << endl;
                unRecoverNum ++;
            }
        }
        if(tempCounter%4 == 3 && type == "qualities"){
            getline(infileB, lineB);
            if(easyHash(lineB)!=easyHash(lineA)){
                cout << "Wrong lossless recover qualities at line: " << tempCounter << endl;
                cout << "lineA: " << lineA << endl;
                cout << "lineB: " << lineB << endl;
                unRecoverNum ++;
            }
        }
        tempCounter ++;
    }
    infileB.close();
    infileA.close();
    return unRecoverNum;
}

/* 获取输入文件夹的统计信息 */
int getDirInfo(std::string dirPath){
    vector<std::string> fileNameList;
    DIR* dir = opendir(dirPath.c_str());
    if (dir == nullptr) {
        std::cerr << "Failed to open directory" << std::endl;
        return 1;
    }
    struct dirent* entry;
    uint64_t totalReadsNum = 0;        /* 总序列数目 */
    uint64_t totalReadsNumWithN = 0;   /* 含有碱基“N”的总序列数目 */
    uint64_t totalBases = 0;           /* 总的DNA碱基数目 */
    uint64_t totalBasesN = 0;          /* 总的N碱基数目 */
    uint64_t minReadLength = 1844674407370955161;        /* 最短序列长度 */
    uint64_t maxReadLength = 0;        /* 最长序列长度 */
    uint64_t avgReadLength = 0;        /* 平均序列长度 */
    int fileNum = 0;                   /* 文件数目 */
    uint64_t totalFileSize = 0;

    while ((entry = readdir(dir)) != nullptr) {
        // 如果当前实体是文件，则输出文件名
        if (entry->d_type == DT_REG and isFastqFile(entry->d_name)) {
            fileNum ++;
            /*判断输入文件是否是FastQ类型文件*/
            std::string fileName = dirPath + "/" + entry->d_name;
            fileNameList.push_back(fileName);
            ifstream infile(fileName);
            if (!infile.is_open()) {cout << "Failed to open file: " << fileName << "." << endl; exit(1);}
            string line;
            int tempCounter = 0;
            while(getline(infile, line)){
            if (tempCounter == 1){ /*找到reads所在的行*/
                totalReadsNum ++;
                uint64_t readLen = line.length();
                int count = std::count(line.begin(), line.end(), 'N');
                if(count != 0) {
                    totalReadsNumWithN ++;
                    totalBasesN = totalBasesN + count;
                }
                if(readLen < minReadLength) minReadLength = readLen;
                if(readLen > maxReadLength) maxReadLength = readLen;
                totalBases = totalBases + readLen;
            }
            tempCounter ++;
            tempCounter = tempCounter % 4;
            }
            infile.close();
            /* 获取测试文件大小 */
            const char* filePath = fileName.c_str();
            FILE* file = std::fopen(filePath, "rb");
            std::fseek(file, 0, SEEK_END);
            long fileSize = std::ftell(file);
            totalFileSize = fileSize + totalFileSize;


        }
    }
    if(PrintFlag){
        avgReadLength = totalBases/totalReadsNum;
        cout << "**************************************************************" << endl;
        cout << "DIR:\t" << dirPath << endl;
        cout << "totalReadsNum:\t" << totalReadsNum << endl;
        cout << "totalReadsNumWithN:\t" << totalReadsNumWithN << endl;
        cout << "totalBases:\t" << totalBases << endl;
        cout << "totalBasesN:\t" << totalBasesN << endl;
        cout << "minReadLength:\t" << minReadLength << endl;
        cout << "maxReadLength:\t" << maxReadLength << endl;
        cout << "avgReadLength:\t" << avgReadLength << endl;
        cout << "reads_raw_size(Bits):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum) * 8   << endl;
        cout << "reads_raw_size(B):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)    << endl;
        cout << "reads_raw_size(KB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024)  << endl;
        cout << "reads_raw_size(MB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024 * 1024)  << endl;
        cout << "reads_raw_size(GB):\t" << std::fixed << std::setprecision(3) << float(totalBases + totalReadsNum)  / float(1024 * 1024 * 1024) << endl;
        totalFileSize = totalFileSize + 4*totalReadsNum;
        cout << "file_raw_size(Bits):\t" << std::fixed << std::setprecision(3) << static_cast<float>(totalFileSize) * 8 << endl;
        cout << "file_raw_size(B):\t" << std::fixed << std::setprecision(3) << static_cast<float>(totalFileSize)  << endl;
        cout << "file_raw_size(KB):\t" << std::fixed << std::setprecision(3) << static_cast<float>(totalFileSize) / float(1024 ) << endl;
        cout << "file_raw_size(MB):\t" << std::fixed << std::setprecision(3) << static_cast<float>(totalFileSize) / float(1024 * 1024) << endl;
        cout << "file_raw_size(GB):\t" << std::fixed << std::setprecision(3) << static_cast<float>(totalFileSize) / float(1024 * 1024 * 1024) << endl;
        cout << "fileNum:\t" << fileNum << endl;
        for(int i=0; i< fileNum; i++) cout << "\t" << fileNameList[i] << endl;
    }


    return 0;
}

/* 获取输入文件的文件名 */
std::string getPureFileName(const std::string fileName){
    std::string input_file_name_without_extension = fileName.substr(0, fileName.find_last_of('.'));
    return input_file_name_without_extension;
}

/* 分割原始文件 */
int splitFastqFile(std::string fileName, std::string mode){
    if (!isFastqFile(fileName)) {cout << "Please input *.fastq file." << endl; exit(1);}
    ifstream infile(fileName);
    if (!infile.is_open()) {cout << "Failed to open file: " << fileName << "." << endl; exit(1);}
    std::string pureName = getPureFileName(fileName);
    ofstream outQualities, outIds, outReads, outDescribes;
    if (mode == "qualities") { outQualities.open(pureName + ".qualities");}
    if (mode == "ids") { outIds.open(pureName + ".ids");}
    if (mode == "reads") { outReads.open(pureName + ".reads");}
    if (mode == "describes") { outDescribes.open(pureName + ".describes");}
    if (mode == "all") {
        outQualities.open(pureName + ".qualities");
        outIds.open(pureName + ".ids");
        outReads.open(pureName + ".reads");
        outDescribes.open(pureName + ".describes");}

    std::string line;
    int tempCounter = 0;
    while(getline(infile, line)){
        if(tempCounter == 0 && mode == "ids") outIds << line << "\n";
        if(tempCounter == 1 && mode == "reads") outReads << line << "\n";
        if(tempCounter == 2 && mode == "describes") outDescribes << line << "\n";
        if(tempCounter == 3 && mode == "qualities") outQualities << line << "\n";
        if(mode == "all"){
            if(tempCounter == 0) outIds << line << "\n";
            if(tempCounter == 1) outReads << line << "\n";
            if(tempCounter == 2) outDescribes << line << "\n";
            if(tempCounter == 3) outQualities << line << "\n";
        }
        tempCounter ++;
        tempCounter = tempCounter % 4;
    }

    infile.close();
    if (mode == "qualities") { outQualities.close();}
    if (mode == "ids") { outIds.close();}
    if (mode == "reads") { outReads.close();}
    if (mode == "describes") { outDescribes.close();}
    if (mode == "all") { outQualities.close(); outIds.close(); outReads.close(); outDescribes.close(); }

    return 0;
}