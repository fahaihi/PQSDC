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
#include<bitset>
#include<queue>
#include <omp.h>
#include<time.h>
using namespace std;
double get_data(std::string &Qscore);
void compress(std::string &inputfile,int thread);
void old_compress(std::string &inputfile);
void decompress(std::string &inputfile);
string lcqs(std::string &Qscore);
string rle_lcqs(std::string &Qscore);
string de_lcqs(std::string &instring,int lens);
string de_rle_lcqs(std::string &instring,int lens);
int main(int argc, char** argv){
    string A = std::string(argv[3]);
    //string A = "/public/home/jd_sunhui/genCompressor/Data/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12/110902_I244_FCC02FUACXX_L4_006SCELL03AEAAAPEI-12_2.fq";
    //string A = "/public/home/jd_sunhui/genCompressor/Data/NCBI/PhiX/PhiX_12.fastq";
    string method=argv[1];
    int thread=std::stoi(argv[2]);
    double start,end,duration;
    if(method=="-c")
    {
//        start=omp_get_wtime();
//        old_compress(A);
//        end=omp_get_wtime();
//        duration=(double)(end-start);
//        cout<<"未采用omp并行消耗时间:"<<duration<<endl;

        start=omp_get_wtime();
        compress(A,thread);
        end=omp_get_wtime();
        duration=(double)(end-start);
        cout<<"采用omp并行消耗时间:"<<duration<<endl;
    }

    else if(method=="-d")
        decompress(A);
    //cout<<method<<"  "<<method.size()<<endl;
    //cout<<A<<endl;
    return 0;
}

void compress(std::string &inputfile,int thread){
    string out_data = inputfile+".PQVRC";
    std::ifstream inFile;
    inFile.open(inputfile.c_str(), std::ios::in);
    if (!inFile) throw("Source_File_Wrong!");
    std::ofstream out_char_File;
    out_char_File.open(out_data.c_str(),std::ios::trunc|std::ios::binary);

    string Qscore;
    int C,pre;//参考数C和用于找连续串的pre
    int len=0,begin,n,c,k;
    string outstring,s,out;
    queue<int> rbegin,rnum;
    vector<int> ct(128,0);//统计找众数
    int num1=0,num2=0,num3=0,num=0,qs_num;
    double regression;
    getline(inFile,Qscore);
    len=Qscore.size();
    out_char_File<<len<<'\n';
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);

    //return ;
    vector<string> a(10001),outs(10001);
    double threshold=0.34;
    //string outs;
    while (inFile.peek() != EOF){
        //读取10000条拼接序列
        qs_num=0;
        //cout<<1<<endl;
        //拼接成一条
        while(inFile.peek() != EOF && qs_num<10000)
        {
            a[qs_num]="";
            num=0;
            while(inFile.peek() != EOF && num<100)
            {
                getline(inFile,s);
                a[qs_num]+=s;
                ++num;
            }
            outs[qs_num]=a[qs_num];
            ++qs_num;
        }

        //cout<<2<<endl;

//        int blockSize = (qs_num+thread-1)/thread;
//        #pragma omp parallel num_threads(thread)
//        {
//            int id=omp_get_thread_num();
//            double regression;
//            int begin=id * blockSize,end=min((id + 1) * blockSize,qs_num);
//            for (int i = begin; i < end; i++){
//                regression=get_data(a[i]);
//                if(regression>threshold)
//                {
//                    outs[i]=rle_lcqs(Qscore);
//                    #pragma omp critical
//                    {
//                        num2++;
//                    }
//                }
//                else
//                {
//                    outs[i]=lcqs(Qscore);
//                    #pragma omp critical
//                    {
//                        num1++;
//                    }
//                }
//            }
//        }

        //cout<<qs_num<<endl;
        #pragma omp parallel for num_threads(thread),private(Qscore,regression)
        for(int i=0;i<qs_num;i++)
        {
            Qscore=a[i];
            regression=get_data(Qscore);
            //cout<<regression;
            // break;
            if(regression>threshold)
            {
                outs[i]=rle_lcqs(Qscore);
                #pragma omp critical
                {
                    num2++;
                }
            }
            else
            {
                outs[i]=lcqs(Qscore);
                #pragma omp critical
                {
                    num1++;
                }
            }
        }
        //cout<<3<<endl;
        //cout<<outs[0]<<endl;
        for(int i=0;i<qs_num;i++)
        {
            out=outs[i];
            int len1=out.size();
            std::bitset<8> bits;
            for(unsigned char b:out)
            {
                bits = std::bitset<8>(b);
                out_char_File.write(reinterpret_cast<char*>(&bits), 1);
            }
            bits=std::bitset<8> (0);
            out_char_File.write(reinterpret_cast<char*>(&bits), 1);
        }

    }
    cout<<num1<<endl<<num2<<endl;
    inFile.close();
    out_char_File.close();

    return ;
}

