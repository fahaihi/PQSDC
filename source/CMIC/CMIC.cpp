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
using namespace std;
void compress(std::string &inputfile);
void decompress(std::string &inputfile);
string lcqs(std::string &Qscore);
string rle_lcqs(std::string &Qscore);
string de_lcqs(std::string &instring,int lens);
vector<long> ans;
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
    string out_data = inputfile+".CMIC";
    std::ifstream inFile;
    inFile.open(inputfile.c_str(), std::ios::in);
    if (!inFile) throw("Source_File_Wrong!");
    std::ofstream out_char_File;
    out_char_File.open(out_data.c_str(),std::ios::trunc|std::ios::binary);

    string Qscore;
    int C,pre;//参考数C和用于找连续串的pre
    int len=0,begin,n,c,k,j;
    string outstring,s;
    queue<int> rbegin,rnum;
    vector<int> ct(128,0);//统计找众数
    int num1=0,num2=0,num3=0,num=0;
    std::bitset<72> a;
    std::bitset<72> bits;
    getline(inFile,Qscore);
    len=Qscore.size();
    out_char_File<<len<<'\n';
    inFile.close();
    inFile.open(inputfile.c_str(), std::ios::in);
    int llen=(len*100+4)/8;

    int z,pre_k;
    while (inFile.peek() != EOF){
        getline(inFile,Qscore);
        num=1;k=0;
        a.reset();
        while(num<100)
        {
            getline(inFile,s);
            Qscore+=s;
            ++num;
        }
        //string outs1,outs2,outs;
        //outs=lcqs(Qscore);
        n=Qscore.size();
        //统计众数C并且统计RLE
        std::fill(ct.begin(),ct.end(),0);
        for(int i=0;i<n;i++)
        {
            c=Qscore[i];
            ct[c]++;
            if(ct[c]>=ct[C])
                C=c;
        }
        bits = std::bitset<72>(C);a=(a<<9)|bits; ++k;
        //outstring+=C;//存储众数
        //cout<<C<<endl;
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
                //映射201-254
                while(j-i>54)
                {   bits = std::bitset<72>(254);i+=54;
                a=(a<<9)|bits; ++k;
                    if(k==8)
                    {
                        out_char_File.write(reinterpret_cast<const char*>(&a), 9);
                        k=0;
                        a.reset();
                    }
                    //cout<<a<<endl;

                }
                if(j-i>0)
                {
                    bits = std::bitset<72>(200+j-i);a=(a<<9)|bits; ++k;
                    if(k==8)
                    {
                        out_char_File.write(reinterpret_cast<const char*>(&a), 9);
                        k=0;
                        a.reset();
                    }
                    //cout<<a<<endl;
                }
                i=j-1;continue;
            }

            pre_k=k;
            if(i<n-1 && c>=(C-3) && c<=(C+3))
            {
            //cout<<c<<" ";
                if(i<n-2 && (Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3) && (Qscore[i+2]>=C && Qscore[i+2]<=C+3))//319-382
                { bits = std::bitset<72>(((C+3)-Qscore[i])*16+((C+3)-Qscore[i+1])*4+((C+3)-Qscore[i+2])+319);a=(a<<9)|bits; ++k;i+=2;}
                else if(i<n-2 && (Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C) && (Qscore[i+2]>=C-3 && Qscore[i+2]<=C))//137-200
                { bits = std::bitset<72>((Qscore[i]-(C-3))*16+(Qscore[i+1]-(C-3))*4+(Qscore[i+2]-(C-3))+137);a=(a<<9)|bits; ++k;i+=2;}
                else if((Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3))//255-318
                { bits = std::bitset<72>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;++i;}
                else if((Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C))//73-136
                { bits = std::bitset<72>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;++i;}
                else if(((Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+7)) || ((Qscore[i]>=C && Qscore[i]<=C+7) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3)))//255-318
                { bits = std::bitset<72>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;++i;}
                else if(((Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C)) || ((Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C)))//73-136
                { bits = std::bitset<72>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;++i;}
                if(k==8)
                    {
                        out_char_File.write(reinterpret_cast<const char*>(&a), 9);
                        k=0;
                        a.reset();
                    }
            }
            if(i<n-1 && c>=(C-7) && c<=(C+7)&& pre_k==k)
            {
                if((Qscore[i]>=C && Qscore[i]<=C+7) && (Qscore[i+1]>=C && Qscore[i+1]<=C+7))//255-318
                { bits = std::bitset<72>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;++i;}
                else if((Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C))//73-136
                { bits = std::bitset<72>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;++i;}
                if(k==8)
                    {
                        out_char_File.write(reinterpret_cast<const char*>(&a), 9);
                        k=0;
                        a.reset();
                    }
            }
            if(pre_k==k)
            {
                bits = std::bitset<72>(c-32);a=(a<<9)|bits; ++k;
                if(k==8)
                    {
                        out_char_File.write(reinterpret_cast<const char*>(&a), 9);
                        k=0;
                        a.reset();
                    }
            }
