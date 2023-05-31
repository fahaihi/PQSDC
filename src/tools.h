/* ***********************************************************************************
* authors：SH
* date：20230517
* describe：a cpp head file for define some help function
*************************************************************************************/
#ifndef TOOLS_H
#define TOOLS_H


#define PrintFlag 1


bool isFastqFile(const std::string& fileName);
int getStatisticsInfo(std::string fileName);
std::string parseCommandLine(int argc, char* argv[], std::string parameter);
std::size_t easyHash(std::string str);
int verify(std::string fileA, std::string type, std::string fileB);
int getDirInfo(std::string dirPath);
std::string getPureFileName(const std::string fileName);
int splitFastqFile(std::string fileName, std::string mode);



#endif