void old_compress(std::string &inputfile){
    string out_data = inputfile+".Old_PQVRC";
    std::ifstream inFile;
    inFile.open(inputfile.c_str(), std::ios::in);
    if (!inFile) throw("Source_File_Wrong!");
    std::ofstream out_char_File;
    out_char_File.open(out_data.c_str(),std::ios::trunc|std::ios::binary);

    string Qscore;
    int C,pre;//参考数C和用于找连续串的pre
    int len=0,begin,n,c,k;
    string outstring,s;
    queue<int> rbegin,rnum;
    vector<int> ct(128,0);//统计找众数
    int num1=0,num2=0,num3=0,num=0,qs_num;
    double regression;
    getline(inFile,Qscore);
    len=Qscore.size();
    out_char_File<<len<<'\n';
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);

    //return ;
    vector<string> a[10000];
    double threshold=0.19;
    string outs;
    while (inFile.peek() != EOF){
        getline(inFile,Qscore);
        //cout<<Qscore.size()<<endl;
        num=1;
        while(inFile.peek() != EOF && num<100)
        {
            getline(inFile,s);
            //cout<<s.size()<<endl;
            Qscore+=s;
            ++num;
        }

        regression=get_data(Qscore);
        //cout<<regression;
       // break;
        if(regression>threshold)
           {
                outs=rle_lcqs(Qscore);
                ++num2;
           }
        else
         {
            outs=lcqs(Qscore);
            ++num1;
         }
        const int len1=outs.size();
        std::bitset<8> bits;
        for(unsigned char b:outs)
        {
            bits = std::bitset<8>(b);
            out_char_File.write(reinterpret_cast<char*>(&bits), 1);
        }
        bits=std::bitset<8> (0);
        out_char_File.write(reinterpret_cast<char*>(&bits), 1);
    }
        cout<<num1<<endl<<num2<<endl;
    inFile.close();
    out_char_File.close();



    return ;
}

double get_data(std::string &Qscore){
    double C_Proportion,Difference_ratio=0;
    int RLE_num=0,score_num=0,num=0,begin=0;
    int n=Qscore.size(),C=0,c;
    vector<int> ct(128,0);//统计找众数
    for(int i=0;i<n;i++)
        {
            if(i>0&&abs(Qscore[i]-Qscore[i-1])>7)
                ++Difference_ratio;
            c=Qscore[i];
            ct[c]++;
            if(ct[c]>ct[C])
                C=c;
            else if(ct[c]==ct[C] && c>C)
                C=c;
            if(Qscore[begin]==c)
                ++num;
            else
            {
                if(num>3) ++RLE_num;
                begin=i;
                num=1;
            }
            //pre=c;
        }
    C_Proportion=(double)ct[C]/n;
    Difference_ratio=(double)Difference_ratio/(n-1);
    for(int num : ct)
    {
        if(num)
            ++score_num;
    }

    //cout<<C_Proportion<<','<<RLE_num<<','<<score_num<<','<<Difference_ratio<<endl;
    double ret=0.944556357242;
    double b[5],parameter[5];
    parameter[1]=C_Proportion;parameter[2]=RLE_num;parameter[3]=score_num;parameter[4]=Difference_ratio;
    b[0]=0.944556357242;b[1]=-2.127772179987909;b[2]= 1.0646078294906305;b[3]= 1.1982964414505566;b[4]= -1.7283286626425758;

    #pragma omp simd reduction(+:ret)
    for(int i=1;i<5;i++)
        ret+=parameter[i]*b[i];



//    // 获取程序启动时的 CPU 时间
//    start_time = clock();
//    ret=7.75981888762;
    //ret+=b[1]*C_Proportion+b[2]*RLE_num+b[3]*score_num+b[4]*Difference_ratio;  ///SIMD并行
//    // 获取当前时刻的 CPU 时间
//    end_time = clock();
//    // 计算代码执行时间，单位为秒
//    exec_time = static_cast<double>(end_time - start_time) / CLOCKS_PER_SEC;
//    // 输出代码执行时间
//    std::cout << "代码执行时间为 " << exec_time*1000000 << " 秒" << std::endl;

    return ret;
}

