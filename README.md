# PQSDC 
![made-with-C++](https://img.shields.io/badge/Made%20with-C++11-brightgreen)
![made-with-OpenMP](https://img.shields.io/badge/Made%20with-OpenMP-blue)

<!-- LOGO -->
<br />
<h1>
<p align="center">
  <img src="https://github.com/fahaihi/PQSDC/blob/master/PQSDC_LOG.png" alt="Logo" width="892" height="207">
</h1>
  <p align="center">
    A Lossless Parallel Quality Scores Data Compressor For Large Scale Genomic Sequencing Data.
    </p>
</p>
<p align="center">
  <a href="#about-the-pmffrc">About The PQSDC</a> •
  <a href="#copy-our-project">Copy Our Project</a> •
  <a href="#useage">Useage</a> •
  <a href="#example">Example</a> •
  <a href="#our-experimental-configuration">Our Experimental Configuration</a> •
    <a href="#dataset-acquisition">Dataset Acquisition</a> •
  <a href="#aknowledgements">Acknowledgements</a> •
</p>  

<p align="center">
  
![screenshot](img/clip.gif)
</p>                                                                                                                             
                                                                                                                                                      
## About The PQSDC 
PQSDC is an experimental open-source compressor, which utilizes parallel sequence partitioning and a four-level run-length prediction model to increase compression ratio while minimizing memory and time consumption. Furthermore, the compression process can be accelerated through the use of multi-core CPU clusters, resulting in a significant reduction of time overhead.

PQSDC (Parallel Quality Scores Data Compressor).

## Copy Our Project

Firstly, clone our tools from GitHub:
```shell script
git clone https://github.com/fahaihi/PQSDC.git
```
Secondly, turn to PQSDC directory：
```shell script
cd PQSDC/pqsdc_v2
```
Thirdly, Run the following command：
```shell script
bash install.sh
#Warning!:GNU Make > 3.82.
```
Finally, Configure the environment variables with the following command:
```shell script
export PATH=$PATH:`pwd`/
export PQSDC_V2_PATH="`pwd`/"
source ~/.bashrc
```
## Usage
```sh
    Basic Useage: pqsdc_v2 [command option]
       -c [qualities file] [threads]                      *compression mode.
       -d [pqsdc generate directory] [threads]            *decompression mode.
       -h                                                 *print this message.
    Advanced Usage:pqsdc_tools [command option]
       -fileinfo [input-fastq-file]                       *print basic statistic information.
       -dirinfo [input-dir-name]                          *print basic statistic information.
       -verify [source-fastq-file] <mode> [verify-file]   *verify decompression.
          <mode> = reads
          <mode> = qualities
       -filesplite [input-fastq-file] mode <mode>         *splite a FastQ file according <mode>.
          <mode> = ids
          <mode> = reads
          <mode> = describes
          <mode> = qualities
          <mode> = all
```
Notes: In order to be compatible with any personal computer, the current version only open-sources the method of parallel compression on a single CPU node with multiple cores. 
The BIOCONDA version will be updated soon...


## Examples
We present the validation dataset `PQSDC/data/test.qualities` 
#### 1、Using 8 CPU cores for compression.
```sh
cd ${PQSDC_V2_PATH}data
pqsdc_v2 -c test.qualities 8
```
results:
```sh
compression mode.
fileName : test.qualities
threads  : 8
savepath : test.qualities.partition/result.pqsdc_v2
----------------------------------------------------------------------
1 reads partition, generate test.qualities.partition directory.
2 parallel run-length encoding prediction mapping.
3 cascade zpaq compressor.
4 pacing files into test.qualities.partition/result.pqsdc_v2.
5 removing redundant files.
over!
----------------------------------------------------------------------
```
#### 2、Using 8 CPU cores for decompression.
```sh
pqsdc_v2 -d test.qualities.partition 8
```
results:
```sh
running pqsdc algorithm at Sat Jun 17 15:31:22 CST 2023
de-compression mode
fileName : test.qualities.partition
threads  : 8
savepath : test.qualities.partition.partition.pqsdc_v2
----------------------------------------------------------------------
1 unpacking test.qualities.partition/result.pqsdc_v2.
2 unsing zpaq decompression files.
3 parallel run-length encoding prediction mapping.
4 merge partitions to restore the original file
over
----------------------------------------------------------------------
```
#### 3、Verify if the decompression is successful.
```sh
pqsdc_tools -verify test.fastq qualities test.qualities.pqsdc_de_v2
```
results:
```sh
lossless recover all qualities.
```

## Our Experimental Configuration
Our experiment was conducted on the SUGON-7000A supercomputer system at the Nanning Branch of the National Supercomputing Center, using a queue of CPU/GPU heterogeneous computing nodes. The compute nodes used in the experiment were configured as follows: 
  
  2\*Intel Xeon Gold 6230 CPU (2.1Ghz, total 40 cores), 
  
  2\*NVIDIA Tesla-T4 GPU (16GB CUDA memory, 2560 CUDA cores), 
  
  512GB DDR4 memory, and 
  
  8\*900GB external storage.

## Dataset Acquisition
We experimentally evaluated using the real publicly available sequencing datasets from the NCBI database.
download this dataset by the following command:
```sh
nohup bash data_download.sh > data_download.log &
```
Dataset download and extraction using the `SRA-Tools：https://github.com/ncbi/sra-tools tool`.

## Acknowledgements
- Thanks to [@HPC-GXU](https://hpc.gxu.edu.cn) for the computing device support.   
- Thanks to [@NCBI](https://www.freelancer.com/u/Ostokhoon) for all available datasets.

## Additional Information
**Source-Version：**    V1.2023.05.18.

**Latest-Version：**    V2.1.2023.06.17.

**Authors:**     NBJL-BioGrop.

**Contact us:**  https://nbjl.nankai.edu.cn OR sunh@nbjl.naikai.edu.cn
