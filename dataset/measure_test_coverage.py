import json
import pathlib
import shlex
import subprocess as sp
import sys

import pandas as pd
from tqdm import tqdm


def compile(src: pathlib.Path, dst: pathlib.Path, flag: str):
    cmd = f"gcc {flag} -o {str(dst)} {str(src)}"
    sp.run(shlex.split(cmd))


def compile_for_gcov(src: pathlib.Path, dst: pathlib.Path):
    compile(src=src, dst=dst, flag="-fprofile-arcs -ftest-coverage")


def show_test_coverage(file: pathlib.Path):
    data = pd.DataFrame(json.loads(file.read_text())).set_index("name")
    print(data.to_markdown())


if __name__ == '__main__':
    cwd = pathlib.Path().resolve()
    dataset = pathlib.Path(sys.argv[1]).resolve()
    if not dataset.exists():
        sys.exit(1)
    src_dir = dataset.joinpath("src")
    test_dir = dataset.joinpath("test")
    coverage_file = dataset.joinpath("test_coverage.json")

    if coverage_file.exists():
        show_test_coverage(coverage_file)
        sys.exit(0)

    data = sorted([
        (i, json.loads(test_dir.joinpath(i.with_suffix(".json").name).read_text()))
        for i in src_dir.glob("*.c")], key=lambda x: x[0])
    coverage_result = []
    for (src, testcase) in tqdm(data, desc="Measuring test coverage (gcov)"):
        # Set path
        elf = src.with_suffix(".elf")
        gcda = cwd.joinpath(src.with_suffix(".gcda").name)
        gcno = cwd.joinpath(src.with_suffix(".gcno").name)
        gcov_code = cwd.joinpath(src.with_suffix(".c.gcov").name)
        [i.unlink() for i in [elf, gcda, gcno, gcov_code] if i.exists()]

        # Test
        compile_for_gcov(src=src, dst=elf)
        [sp.run(shlex.split(' '.join([str(elf), *case["args"]])),
                capture_output=True, encoding="utf-8") for case in testcase]

        # Check coverage
        cmd = shlex.split(f"gcov -b {str(gcda)}")
        res = sp.run(cmd, capture_output=True, encoding="utf-8",
                     env={"LANG": "C"}).stdout.splitlines()[1:5]
        coverage_result += [dict(
            name=src.name,
            # lines_executed=res[0].split(":")[1],
            # branches_executed=res[1].split(":")[1],
            # taken_at_least_one=res[2].split(":")[1],
            # calls_executed=res[3].split(":")[1])]
            lines_executed=res[0].split(":")[1].split()[0],
            branches_executed=res[1].split(":")[1].split()[0],
            taken_at_least_one=res[2].split(":")[1].split()[0],
            calls_executed=res[3].split(":")[1].split()[0])]

        [i.unlink() for i in [elf, gcda, gcno, gcov_code] if i.exists()]

    coverage_file.write_text(json.dumps(coverage_result, indent=4))
    show_test_coverage(coverage_file)
    sys.exit(0)