void decompress(std::string &inputfile){
    string in_data = inputfile;
    string out_File=inputfile.substr(0,inputfile.rfind('.'))+".PQVRC_de";
    std::ifstream in_data_File;
    in_data_File.open(in_data.c_str(), std::ios::in|std::ios::binary);
    std::ofstream outputFile;
    outputFile.open(out_File.c_str(),std::ios::trunc);
    string Qscore,instring;
    getline(in_data_File,instring);
    int lens =atoi(instring.c_str());//>0 ? instring[0] : instring[0]+256;
    //cout<<lens;
    instring="";
    std::bitset<8> bits;
    int count=0,C=0,len=0,k=7,c;
    while(in_data_File.peek() != EOF)
    {
        in_data_File.read(reinterpret_cast<char*>(&bits),1);
        c=bits.to_ulong();
        if(c)
            instring+=c;
        else if(c==0)
        {
            if(instring[0]>0&&instring[0]<=128)
                Qscore=de_lcqs(instring,lens);
            else
                Qscore=de_rle_lcqs(instring,lens);
            outputFile<<Qscore;
            instring="";
        }
        //getline(in_data_File,instring);


    }
    in_data_File.close();
    outputFile.close();

    return ;
}

string lcqs(std::string &Qscore){
    int C,pre;//参考数C和用于找连续串的pre
    int len=0,num=0,begin,n,c,k,j;
    vector<int> ct(128,0);//统计找众数
    string outstring="";
    pre=0;begin=0;n=Qscore.size();num=0;C=0;
    //统计众数C并且统计RLE
        for(int i=0;i<n;i++)
        {
            c=Qscore[i];
            ct[c]++;
            if(ct[c]>=ct[C])
                C=c;
        }
        outstring+=C;
        for(int i=0;i<n;i++)
        {
            c=Qscore[i];
            if(c==C)
            {
                j=i+1;
                while(j<n && Qscore[j]==C)
                {
                    ++j;
                }
                while(j-i>54)
                {   outstring+=255;i+=54;}
                if(j-i>0)
                    outstring+=201+(j-i);
                i=j-1;continue;
            }

            if(i<n-2 && (Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C) && (Qscore[i+2]>=C-3 && Qscore[i+2]<=C))
            {
                outstring+=(Qscore[i]-C+3)*16+(Qscore[i+1]-C+3)*4+(Qscore[i+2]-C+3)+137;
                i+=2;
            }
            else if(i<n-1  && (Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C))
            {
                outstring+=(Qscore[i]-C+7)*8+(Qscore[i+1]-C+7)+73;
                i+=1;
            }
            else
                outstring+=Qscore[i]-32;
        }
        return outstring;
}


