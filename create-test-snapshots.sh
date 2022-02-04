#!/bin/bash

export RUSTFLAGS="-C target-cpu=native"

for dir in $(ls -1d test/test-*) ; do
  IFS='_' read -a files <<< "$(basename $dir | cut -c6-)"
  file_a=${files[0]}
  file_b=${files[1]}

  for result in "$dir"/* ; do
    args=$(basename "${result}" | cut -c8-)

    snapshot_dir="tmp-tests/snapshots/$dir"
    mkdir -p "$snapshot_dir"
    cargo run -- test/__fixtures__/${file_a}.kdbx test/__fixtures__/${file_b}.kdbx ${args} > "$snapshot_dir/result_${args}"
  done
done
