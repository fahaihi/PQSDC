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
#include<map>
#include<bitset>
#include <omp.h>
#include<time.h>
using namespace std;
void compress(std::string &inputfile,int thread);
void old_compress(std::string &inputfile,int thread);
void decompress(std::string &inputfile);
int main(int argc, char** argv){
    string A = std::string(argv[3]);
    //string A = "/public/home/jd_sunhui/genCompressor/Data/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12_2.fq";
    //string A = "/public/home/jd_sunhui/genCompressor/Data/NCBI/PhiX/PhiX_12.fastq";
    string method=argv[1];
    int thread=std::stoi(argv[2]);
    if(method=="-c")
    {
        //old_compress(A,thread);
        compress(A,thread);
    }
    else if(method=="-d")
        decompress(A);
    //cout<<method<<"  "<<method.size()<<endl;
    //cout<<A<<endl;
    return 0;
}

void compress(std::string &inputfile,int thread){
    string out_File = inputfile+".partition";
    string out_partition=out_File+"/partition_dat";
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
    std::ofstream outdata1,outdata2,outpar;
    outdata1.open(out_data_1.c_str(),std::ios::trunc);
    outdata2.open(out_data_2.c_str(),std::ios::trunc);
    outpar.open(out_partition.c_str(),std::ios::trunc|std::ios::binary);
    double Count_F=0;
    int M=100000,len,k=4;
    string Qscore,kmer;
    map<string,int> F;
    map<string,double> S;

    vector<string> a(100001);
    for(int i=0;i<M;i++)
        getline(inFile,a[i]);

    Count_F=M*(a[0].size()-k+1);
    len=a[0].size()-k+1;
    //len=a[0].size()-k;
    //Phase 1
    double start,end,duration;

    //#pragma omp parallel for num_threads(thread),private(kmer)
    for(int i=0;i<M;i++){
        for(int j=0;j<len;j++)
        {
            kmer=a[i].substr(j,k);
           //#pragma omp critical
            {
                ++F[kmer];
            }
        }
    }
    //Phase 2 计算各kmer所占比重
    auto iter=F.begin();
    while(iter!=F.end())
    {
        S[iter->first]=(double)iter->second/Count_F;
        ++iter;
    }

    start=omp_get_wtime();
    //Phase 3
    double Max_Weight=0.,Line_Weight;
    vector<double> MW;            MW.assign(thread, 0.0);
    //vector<int>ID;                ID.assign(thread, 0);
    int blockSize = M/thread;
    #pragma omp parallel num_threads(thread)
    {
        int id=omp_get_thread_num();
        double tempValue;
        for (int i = id * blockSize; i < (id + 1) * blockSize; i++){
            tempValue=0;
            for(int j = 0;j < len; j++) tempValue += S[a[i].substr(j,k)];
            if (tempValue > MW[id]) MW[id] = tempValue;
        }
    }
    for(int i = 0; i < thread; i++) if (MW[i] > Max_Weight) Max_Weight = MW[i];
    Max_Weight = Max_Weight / len ;
    //cout<<Max_Weight<<endl;

    end=omp_get_wtime();
    duration=(double)(end-start);
    cout<<"采用omp并行消耗时间:"<<duration<<endl;


    start=omp_get_wtime();
    //Phase 4
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);
    double threshold=0.15;
    std::bitset<8> bits;
    int kk=0,num=0;
    bits.reset();
    vector<int> b;

    while(inFile.peek() != EOF)
    {
        num=0;
        while(inFile.peek() != EOF && num<100000)
            getline(inFile,a[num++]);
        int z=0;
        b.assign(100001,0);
//        Line_Weight=0;
//        getline(inFile,Qscore);
//        len=Qscore.size();

        #pragma omp parallel for num_threads(thread),private(kmer,Line_Weight,Qscore)
        for(int i=0;i<num;i++)
        {
            Line_Weight=0;
            Qscore=a[i];
            for(int j=0;j<len;j++)
            {
                kmer=Qscore.substr(j,k);
                Line_Weight+=S[kmer];
            }
            Line_Weight=Line_Weight/(double)len;
            if(Line_Weight/Max_Weight>= threshold)
            {
                b[i]=1;
//                while(z<i) ;
//                #pragma omp critical
//                {
//                    outdata1<<Qscore<<'\n';
//                    bits<<= 1;
//                    ++kk;++z;
//                    bits[0]=1;
//                    if(kk==8)
//                    {
//                        outpar.write(reinterpret_cast<const char*>(&bits), 1);
//                        kk=0;bits.reset();
//                    }
//                }
            }
//            else
//            {
//                while(z<i) ;
//                #pragma omp critical
//                {
//                    bits<<= 1;
//                    ++kk;++z;
//                    outdata2<<Qscore<<'\n';
//                    if(kk==8)
//                    {
//                        outpar.write(reinterpret_cast<const char*>(&bits), 1);
//                        kk=0;bits.reset();
//                    }
//                }
//            }
        }
        for(int i=0;i<num;i++)
        {
            bits<<=1;
            bits[0]=b[i];
            if(b[i])
                outdata1<<a[i]<<'\n';
            else
                outdata2<<a[i]<<'\n';
            if(i%8==7)
                outpar.write(reinterpret_cast<const char*>(&bits), 1);
        }
    }
    kk=num%8;
    if(kk)
    {
        bits<<=(8-kk);
        outpar.write(reinterpret_cast<const char*>(&bits), 1);
    }

    end=omp_get_wtime();
    duration=(double)(end-start);
    cout<<"采用omp并行消耗时间:"<<duration<<endl;


    inFile.close();
    outdata1.close();
    outdata2.close();
    return ;
}

