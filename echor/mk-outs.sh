#!/usr/bin/env bash  
# 1

OUTDIR="tests/expected" # 2 
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR" # 3

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello  there" > $OUTDIR/hello1.n.txt
echo -n "Hello"  "there" > $OUTDIR/hello2.n.txt


# 1 - A special comment (aka a shebang) that tells the operating system to use the environment to execute bash for the following code.
# 2 - Define a variable for the output directory.
# 3 - Test if the output directory does not exist and create it if needed.