//            cout<<a<<endl;
//            cout<<i<<endl;


        }
        if(k)
        {
              a<<=(9*(8-k));
              out_char_File.write(reinterpret_cast<const char*>(&a), 9);
              k=0;
              a.reset();
        }
        //out_char_File<<'\n';

//        std::bitset<8> bits;
////        std::bitset<8 * len1> result;
//        for(unsigned char b:outs)
//        {
//            bits = std::bitset<8>(b);
////            result <<= 8;
////            result |= bits;
//            out_char_File.write(reinterpret_cast<char*>(&bits), 1);
//        }

        //outs = outs1.size() <outs2.size() ? outs1 : outs2;
        //out_char_File<<result<<'\n';
    }
            //cout<<num1<<endl<<num2<<endl<<num3<<endl;
    next:;
    inFile.close();
    out_char_File.close();
    return ;
}

void decompress(std::string &inputfile){
    string in_data = inputfile;
    string out_File=inputfile.substr(0,inputfile.rfind('.'))+".CMIC_de";
    std::ifstream in_data_File;
    in_data_File.open(in_data.c_str(), std::ios::in|std::ios::binary);
    std::ofstream outputFile;
    outputFile.open(out_File.c_str(),std::ios::trunc);
    string Qscore="",instring;
    getline(in_data_File,instring);
    int lens =atoi(instring.c_str());//>0 ? instring[0] : instring[0]+256;
    //cout<<lens;
    std::bitset<72> a;
    int count=0,C=0,len=0,k=7,c;
    int num;
    while(in_data_File.peek() != EOF)
    {
        in_data_File.read(reinterpret_cast<char*>(&a),9);

        //getline(in_data_File,instring);
        k=7;
        if(count==0&&len==0)
        {
            for(int i=k*9+8;i>=k*9;i--)
                C=C*2+a[i];
            //C=((int)(a>>(9*k)))&511;
            k--;
        }
        while(k>=0)
        {
        //cout<<count<<'\n';
            c=0;
            for(int i=k*9+8;i>=k*9;i--)
                c=c*2+a[i];
            //cout<<c<<endl;
            if(c==0)break;
            k--;
            if(c>200&&c<255)
            {
                num=c-200;
                while(num--){
                    Qscore+=C;++len;
                    if(len==lens) {Qscore+='\n';++count;len=0;}
                }

            }
            else if(c>318)
            {
                Qscore+=C+3-(c-319)/16;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=C+3-((c-319)%16)/4;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=C+3-(c-319)%4;;++len;if(len==lens){Qscore+='\n';++count;len=0;}

            }
            else if(c>254)
            {
                Qscore+=C+7-(c-255)/8;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=C+7-(c-255)%8;++len;if(len==lens) {Qscore+='\n';++count;len=0;}

            }
            else if(c>136)
            {
                Qscore+=(c-137)/16+C-3;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=(c-137)%16/4+C-3;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=(c-137)%4+C-3;;++len;if(len==lens){Qscore+='\n';++count;len=0;}

            }
            else if(c>72)
            {
                Qscore+=(c-73)/8+C-7;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
                Qscore+=(c-73)%8+C-7;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
            }
            else
            {
                Qscore+=c+32;++len;if(len==lens) {Qscore+='\n';++count;len=0;}
            }
        }
        if(count==100)
        {
            outputFile<<Qscore;
            count=0;
            Qscore="";
        }
//        if(instring[0]>0)
//            Qscore=de_lcqs(instring,lens);
        //break;
    }
    //cout<<count<<'\n';
    if(count)
        outputFile<<Qscore;
    in_data_File.close();
    outputFile.close();

    return ;
}

