#!/usr/bin/env bash

(

  set -e

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

  echo "### Running equality tests, depending on order"
  echo "# Run a <diff> b"
  cargo run "$PWD/test/test.kdbx" "$PWD/test/test2.kdbx" --passwords demopass --no-color >"$PWD/target/test-result-01.txt"
  echo "# Run b <diff> a"
  cargo run "$PWD/test/test2.kdbx" "$PWD/test/test.kdbx" --passwords demopass --no-color >"$PWD/target/test-result-02.txt"

  lines_of_run_01=$(cat target/test-result-01.txt | wc -l)
  lines_of_run_02=$(cat target/test-result-02.txt | wc -l)

  test_equal "first run should have same amount of lines as second run" $lines_of_run_01 $lines_of_run_02

  amount_of_plus_01=$(cat target/test-result-01.txt | grep '^+' | wc -l)
  amount_of_minus_01=$(cat target/test-result-01.txt | grep '^-' | wc -l)
  amount_of_tilde_01=$(cat target/test-result-01.txt | grep '^~' | wc -l)
  amount_of_plus_02=$(cat target/test-result-02.txt | grep '^+' | wc -l)
  amount_of_minus_02=$(cat target/test-result-02.txt | grep '^-' | wc -l)
  amount_of_tilde_02=$(cat target/test-result-02.txt | grep '^~' | wc -l)

  test_gt "should output more than 0 plus lines" $amount_of_plus_01 0
  test_gt "should output more than 0 minus lines" $amount_of_minus_01 0
  test_gt "should output more than 0 tilde lines" $amount_of_tilde_01 0

  test_equal "first run should have same amount of tilde lines as second run" $amount_of_tilde_01 $amount_of_tilde_02
  test_equal "first run should have same amount of plus lines as second run has minus lines" $amount_of_plus_01 $amount_of_minus_02
  test_equal "first run should have same amount of minus lines as second run has plus lines" $amount_of_minus_01 $amount_of_plus_02

)
