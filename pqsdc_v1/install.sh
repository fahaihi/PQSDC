#!/bin/bash
g++ partition.cpp -std=c++11 -fopenmp -O3 -o partition.out
g++ PQVRC.cpp -std=c++11 -fopenmp -O3 -o pqsdc.out