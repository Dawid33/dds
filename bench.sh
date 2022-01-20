#!/bin/sh

if [[ ! -d benchmarks ]]; then
    mkdir benchmarks;
fi

cd benchmarks

output="Default Output : This should be overriden."
filename="my_file"

echo "Running benchmark..."

html_parser=$(cd ../html-parser && cargo bench)
//TODO : parse this output into something useful

echo "Saving benchmark to $filename"