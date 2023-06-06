CXX=g++
CPPFLAGS+=-Dunix
CXXFLAGS+=-O3
CXXFLAGS+=-std=c++11



all: lcqs

libzpaq.o: libzpaq.cpp libzpaq.h
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -o $@ -c libzpaq.cpp -static -static-libstdc++ -L/usr/lib/x86_64-redhat-linux6E/lib64 -lm -lpthread -lc

lcqs.o: lcqs.cpp lcqs.h libzpaq.h
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -w -o $@ -c lcqs.cpp -pthread -static -static-libstdc++ -L/usr/lib/x86_64-redhat-linux6E/lib64 -lm -lpthread -lc

main.o: main.cpp lcqs.h
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -o $@ -c main.cpp -pthread -static -static-libstdc++ -L/usr/lib/x86_64-redhat-linux6E/lib64 -lm -lpthread -lc

lcqs: main.o lcqs.o libzpaq.o
	$(CXX) $(LDFLAGS) -o $@ main.o lcqs.o libzpaq.o -pthread -static -static-libstdc++ -L/usr/lib/x86_64-redhat-linux6E/lib64 -lm -lpthread -lc

clean:
	rm -f main.o lcqs.o libzpaq.o lcqs

