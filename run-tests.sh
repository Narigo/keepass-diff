#!/usr/bin/env bash

(

  set -e

  export RUSTFLAGS='-C target-cpu=native'

  test_equal() {
    if [ "$2" == "$3" ]; then
      echo "✅ $1"
    else
      echo "❌ $1"
      echo "$2"
      echo " -> "
      echo "$3"
      exit 1
    fi
  }

  test_gt() {
    if [ "$2" -gt "$3" ]; then
      echo "✅ $1"
    else
      echo "❌ $1"
      echo "$2"
      echo " is smaller or equal to "
      echo "$3"
      exit 1
    fi
  }

  echo "### Preparing tests"
  echo "# Creating temporary directory for test results"
  mkdir -p "$PWD/tmp-tests"

  echo "### Running verbose equality tests, depending on order"
  echo "# Run a <diff> b"
  cargo run --release -- "$PWD/test/test.kdbx" "$PWD/test/test2.kdbx" --passwords demopass --no-color --verbose >"$PWD/tmp-tests/test-result-01.txt"
  echo "# Run b <diff> a"
  cargo run --release -- "$PWD/test/test2.kdbx" "$PWD/test/test.kdbx" --passwords demopass --no-color --verbose >"$PWD/tmp-tests/test-result-02.txt"

  lines_of_run_01=$(cat "$PWD/tmp-tests/test-result-01.txt" | wc -l)
  lines_of_run_02=$(cat "$PWD/tmp-tests/test-result-02.txt" | wc -l)

  test_equal "first run should have same amount of lines as second run" $lines_of_run_01 $lines_of_run_02

  amount_of_plus_01=$(cat "$PWD/tmp-tests/test-result-01.txt" | grep '^+' | wc -l)
  amount_of_minus_01=$(cat "$PWD/tmp-tests/test-result-01.txt" | grep '^-' | wc -l)
  amount_of_tilde_01=$(cat "$PWD/tmp-tests/test-result-01.txt" | grep '^~' | wc -l)
  amount_of_plus_02=$(cat "$PWD/tmp-tests/test-result-02.txt" | grep '^+' | wc -l)
  amount_of_minus_02=$(cat "$PWD/tmp-tests/test-result-02.txt" | grep '^-' | wc -l)
  amount_of_tilde_02=$(cat "$PWD/tmp-tests/test-result-02.txt" | grep '^~' | wc -l)

  test_gt "should output more than 0 plus lines" $amount_of_plus_01 0
  test_gt "should output more than 0 minus lines" $amount_of_minus_01 0
  test_gt "should output more than 0 tilde lines" $amount_of_tilde_01 0

  test_equal "first run should have same amount of tilde lines as second run" $amount_of_tilde_01 $amount_of_tilde_02
  test_equal "first run should have same amount of plus lines as second run has minus lines" $amount_of_plus_01 $amount_of_minus_02
  test_equal "first run should have same amount of minus lines as second run has plus lines" $amount_of_minus_01 $amount_of_plus_02

  echo "### Running regular equality tests, depending on order"
  echo "# Run a <diff> b"
  cargo run --release -- "$PWD/test/test.kdbx" "$PWD/test/test2.kdbx" --passwords demopass --no-color >"$PWD/tmp-tests/test-result-03.txt"
  echo "# Run b <diff> a"
  cargo run --release -- "$PWD/test/test2.kdbx" "$PWD/test/test.kdbx" --passwords demopass --no-color >"$PWD/tmp-tests/test-result-04.txt"

  lines_of_run_03=$(cat "$PWD/tmp-tests/test-result-03.txt" | wc -l)
  lines_of_run_04=$(cat "$PWD/tmp-tests/test-result-04.txt" | wc -l)

  test_equal "first run should have same amount of lines as second run" $lines_of_run_03 $lines_of_run_04

  amount_of_plus_03=$(cat "$PWD/tmp-tests/test-result-03.txt" | grep '^+' | wc -l)
  amount_of_minus_03=$(cat "$PWD/tmp-tests/test-result-03.txt" | grep '^-' | wc -l)
  amount_of_tilde_03=$(cat "$PWD/tmp-tests/test-result-03.txt" | grep '^~' | wc -l)
  amount_of_plus_04=$(cat "$PWD/tmp-tests/test-result-04.txt" | grep '^+' | wc -l)
  amount_of_minus_04=$(cat "$PWD/tmp-tests/test-result-04.txt" | grep '^-' | wc -l)
  amount_of_tilde_04=$(cat "$PWD/tmp-tests/test-result-04.txt" | grep '^~' | wc -l)

  test_gt "should output more than 0 plus lines" $amount_of_plus_03 0
  test_gt "should output more than 0 minus lines" $amount_of_minus_03 0
  test_equal "should output 0 tilde lines" $amount_of_tilde_03 0
  test_equal "should output 0 tilde lines in second run as well" $amount_of_tilde_04 0

  test_equal "first run should have same amount of plus lines as second run has minus lines" $amount_of_plus_03 $amount_of_minus_04
  test_equal "first run should have same amount of minus lines as second run has plus lines" $amount_of_minus_03 $amount_of_plus_04

  echo "### Running test to open KDBX 3.1 files"
  echo "# Run a <diff> b"
  cargo run --release -- "$PWD/test/issue-24-kdbx-3.1/Test1.kdbx" "$PWD/test/issue-24-kdbx-3.1/Test2.kdbx" --password-a Test1 --password-b Test2 --no-color >"$PWD/tmp-tests/test-result-05.txt"
  echo "# Run b <diff> a"
  cargo run --release -- "$PWD/test/issue-24-kdbx-3.1/Test2.kdbx" "$PWD/test/issue-24-kdbx-3.1/Test1.kdbx" --password-a Test2 --password-b Test1 --no-color >"$PWD/tmp-tests/test-result-06.txt"

  lines_of_run_05=$(cat "$PWD/tmp-tests/test-result-05.txt" | wc -l)
  lines_of_run_06=$(cat "$PWD/tmp-tests/test-result-06.txt" | wc -l)

  test_equal "first run should have same amount of lines as second run" $lines_of_run_05 $lines_of_run_06

  amount_of_plus_05=$(cat "$PWD/tmp-tests/test-result-05.txt" | grep '^+' | wc -l)
  amount_of_minus_05=$(cat "$PWD/tmp-tests/test-result-05.txt" | grep '^-' | wc -l)
  amount_of_tilde_05=$(cat "$PWD/tmp-tests/test-result-05.txt" | grep '^~' | wc -l)
  amount_of_plus_06=$(cat "$PWD/tmp-tests/test-result-06.txt" | grep '^+' | wc -l)
  amount_of_minus_06=$(cat "$PWD/tmp-tests/test-result-06.txt" | grep '^-' | wc -l)
  amount_of_tilde_06=$(cat "$PWD/tmp-tests/test-result-06.txt" | grep '^~' | wc -l)

  test_gt "should output more than 0 plus lines" $amount_of_plus_05 0
  test_gt "should output more than 0 minus lines" $amount_of_minus_05 0
  test_equal "should output 0 tilde lines" $amount_of_tilde_05 0
  test_equal "should output 0 tilde lines in second run as well" $amount_of_tilde_06 0

  test_equal "first run should have same amount of plus lines as second run has minus lines" $amount_of_plus_05 $amount_of_minus_06
  test_equal "first run should have same amount of minus lines as second run has plus lines" $amount_of_minus_05 $amount_of_plus_06

  echo "### Testing snapshots against fixtures"
  for dir in $(ls -1d test/test-*) ; do
    IFS='_' read -a files <<< "$(basename $dir | cut -c6-)"
    file_a=${files[0]}
    file_b=${files[1]}
  
    for snapshot in "$dir"/* ; do
      args=$(basename "${snapshot}" | cut -c8-)
  
      mkdir -p "res/$dir"
      test_result_name="$PWD/tmp-tests/snapshot-result-$(basename $dir).txt"
      cargo run --release -- test/__fixtures__/${file_a}.kdbx test/__fixtures__/${file_b}.kdbx ${args} > $test_result_name
      echo "# Run $snapshot"
      diff "$test_result_name" "$snapshot"
      if [ "$?" -eq "0" ]; then
        echo "✅ $snapshot"
      else
        echo "❌ $snapshot"
        exit 1
      fi
    done
  done

  echo "### Cleaning up test results - everything was good!"
  echo "# Removing temporary directory of test results"
  rm -r "$PWD/tmp-tests"

)
