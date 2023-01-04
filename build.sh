#!/bin/bash

cargo build --release --bin "[0-9]*" -q
cargo build --release --bin "[0-9]*" -q --target x86_64-pc-windows-gnu

find ./target/release -maxdepth 1 -type f -perm 755 | while read file; do
	arr=(${file//\// })
	mv $file binaries/linux/${arr[3]}
done

find ./target/x86_64-pc-windows-gnu/release -maxdepth 1 -type f -perm 755 | while read file; do
	arr=(${file//\// })
	mv $file binaries/win64/${arr[4]}
done
