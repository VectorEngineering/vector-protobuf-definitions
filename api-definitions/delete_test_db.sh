#!/bin/bash

# Delete all gen_test.db files in the current directory and subdirectories
find . -name "gen_test.db" -type f -delete
echo "Test database files deleted successfully"
