# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -g -I$(INCLUDE_DIR) -MMD -MP
LDFLAGS = -lyaml

# Directories
SRC_DIR = src
BUILD_DIR = .build
INCLUDE_DIR = src

# Source files and object files
SRC_MAIN = $(SRC_DIR)/main.c
SOURCES = $(shell find $(SRC_DIR) -type f -name *.c)
HEADERS = $(shell find $(SRC_DIR) -type f -name *.h)
OBJECTS = $(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(SOURCES))
DEPS = $(OBJECTS:.o=.d)

# Target executable
TARGET = procmand

# Default target
all: $(TARGET)

# Link the target executable
$(TARGET): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

# Compile source files to object files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(dir $@)
	$(CC) -c -o $@ $< $(CFLAGS)

# Include dependency files
-include $(DEPS)

# Clean up build artifacts
.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) $(TARGET)

# Print a message when the Makefile is run
.PHONY: info
info:
	@echo "Building $(TARGET) with sources: $(SOURCES)"