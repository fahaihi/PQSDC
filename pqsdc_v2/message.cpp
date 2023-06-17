#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

void helpFunc(){
    cout << "-------------------------------------------------------------------------------------------------" << endl;
    cout << "-------------------------------------------------------------------------------------------------" << endl;
    cout << "PQSDC: a Parallel Fixed-Length Quality Score Data Compressor" << endl;
    cout << "Version: 2023.05.30" << endl;
    cout << "Authors: SH, ZYF" << endl;
    cout << "Contact Us: sunh@nbjl.nankai.edu.cn" << endl;


    cout << endl;
    cout << "Basic Usage:" << endl;
    cout << "With '--best mode' as default:" << endl;
    cout << "\t -c [input-qualities-file] -t [threads] -o [output-compressed-file] " << endl;
    cout << "\t                                                    *running PQSDC compressor with best mode. " << endl;
    cout << "\t -d [input-compressed-file] -t [thread] -o [recovered-quality-files]" << endl;
    cout << "\t                                                    *running PQSDC decompressor with best mode. " << endl;
    cout << "With '--fast mode' as default:" << endl;
    cout << "Cluster-config file see: 'fast_mode.config'";
    cout << "\t -c [input-qualities-file] -t [threads] -o [output-compressed-file] --fast [block-size]" << endl;
    cout << "\t                                                    *running PQSDC compressor with fast mode. " << endl;
    cout << "\t -d [input-compressed-file] -t [thread] -o [recovered-quality-files] --fast [block-size]" << endl;
    cout << "\t                                                    *running PQSDC decompressor with fast mode. " << endl;


    cout << endl;
    cout << "Advanced Usage:" << endl;
    cout << "\t -fileinfo [input-fastq-file]                       *print basic statistic information." << endl;
    cout << "\t -dirinfo [input-dir-name]                          *print basic statistic information." << endl;
    cout << "\t -verify [source-fastq-file] <mode> [verify-file]   *verify decompression." << endl;
    cout << "\t \t <mode> = reads" << endl;
    cout << "\t \t <mode> = qualities" << endl;
    cout << "\t -filesplite [input-fastq-file] mode <mode>         *splite a FastQ file according <mode>." << endl;
    cout << "\t \t <mode> = ids" << endl;
    cout << "\t \t <mode> = reads" << endl;
    cout << "\t \t <mode> = describes" << endl;
    cout << "\t \t <mode> = qualities" << endl;
    cout << "\t \t <mode> = all" << endl;
    cout << "\t -help                                              *print this message." << endl;
}

/* 解析命令行参数 */
int parseHelpLine(int argc, char* argv[]){
    std::vector<std::string> args(argv, argv + argc);
    auto it = std::find(args.begin(), args.end(), "-help");
    if (it != args.end()) {helpFunc(); exit(1);}
    return 0;
}

