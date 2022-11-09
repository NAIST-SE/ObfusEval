import pathlib
import subprocess as sp
import shlex
import json
import sys
from typing import List


def compile(src: pathlib.Path, dst: pathlib.Path, flag: str) -> None:
    cmd = f"gcc -Wall {flag} -o {str(dst)} {str(src)}"
    if flag is None:
        cmd = f"gcc -Wall -o {str(dst)} {str(src)}"

    sp.run(shlex.split(cmd))


def compile_for_gcov(src: pathlib.Path, dst: pathlib.Path) -> None:
    compile(src=src, dst=dst, flag="-fprofile-arcs -ftest-coverage")


def make_testcases(src: pathlib.Path, args: List[str]) -> dict:
    elf = src.with_suffix(".elf")
    if elf.exists():
        elf.unlink()

    compile(src=src, dst=elf, flag=None)

    testcase = []
    for x in args:
        cmd = shlex.split(' '.join([str(elf), *x]))
        res = sp.run(cmd, capture_output=True, encoding="utf-8",)
        testcase += [dict(
            args=x,
            stdout=res.stdout,
            stderr=res.stderr,
            exit_status=res.returncode)]
    return testcase


if __name__ == '__main__':
    cwd = pathlib.Path().resolve()
    dataset = pathlib.Path(sys.argv[1]).resolve()
    if not dataset.exists():
        sys.exit(1)
    src_dir = dataset.joinpath("src")
    test_dir = dataset.joinpath("test")

    for src in sorted(src_dir.glob("*.c")):
        test_file = test_dir.joinpath(src.with_suffix(".json").name)
        if test_file.exists():
            continue

        print(f"{'_' * 80}\n{src.name}\n")

        args = [
            # Write args!
            [],
            ["0"],
        ]

        # Make Testcase
        testcases = make_testcases(src=src, args=args)
        for case in testcases:
            print("stdin:\n", case["args"])
            print("stdout:\n", case["stdout"])

        # Get test coverage
        elf = src.with_suffix(".elf")
        gcda = cwd.joinpath(src.with_suffix(".gcda").name)
        gcno = cwd.joinpath(src.with_suffix(".gcno").name)
        gcov_code = cwd.joinpath(src.with_suffix(".c.gcov").name)
        [i.unlink() for i in [elf, gcda, gcno, gcov_code] if i.exists()]

        compile_for_gcov(src=src, dst=elf)

        [sp.run(shlex.split(' '.join([str(elf), *case["args"]])),
                capture_output=True) for case in testcases]

        # Check coverage
        sp.run(shlex.split(f"gcov -b -f {str(gcda)}"),
               encoding="utf-8", env={"LANG": "C"})
        print("_" * 40)

        # Clean temp file
        [i.unlink() for i in [elf, gcda, gcno] if i.exists()]

        # Check that whether exist never executed code
        stop_flag = False
        is_all_executed_code = not [
            i for i in gcov_code.read_text().splitlines()
            if "never executed" in i]
        if not is_all_executed_code:
            print("Exist never executed code")
            stop_flag = True

        is_zero_percent_taken = not [
            i for i in gcov_code.read_text().splitlines()
            if "taken 0%" in i]
        if not is_zero_percent_taken:
            print("Exist zero percent taken branch or call")
            stop_flag = True

        if stop_flag:
            print(src)
            print(gcov_code)

        # Wait while something key
        if input("If ok, press the 'y' key then export testcase: ") != "y":
            exit()

        # Export testcase and clean file
        print(f"Export {test_file}")
        preview = json.dumps(testcases, indent=4)
        test_file.write_text(preview)
        gcov_code.unlink()
