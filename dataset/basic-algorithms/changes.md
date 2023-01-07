# Changes from the original

## anagram

**Misc**
- (Delete) `rand_string` function and random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`stdlib.h`, `time.h`).

## armstrong

**Input**
- (Change) Use `strtol` function to read argv.

**Output**
- (Add) Display the input value.

**Misc**
- (Delete) Include statement for unnecessary libraries (`time.h`).

## binaryadd

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

## binarymult

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

## binarysearch

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 12.

## binarysearchrec

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 12.

## bubblesort

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Processes to exit with a `return 1` if argc is not 11.

## comparestrings

- None

## concatstrings

- None

## decimaltobinary

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if `num` is less than 0.

## decimaltohex

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if `num` is less than 0.

## decimaltooctal

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if `num` is less than 0.

## factorial

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Exit with a `return 1` if `num` is less than 0.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## factorialrec

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Return 1 if `num` is 0.
- (Add) Exit with a `return 1` if `num` is less than 0.

## fib

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Exit with a `return 1` if `n` is less than or equal to 0.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## floyd

**Input**
- (Change) Use `strtol` function to read argv.

**Misc**
- (Delete) `rand_string` function and random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## frequency

**Misc**
- (Delete) `rand_string` function and random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## gcd

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Exit with a `return 1` if either `num1` or `num2` is less than or equal to 0.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## gcdrec

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if either `n1` or `n2` is less than or equal to 0.

## insertionsort

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.

## iofile

**Process**
- (Add) Remove `program.txt` after write and read process.

**Misc**
- (Delete) `rand_string` function and random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## lcm

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Exit with a `return 1` if either `num1` or `num2` is less than or equal to 0.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## linearsearch

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.

## mergesort

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.

## minmaxarray

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.

## multtable

**Output**
- (Change) Output format from `%d` to `%ld` to prevent overflow of digits.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## numtowords

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argv is less than 0 or greater than the maximum int value (2147483648).

## palindrome

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Exit with a `return 1` if argv is less than 0 or greater than the maximum int value (2147483648).

**Misc**
- (Delete) Include statement for unnecessary libraries (`time.h`).

## perfect

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argv is not a natural number (assume 0 is not in the natural numbers).

## prime

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Determine that a number is not prime if n = 1.

**Misc**
- (Delete) Random seed value setting statement.
- (Delete) Include statement for unnecessary libraries (`time.h`).

## printinitials

**Process**
- (Add) Exit with a `return 1` if argv do not have a whitespace.
    - (Add) `string.h` library.

## pyramid

**Input**
- (Change) Use `strtol` function to read argv.

**Misc**
- (Delete) Include statement for unnecessary libraries (`time.h`).


## quicksort

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.


## random

**Input**
- (Change) Use `strtol` function to read argv.

**Process**
- (Add) Fix random number seed value at 10.
- (Add) Exit with a `return 1` if argv is less than 0 or greater than the maximum int value (2147483648).

## reverse

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argv is less than 0 or greater than the maximum int value (2147483648).

## romannumerals

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

## selectionsort

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Add) Exit with a `return 1` if argc is greater than 11.

## stringtoASCII

- None

## sumrec

**Input**
- (Change) Use `strtol` function to read argv.
    - (Add) `stdlib.h` library.

**Process**
- (Change) Initialize variable `s` with 0 at definition.
- (Add) Exit with a `return 1` if argv is less than 0 or greater than the maximum int value (2147483648).

**Output**
- (Change) Output from hex to decimal.

## vowels

**Misc**
- (Delete) `rand_string` function and random seed value setting statement, no longer needed variables (`o`).
- (Delete) Include statement for unnecessary libraries (`time.h`).