//string lcqs(std::string &Qscore){
//    int C,pre;//参考数C和用于找连续串的pre
//    int len=0,num=0,begin,n,c,k,j;
//    vector<int> ct(128,0);//统计找众数
//    string outstring="";
//    pre=0;begin=0;n=Qscore.size();num=0;C=0;
//    std::bitset<72> a;
//    std::bitset<9> bits;
//    k=0;
//    //统计众数C并且统计RLE
//        for(int i=0;i<n;i++)
//        {
//            c=Qscore[i];
//            ct[c]++;
//            if(ct[c]>=ct[C])
//                C=c;
//        }
//        bits = std::bitset<9>(C);a=(a<<9)|bits; ++k;
//        //outstring+=C;//存储众数
//        for(int i=0;i<n;i++)
//        {
//            c=Qscore[i];
//            if(c==C)
//            {
//                j=i+1;
//                while(j<n && Qscore[j]==C)
//                {
//                    ++j;
//                }
//                while(j-i>54)
//                {   bits = std::bitset<9>(255);i+=54;
//                a=(a<<9)|bits; ++k;
//                    if(k==5)
//                    {
//                        out_char_File.write(reinterpret_cast<char*>(&a), 9);
//                        k=0;
//                        a.reset();
//                    }
//                }
//                if(j-i>0)
//                {
//                    bits = std::bitset<9>(201+j-i);a=(a<<9)|bits; ++k;
//                    if(k==5)
//                    {
//                        out_char_File.write(reinterpret_cast<char*>(&a), 9);
//                        k=0;
//                        a.reset();
//                    }
//                }
//                i=j-1;continue;
//            }
//
//            if(i<n-1 && c>=(C-3) && c<=(C+3))
//            {
//                if((Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3))
//                { bits = std::bitset<9>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;}
//                else if((Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C))
//                { bits = std::bitset<9>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;}
//                else if(i<n-2 && (Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3) && (Qscore[i+2]>=C && Qscore[i+2]<=C+3))
//                { bits = std::bitset<9>(((C+3)-Qscore[i])*16+((C+3)-Qscore[i+1])*4+((C+3)-Qscore[i+2])+319);a=(a<<9)|bits; ++k;}
//                else if((Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C) && (Qscore[i+2]>=C-3 && Qscore[i+2]<=C))
//                { bits = std::bitset<9>((Qscore[i]-(C-3))*16+(Qscore[i+1]-(C-3))*4+(Qscore[i+2]-(C-3))+137);a=(a<<9)|bits; ++k;}
//                else if(((Qscore[i]>=C && Qscore[i]<=C+3) && (Qscore[i+1]>=C && Qscore[i+1]<=C+7)) || ((Qscore[i]>=C && Qscore[i]<=C+7) && (Qscore[i+1]>=C && Qscore[i+1]<=C+3)))
//                { bits = std::bitset<9>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;}
//                else if(((Qscore[i]>=C-3 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C)) || ((Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-3 && Qscore[i+1]<=C)))
//                { bits = std::bitset<9>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;}
//                if(k==5)
//                    {
//                        out_char_File.write(reinterpret_cast<char*>(&a), 9);
//                        k=0;
//                        a.reset();
//                    }
//            }
//            else if(i<n-1 && c>=(C-7) && c<=(C+7))
//            {
//                if((Qscore[i]>=C && Qscore[i]<=C+7) && (Qscore[i+1]>=C && Qscore[i+1]<=C+7))
//                { bits = std::bitset<9>(((C+7)-Qscore[i])*8+((C+7)-Qscore[i+1])+255);a=(a<<9)|bits; ++k;}
//                else if((Qscore[i]>=C-7 && Qscore[i]<=C) && (Qscore[i+1]>=C-7 && Qscore[i+1]<=C))
//                { bits = std::bitset<9>((Qscore[i]-(C-7))*8+(Qscore[i+1]-(C-7))+73);a=(a<<9)|bits; ++k;}
//                if(k==5)
//                    {
//                        out_char_File.write(reinterpret_cast<char*>(&a), 9);
//                        k=0;
//                        a.reset();
//                    }
//            }
//            else
//            {
//                bits = std::bitset<9>(c-32);a=(a<<9)|bits; ++k;
//                if(k==5)
//                    {
//                        out_char_File.write(reinterpret_cast<char*>(&a), 9);
//                        k=0;
//                        a.reset();
//                    }
//            }
//
//        }
//        return outstring;
//}



string de_lcqs(std::string &instring,int lens)
{
    string Qscore="";
    int len = instring.size(),C,c,num,k=0;
    C=instring[0];
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
        else if(c>153)
        {
            Qscore+=(c-154)/16+C-3;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=((c-154)%16)/4+C-3;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=(c-154)%4+(C-3);++k;if(k==lens) {Qscore+='\n';k=0;}

        }
        else if(c>72)
        {
            Qscore+=(c-73)/9+C-8;++k;if(k==lens) {Qscore+='\n';k=0;}
            Qscore+=(c-73)%9+C-8;++k;if(k==lens) {Qscore+='\n';k=0;}

        }
        else
        {
            Qscore+=c+32;++k;if(k==lens) {Qscore+='\n';k=0;}
        }
    }
    return Qscore;
}



