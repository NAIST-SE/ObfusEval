# ObfusEval

ObfusEval is the benchmarking tool to evaluate the reliability of the code obfuscating transformation.

The following two metrics related the reliability are evaluated.

- **Test Pass Rate** indicates the rate of programs that are successfully obfuscated while keeping the functionalities. 
- **Code Distance Mean** indicates how an obfuscated program is changed from the original program.

## How to use

### Preparation

1. Add a docker image of an obfuscating transformation tool.

    - Please place the Dockerfile in [docker/](./docker) subdirectory.
    - Define the `obfuscate` command in your Dockerfile as follows to handle commands related to obfuscation uniformly in this tool.
        ```bash
        RUN echo '#! /bin/sh\n<PATH TO THE OBFUSCATOR> $@' >> /bin/obfuscate && \
            chmod a+x /bin/obfuscate
        ```

2. Edit [obfuscator](./obfuscator)/tool_name.json

    - Set the parameters as follows.
        - Please refer to the [obfuscator](./obfuscator) existing file for details
        ```json
        {
            "name": <TOOL_NAME>,
            "compiler": <PATH_TO_COMPILER_IN_TOOL>,
            "common_flag": <COMMON_FLAG_ON_COMPILING>, 
            "is_bin_only": <Whether the obuscator generates obfuscated binary only>,
            "transformations": [
                "name": <This transformation name>,
                "display": <This transformation abbreviation>,
                "flag": [
                    <Command line parametes to execute the docker image>
                ]
            ]
        }
        ```

3. Edit [properties.json](./properties.json)
    
    - Set the pair of obfuscator that contains obfuscating transformations and dataset.

        **example**
        ```json
        {
            "name": "prop1",
            "dataset": "basic-algorithms", // the name of dataset/<each dataset>/
            "obfuscator": "tigress" // the name of obfuscator/<tool>.json
        }
        ```

4. Run docker service

    ```bash
    $ docker-compose up -d
    ```

### Setup

- Apply obfuscation to the code in the dataset
- Compile code using compiler in docker service

```bash
$ cargo run setup <only_mode_option> <prop-name>
```

**only_mode_option**
- `--compile-code`: Compile code in dataset by compiler same using obfuscator
- `--obfuscate-code`: Obfuscate code in dataset

When run with no option, do all mode (compile and obfuscate code in dataset.)

### Evaluate

```bash
$ cargo run evaluate <only_mode_option> <prop-name>
```

**only_mode_option**
- `--test-pass-rate`: *Write discription*
- `--code-distance-mean`: *Write discription*

When run with no option, do all mode.
<!-- 
## Benchmarks

### Test Pass Rate

<!-- *Write discription* -->
<!-- Test Pass Rate indicates how much that obfuscated program preserves original program functions.  
This benchmark value is recommended to be 1.0.

let $P$ is the set of programs in dataset, $T$ is the set of testcases in program.
And, $o()$ is function to apply obfuscation.
Then, Test Pass Rate is calculated below.

$$ Test\ Pass\ Rate = \dfrac{ \mid \left\{ p \in P \mid Passed(o(p), T) \right\} \mid }{|P|} $$
$$ Passed(o(p), T) = 
  \begin{cases}
    1 & \text{if}\ o(p)\ \text{passed all}\ testcase (testcase \in T)\\
    0 & otherwise
  \end{cases}$$

Passing a test case means that the program given the stdin (standard input) of test case satisfies all of the following.

- stdout (stndard output)
- stderr (standard error output)
- exit status

### Code Distance Average

*Write discription* -->

## LICENSE

- This source code in [src](src) is licensed MIT.

    > Copyright (c) 2022 Kitaoka Tetsuya - Software Engineering Lab @ NAIST
    > Released under the MIT license
    > https://github.com/NAIST-SE/ObfusEval/blob/master/LICENSE

- The source code in [dataset/basic-algorithms/src](dataset/basic-algorithms/src) is derived from [tum-i4/obfuscation-benchmarks](https://github.com/tum-i4/obfuscation-benchmarks).

    > Copyright (c) 2015 Software Engineering Chair 22 - Faculty of Informatics
    > Released under the MIT license
    > https://github.com/tum-i4/obfuscation-benchmarks/blob/master/LICENSE