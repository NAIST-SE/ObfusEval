# dataset

- This directory contains a dataset consisting of programs written in the C language and testcase JSON file.

|name|link|
|:-|:-|
|[basic-algorithms](basic-algorithms/)|[README.md](basic-algorithms/README.md)|

## Helper script

### [make_testcase.py](make_testcase.py)

```bash
$ python3.9 make_testcase.py <dataset_name>
```

Assist to manual make testcase
- Before run, change the contents of the "args" variable.
- When execution, the code coverage when the program in dataset are given "args" will display.

### [measure_code_loc.py](measure_code_loc.py)

```bash
$ python3.9 measure_code_loc.py <dataset_name>
```

Measure lines of logical code.

### [measure_test_coverage.py](measure_test_coverage.py)

```bash
$ python3.9 measure_test_coverage.py <dataset_name>
```

Measure test coverage and display
