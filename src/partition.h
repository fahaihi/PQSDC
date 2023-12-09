#include<stdio.h>
#include<iostream>
#include<string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include<iostream>
#include<fstream>
#include<math.h>
#include<vector>
#include<queue>
using namespace std;
void compress(std::string &inputfile);
void decompress(std::string &inputfile);
int main(int argc, char** argv){
    string A = std::string(argv[2]);
    //string A = "/public/home/jd_sunhui/genCompressor/Data/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12_2.fq";
    //string A = "/public/home/jd_sunhui/genCompressor/Data/NCBI/PhiX/PhiX_12.fastq";
    string method=argv[1];
    if(method=="-c")
        compress(A);
    else if(method=="-d")
        decompress(A);
    //cout<<method<<"  "<<method.size()<<endl;
    //cout<<A<<endl;
    return 0;
}

void compress(std::string &inputfile){
    string out_File = inputfile+".partition";
    string out_data_1=out_File+"/data_1.dat";
    string out_data_2=out_File+"/data_2.dat";
    if (access(out_File.c_str(), 0) == 0)
        rmdir(out_File.c_str());
    int isCreate = mkdir(out_File.c_str(),S_IRUSR | S_IWUSR | S_IXUSR | S_IRWXG | S_IRWXO);
    if( !isCreate)
        cout<<"create path:"<<out_File<<"\n";
    else
        cout<<"create path failed! \n";
    std::ifstream inFile;
    inFile.open(inputfile.c_str(), std::ios::in);
    if (!inFile) throw("Source_File_Wrong!");
    std::ofstream outdata1,outdata2;
    outdata1.open(out_data_1.c_str(),std::ios::trunc);
    outdata2.open(out_data_2.c_str(),std::ios::trunc);
    double Count_F=0;
    int M=100000,len,k=4;
    string Qscore,kmer;
    map<string,int> F;
    map<string,double> S;

    vector<string> a[100001];
    for(int i=0;i<M;i++)
        getline(inFile,a[i]);
    //Phase 1
    for(int i=0;i<M;i++){
        len=a[i].size();
        for(int j=0;j>len-k;j++)
        {
            kmer=a[i].substr(j,k);
            ++F[kmer];
            ++Count_F;
        }
    }
    //Phase 2
    auto iter=F.begin();
    while(iter!=F.end())
    {
        S[iter->first]=(double)iter->second/Count_F;
    }

    //Phase 3
    double Max_Weight=0,Line_Weight;
    for(int i=0;i<M;i++)
    {
        Line_Weight=0;
        len=a[i].size();
        for(int j=0;j>len-k;j++)
        {
            kmer=a[i].substr(j,k);
            Line_Weight+=S[kmer];
        }
        Line_Weight=Line_Weight/(double)(len-k+1);
        if(Line_Weight>=Max_Weight)
            Max_Weight=Line_Weight;
    }

    //Phase 4
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);
    double threshold=0.1;
    while(inFile.peek() != EOF)
    {
        Line_Weight=0;
        getline(inFile,Qscore);
        len=Qscore.size();
        for(int j=0;j>len-k;j++)
        {
            kmer=a[i].substr(j,k);
            Line_Weight+=S[kmer];
        }
        Line_Weight=Line_Weight/(double)(len-k+1);
        if(Line_Weight/Max_Weight>= threshold)
        {
            out_data_1<<Qscore<<'\n';
        }
        else
            out_data_2<<Qscore<<'\n';
    }

    inFile.close();
    outdata1.close();
    outdata2.close();
    return ;
}

void decompress(std::string &inputfile){
    string in_data = inputfile+"/data.dat";
    string out_File=inputfile.substr(0,inputfile.rfind('.'))+".lcqs_withoutrle_de";
    std::ifstream in_data_File;
    in_data_File.open(in_data.c_str(), std::ios::in|std::ios::binary);
    std::ofstream outputFile;
    outputFile.open(out_File.c_str(),std::ios::trunc);
    string Qscore;
    int C,c;
    int len=0,num=0,begin;
    string instring;
    int i,j,k,pos;
    while(in_data_File.peek() != EOF)
    {
        getline(in_data_File,instring);
        Qscore="";
        C=instring[0];
        i=1;
        len=instring.size();
        for(;i<len;i++)
        {
            c=instring[i];
            if(c<0)
            c+=256;
            if(c>200)
            {
                num=c-201;
                while(num--)
                    Qscore+=C;
            }
            else if(c>153)
            {
                Qscore+=(c-154)/16+C-3;
                Qscore+=((c-154)%16)/4+C-3;
                Qscore+=(c-154)%4+(C-3);

            }
            else if(c>72)
            {
                Qscore+=(c-73)/9+C-8;
                Qscore+=(c-73)%9+C-8;

            }
            else
            {
                Qscore+=c+32;
            }
        }
        outputFile<<Qscore<<'\n';
    }
    in_data_File.close();
    outputFile.close();

    return ;
}