string rle_lcqs(std::string &Qscore){
    int C,pre;//参考数C和用于找连续串的pre
    int len=0,num=0,begin,n,c,k;
    string outstring;
    queue<int> rbegin,rnum;
    vector<int> ct(128,0);//统计找众数
    //std::fill(ct.begin(),ct.end(),0);
    outstring="";
    pre=0;begin=0;n=Qscore.size();num=0;C=0;
    for(int i=0;i<n;i++)
        {
            c=Qscore[i];
            ct[c]++;
            if(ct[c]>ct[C])
                C=c;
            else if(ct[c]==ct[C] && c>C)
                C=c;
            if(Qscore[begin]==c)
                ++num;
            else
            {
                if(num>3)
                {
                    rbegin.push(begin);
                    rnum.push(num);
                    //cout<< pre<<" "<<begin<<" "<<num<<" ";
                }
                begin=i;
                num=1;

            }
            //pre=c;
        }
    if(num>3)
            {
                rbegin.push(begin);
                rnum.push(num);
            }
    outstring+=C+128;
    //判别是否有连续子串
    if(!rbegin.empty())
    {
        begin=rbegin.front();rbegin.pop();
        num=rnum.front();rnum.pop();
    }
    else
        begin=256;
    for(int i=0;i<n;i++)
    {
        c=Qscore[i];
        if(i==begin)
        {
        //cout<<num<<" ";
            if(num>25)
                    k=(num-1)/25+1;
            else k=1;
            if(c==C)
            {
                i+=num-1;
                while(k)
                {
                    outstring+=(num/k)+230;
                    num-=num/k;
                    k--;
                }
                outstring+=c;
                if(!rbegin.empty())
                {
                    begin=rbegin.front();rbegin.pop();
                    num=rnum.front();rnum.pop();
                }
                else
                    begin=256;
                continue;
            }
            else if(c>=C-3 && c<C && num>6)
            {
                i+=num-1;
                while(k)
                {
                    outstring+=(num/k)+230;
                    num-=num/k;
                    k--;
                }
                outstring+=c;
                if(!rbegin.empty())
                {
                    begin=rbegin.front();rbegin.pop();
                    num=rnum.front();rnum.pop();
                }
                else
                    begin=256;
                continue;
            }
            else if(c>=C-8 && c<C-3 &&num>9)
            {
                i+=num-1;
                while(k)
                {
                    outstring+=(num/k)+230;
                    num-=num/k;
                    k--;
                }
                outstring+=c;
                if(!rbegin.empty())
                {
                    begin=rbegin.front();rbegin.pop();
                    num=rnum.front();rnum.pop();
                }
                else
                    begin=255;
                continue;
            }
            else if(c<C-8 || c>=C)
            {
                i+=num-1;
                while(k)
                {
                    outstring+=(num/k)+230;
                    num-=num/k;
                    k--;
                }
                outstring+=c;
                if(!rbegin.empty())
                {
                    begin=rbegin.front();rbegin.pop();
                    num=rnum.front();rnum.pop();
                }
                else
                    begin=255;
                continue;
            }
            if(!rbegin.empty())
                {
                    begin=rbegin.front();rbegin.pop();
                    num=rnum.front();rnum.pop();
                }
            else
                begin=255;
        }
        if(i<n-2  && i<begin-2 && (Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C) && (Qscore[i+2]>=C-3 && Qscore[i+2]<=C))
        {
            outstring+=(Qscore[i]-C+3)*16+(Qscore[i+1]-C+3)*4+(Qscore[i+2]-C+3)+169;
            i+=2;
        }
        else if(i<n-1  && i<begin-1 && (Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C))
        {
            outstring+=(Qscore[i]-C+7)*8+(Qscore[i+1]-C+7)+105;
            i+=1;
        }
        else
            outstring+=Qscore[i];
    }
    return outstring;
}

string de_lcqs(std::string &instring,int lens)
{
    string Qscore="";
    int len = instring.size(),C,c,num,k=0;
    C=instring[0];
    //cout<<C<<" ";
    for(int i=1;i<len;i++)
    {
        c=instring[i];
        if(c<0)
            c+=256;
        if(c>200)
        {
            num=c-201;
            while(num--){
                Qscore+=C;++k;
                if(k==lens) {Qscore+='\n';k=0;}
            }

        }
        else if(c>136)
        {
            Qscore+=(c-137)/16+C-3;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=((c-137)%16)/4+C-3;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=(c-137)%4+(C-3);++k;if(k==lens) {Qscore+='\n';k=0;}

        }
        else if(c>72)
        {
            Qscore+=(c-73)/8+C-7;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=(c-73)%8+C-7;++k;if(k==lens) {Qscore+='\n';k=0;}

        }
        else
        {
            Qscore+=c+32;++k;if(k==lens) {Qscore+='\n';k=0;}
        }
    }
    return Qscore;
}

string de_rle_lcqs(std::string &instring,int lens){
    string Qscore="";
    int len = instring.size(),C,c,num,k=0;
    Qscore="";
    C=instring[0]+128;
    for(int i=1;i<len;i++)
    {
        c=instring[i];
        if(c<0)
            c+=256;
        if(c>232)
        {
            num=0;
            while(c>232)
            {
                num+=c-230;
                c=instring[++i];
                if(c<0)
                    c+=256;
            }
            //if(c<0) c+=256;
            while(num--)
            {
                Qscore+=c;
                ++k;if(k==lens) {Qscore+='\n';k=0;}
            }

        }
        else
        {
            if(c>168)
            {
                Qscore+=(c-169)/16+C-3;++k;if(k==lens){Qscore+='\n';k=0;}
                Qscore+=((c-169)%16)/4+C-3;++k;if(k==lens) {Qscore+='\n';k=0;}
                Qscore+=(c-169)%4+(C-3);++k;if(k==lens) {Qscore+='\n';k=0;}
            }
            else if(c>104)
            {
                Qscore+=(c-105)/8+C-7;++k;if(k==lens) {Qscore+='\n';k=0;}
                Qscore+=(c-105)%8+C-7;++k;if(k==lens) {Qscore+='\n';k=0;}
            }
            else
            {
                Qscore+=c;++k;if(k==lens) {Qscore+='\n';k=0;}
            }
        }
    }
    return Qscore;
}