#!/bin/bash

SRC=$1
TARG=$2
shift 2; TAIL=$*
echo ": $SRC $TAIL |> !rustlib |> `rustc --crate-type rlib --crate-file-name $SRC | tr '\\n' ' '` $TARG"
