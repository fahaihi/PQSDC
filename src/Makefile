CC = icpx
CFLAGS = -std=c++11 -Wall -Werror -Wno-unused-variable -O3
SRC = main.cpp tools.cpp message.cpp
OBJ = $(SRC:.cpp=.o)
TARGET = PQSDC

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $(TARGET)

%.o: %.cpp
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)
