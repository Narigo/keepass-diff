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

  lines_of_run_01=$(wc -l <"$PWD/tmp-tests/test-result-01.txt")
  lines_of_run_02=$(wc -l <"$PWD/tmp-tests/test-result-02.txt")

  test_equal "first run should have same amount of lines as second run" "$lines_of_run_01" "$lines_of_run_02"

  # Need to allow non-0 exit code because we may have 0 lines of tilde here
  set +e
  amount_of_plus_01=$(grep -c '^+' <"$PWD/tmp-tests/test-result-01.txt")
  amount_of_minus_01=$(grep -c '^-' <"$PWD/tmp-tests/test-result-01.txt")
  amount_of_tilde_01=$(grep -c '^~' <"$PWD/tmp-tests/test-result-01.txt")
  amount_of_plus_02=$(grep -c '^+' <"$PWD/tmp-tests/test-result-02.txt")
  amount_of_minus_02=$(grep -c '^-' <"$PWD/tmp-tests/test-result-02.txt")
  amount_of_tilde_02=$(grep -c '^~' <"$PWD/tmp-tests/test-result-02.txt")
  set -e

  test_gt "should output more than 0 plus lines" "$amount_of_plus_01" 0
  test_gt "should output more than 0 minus lines" "$amount_of_minus_01" 0
  test_gt "should output more than 0 tilde lines" "$amount_of_tilde_01" 0

  test_equal "first run should have same amount of tilde lines as second run" "$amount_of_tilde_01" "$amount_of_tilde_02"
  test_equal "first run should have same amount of plus lines as second run has minus lines" "$amount_of_plus_01" "$amount_of_minus_02"
  test_equal "first run should have same amount of minus lines as second run has plus lines" "$amount_of_minus_01" "$amount_of_plus_02"

  echo "### Running regular equality tests, depending on order"
  echo "# Run a <diff> b"
  cargo run --release -- "$PWD/test/test.kdbx" "$PWD/test/test2.kdbx" --passwords demopass --no-color >"$PWD/tmp-tests/test-result-03.txt"
  echo "# Run b <diff> a"
  cargo run --release -- "$PWD/test/test2.kdbx" "$PWD/test/test.kdbx" --passwords demopass --no-color >"$PWD/tmp-tests/test-result-04.txt"

  lines_of_run_03=$(wc -l <"$PWD/tmp-tests/test-result-03.txt")
  lines_of_run_04=$(wc -l <"$PWD/tmp-tests/test-result-04.txt")

  test_equal "first run should have same amount of lines as second run" "$lines_of_run_03" "$lines_of_run_04"

  # Need to allow non-0 exit code because we may have 0 lines of tilde here
  set +e
  amount_of_plus_03=$(grep -c '^+' <"$PWD/tmp-tests/test-result-03.txt")
  amount_of_minus_03=$(grep -c '^-' <"$PWD/tmp-tests/test-result-03.txt")
  amount_of_tilde_03=$(grep -c '^~' <"$PWD/tmp-tests/test-result-03.txt")
  amount_of_plus_04=$(grep -c '^+' <"$PWD/tmp-tests/test-result-04.txt")
  amount_of_minus_04=$(grep -c '^-' <"$PWD/tmp-tests/test-result-04.txt")
  amount_of_tilde_04=$(grep -c '^~' <"$PWD/tmp-tests/test-result-04.txt")
  set -e

  test_gt "should output more than 0 plus lines" "$amount_of_plus_03" 0
  test_gt "should output more than 0 minus lines" "$amount_of_minus_03" 0
  test_equal "should output 0 tilde lines" "$amount_of_tilde_03" 0
  test_equal "should output 0 tilde lines in second run as well" "$amount_of_tilde_04" 0

  test_equal "first run should have same amount of plus lines as second run has minus lines" "$amount_of_plus_03" "$amount_of_minus_04"
  test_equal "first run should have same amount of minus lines as second run has plus lines" "$amount_of_minus_03" "$amount_of_plus_04"

  echo "### Running test to open KDBX 3.1 files"
  echo "# Run a <diff> b"
  cargo run --release -- "$PWD/test/issue-24-kdbx-3.1/Test1.kdbx" "$PWD/test/issue-24-kdbx-3.1/Test2.kdbx" --password-a Test1 --password-b Test2 --no-color >"$PWD/tmp-tests/test-result-05.txt"
  echo "# Run b <diff> a"
  cargo run --release -- "$PWD/test/issue-24-kdbx-3.1/Test2.kdbx" "$PWD/test/issue-24-kdbx-3.1/Test1.kdbx" --password-a Test2 --password-b Test1 --no-color >"$PWD/tmp-tests/test-result-06.txt"

  lines_of_run_05=$(wc -l <"$PWD/tmp-tests/test-result-05.txt")
  lines_of_run_06=$(wc -l <"$PWD/tmp-tests/test-result-06.txt")

  test_equal "first run should have same amount of lines as second run" "$lines_of_run_05" "$lines_of_run_06"

  # Need to allow non-0 exit code because we may have 0 lines of tilde here
  set +e
  amount_of_plus_05=$(grep -c '^+' <"$PWD/tmp-tests/test-result-05.txt")
  amount_of_minus_05=$(grep -c '^-' <"$PWD/tmp-tests/test-result-05.txt")
  amount_of_tilde_05=$(grep -c '^~' <"$PWD/tmp-tests/test-result-05.txt")
  amount_of_plus_06=$(grep -c '^+' <"$PWD/tmp-tests/test-result-06.txt")
  amount_of_minus_06=$(grep -c '^-' <"$PWD/tmp-tests/test-result-06.txt")
  amount_of_tilde_06=$(grep -c '^~' <"$PWD/tmp-tests/test-result-06.txt")
  set -e

  test_gt "should output more than 0 plus lines" "$amount_of_plus_05" 0
  test_gt "should output more than 0 minus lines" "$amount_of_minus_05" 0
  test_equal "should output 0 tilde lines" "$amount_of_tilde_05" 0
  test_equal "should output 0 tilde lines in second run as well" "$amount_of_tilde_06" 0

  test_equal "first run should have same amount of plus lines as second run has minus lines" "$amount_of_plus_05" "$amount_of_minus_06"
  test_equal "first run should have same amount of minus lines as second run has plus lines" "$amount_of_minus_05" "$amount_of_plus_06"

  echo "### Testing snapshots against fixtures"
  for dir in test/test-*; do
    IFS='_' read -r -a files <<<"$(basename "$dir" | cut -c6-)"
    file_a=${files[0]}
    file_b=${files[1]}

    for snapshot in "$dir"/*; do
      IFS=' ' read -r -a args <<<"$(basename "${snapshot}" | cut -c8-)"

      mkdir -p "res/$dir"
      test_result_name="$PWD/tmp-tests/snapshot-result-$(basename "$dir").txt"
      cargo run --release -- "test/__fixtures__/${file_a}.kdbx" "test/__fixtures__/${file_b}.kdbx" "${args[@]}" >"$test_result_name"
      echo "# Run $snapshot"
      if diff "$test_result_name" "$snapshot"; then
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
