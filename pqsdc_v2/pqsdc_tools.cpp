#include <iostream>
#include <string>
#include "tools.h"
#include "message.h"
#include <vector>
using namespace std;

// 所有文件调用写到底下man函数里

int main(int argc, char* argv[]){

    /* 判断是否包含-help指令 */
    parseHelpLine(argc, argv);

    /* 判断是否包含-fileinfo指令 */
    std::string path = parseCommandLine(argc, argv, "-fileinfo");
    if(!path.empty()){
        getStatisticsInfo(path);
        exit(1);
    }

    /* 判断是否包含-verify指令 */
    std::string pathFileA = parseCommandLine(argc, argv, "-verify");
    if(!pathFileA.empty()){

        std::string pathFileB = parseCommandLine(argc, argv, "reads");
        if(!pathFileB.empty()){ /* 验证reads */
            if(verify(pathFileA, "reads", pathFileB) == 0) cout << "lossless recover all reads." << endl;
            exit(1);
        }

        pathFileB = parseCommandLine(argc, argv, "qualities");
        if(!pathFileB.empty()){ /* 验证qualities */
            if(verify(pathFileA, "qualities", pathFileB) == 0) cout << "lossless recover all qualitiess." << endl;
            exit(1);
        }

    }

     /* 判断是否包含-dirinfo指令 */
    path = parseCommandLine(argc, argv, "-dirinfo");
    if(!path.empty()){
        getDirInfo(path);
        exit(1);
    }

    /* 判断是否包含-filesplite指令 */
    path = parseCommandLine(argc, argv, "-filesplite");
    if(!path.empty()){
        std::string mode = parseCommandLine(argc, argv, "mode");
        if(mode == "ids" || mode == "reads" || mode == "qualities" || mode == "describes" || mode == "all") {
            splitFastqFile(path, mode);
            exit(1);}
        else {cout << "wrong parameters." << endl; exit(1);}
    }

    helpFunc();



}