void old_compress(std::string &inputfile,int thread){
    string out_File = inputfile+".partition";
    string out_partition=out_File+"/partition_dat";
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
    std::ofstream outdata1,outdata2,outpar;
    outdata1.open(out_data_1.c_str(),std::ios::trunc);
    outdata2.open(out_data_2.c_str(),std::ios::trunc);
    outpar.open(out_partition.c_str(),std::ios::trunc|std::ios::binary);
    double Count_F=0;
    int M=100000,len,k=4;
    string Qscore,kmer;
    map<string,int> F;
    map<string,double> S;

    vector<string> a(100001);
    for(int i=0;i<M;i++)
        getline(inFile,a[i]);

    Count_F=M*(a[0].size()-k+1);
    len=a[0].size()-k+1;
    //len=a[0].size()-k;
    //Phase 1
    double start,end,duration;

    //#pragma omp parallel for num_threads(thread),private(kmer)
    for(int i=0;i<M;i++){
        for(int j=0;j<len;j++)
        {
            kmer=a[i].substr(j,k);
           //#pragma omp critical
            {
                ++F[kmer];
            }
        }
    }
    //Phase 2 计算各kmer所占比重
    auto iter=F.begin();
    while(iter!=F.end())
    {
        S[iter->first]=(double)iter->second/Count_F;
        ++iter;
    }

    double Max_Weight=0.,Line_Weight;
    start=omp_get_wtime();
    //Phase 3

    for(int i=0;i<M;i++)
    {
        Line_Weight=0.;
        for(int j=0;j<len;j++)
        {
            kmer=a[i].substr(j,k);
            Line_Weight+=S[kmer];
        }
        Line_Weight=(double)Line_Weight/(double)len;
        if(Line_Weight>=Max_Weight)
            Max_Weight=Line_Weight;
    }
    cout<<Max_Weight<<endl;

    end=omp_get_wtime();
    duration=(double)(end-start);
    cout<<"未采用并行消耗时间:"<<duration<<endl;

//    start=omp_get_wtime();
//    //Phase 3
//    vector<double> MW;            MW.assign(thread, 0.0);
//
//    //vector<int>ID;                ID.assign(thread, 0);
//    int blockSize = M/thread;
//    #pragma omp parallel num_threads(thread)
//    {
//        int id=omp_get_thread_num();
//        double tempValue;
//        for (int i = id * blockSize; i < (id + 1) * blockSize; i++){
//            tempValue=0;
//            for(int j = 0;j < len; j++) tempValue += S[a[i].substr(j,k)];
//            if (tempValue > MW[id]) MW[id] = tempValue;
//        }
//    }
//    for(int i = 0; i < thread; i++) if (MW[i] > Max_Weight) Max_Weight = MW[i];
//    Max_Weight = Max_Weight / len ;
//    cout<<Max_Weight<<endl;
//
//    end=omp_get_wtime();
//    duration=(double)(end-start);
//    cout<<"采用omp并行消耗时间:"<<duration<<endl;


    start=omp_get_wtime();
    //Phase 4
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);
    double threshold=0.15;
    std::bitset<8> bits;
    int kk=0;
    bits.reset();


    while(inFile.peek() != EOF)
    {
        Line_Weight=0;
        getline(inFile,Qscore);
        len=Qscore.size();
        for(int j=0;j<=len-k;j++)
        {
            kmer=Qscore.substr(j,k);
            Line_Weight+=S[kmer];
        }
        Line_Weight=Line_Weight/(double)(len-k+1);
        if(Line_Weight/Max_Weight>= threshold)
        {
            outdata1<<Qscore<<'\n';
            bits<<= 1;++kk;
            bits[0]=1;
        }
        else
        {
            bits<<= 1;++kk;
            outdata2<<Qscore<<'\n';
        }
        if(kk==8)
        {
            outpar.write(reinterpret_cast<const char*>(&bits), 1);
            kk=0;bits.reset();
        }
    }
    if(kk)
    {
        bits<<=(8-kk);
        outpar.write(reinterpret_cast<const char*>(&bits), 1);
    }
    end=omp_get_wtime();
    duration=(double)(end-start);
    cout<<"未采用并行消耗时间:"<<duration<<endl;

    inFile.close();
    outdata1.close();
    outdata2.close();
    return ;
}


void decompress(std::string &inputfile){
    string in_data_1=inputfile+"/data_1.dat.PQVRC_de";
    string in_data_2=inputfile+"/data_2.dat.PQVRC_de";
    string in_partition=inputfile+"/partition_dat";
    string out_File=inputfile.substr(0,inputfile.rfind('.'))+".pqsdc_de_v2";

    std::ifstream indata1,indata2,inpartition;
    indata1.open(in_data_1.c_str(),std::ios::in);
    indata2.open(in_data_2.c_str(),std::ios::in);
    inpartition.open(in_partition.c_str(),std::ios::in|std::ios::binary);

    std::ofstream outputFile;
    outputFile.open(out_File.c_str(),std::ios::trunc);
    string Qscore;
    int c,k,a;
    std::bitset<8> bits;
    int num=0;
    while(inpartition.peek() != EOF )
    {
        inpartition.read(reinterpret_cast<char*>(&bits),1);
        c=bits.to_ulong();
//        cout<<bits<<endl;
//        break;
        k=8;
        while(k--)
        {
            a=(c>>k)%2;
            if(a==1 && indata1.peek()!=EOF)
                getline(indata1,Qscore);
            else if(a==0 && indata2.peek()!=EOF)
                getline(indata2,Qscore);
            else
                break;
            //++num;
//            cout<<Qscore<<endl;
//            goto next;
            outputFile<<Qscore<<'\n';
        }

    }
    //next:;
    //cout<<num<<endl;
    indata1.close();
    indata2.close();
    inpartition.close();
    outputFile.close();

    return ;
}