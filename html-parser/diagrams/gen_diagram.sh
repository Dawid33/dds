#!/bin/bash

dot -Tpng parser.dot -o parser.png
dot -Tpng tokenizer.dot -o tokenizer.png

cargo tree --workspace --color always > tree.txt