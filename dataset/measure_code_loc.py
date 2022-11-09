import json
import pathlib
import shlex
import subprocess as sp
import sys
import tarfile

import pandas as pd
from tqdm import tqdm


def get_logical_loc(
        dataset: pathlib.Path,
        src_dir: pathlib.Path,
        loc_file: pathlib.Path) -> None:
    t = dataset.joinpath("target_code.tar")
    with tarfile.open(t, 'w:') as tar:
        [tar.add(i) for i in sorted(src_dir.glob("*.c"))]

    cmd = " ".join([
        "docker run --rm -v",
        f"{dataset}:{dataset} -w {dataset}",
        f"aldanial/cloc {t} -by-file --json --out={loc_file} --skip-uniqueness"])
    sp.run(shlex.split(cmd))
    t.unlink()


def show_loc(loc_file: pathlib.Path):
    loc_data = json.loads(loc_file.read_text())
    x = pd.DataFrame(
        [int(loc_data[str(i)[1:]]["code"]) for i in sorted(src_dir.glob("*.c"))],
        columns=["Logial LOC"],
        index=sorted(src_dir.glob("*.c"))).describe().T

    print(x.to_markdown())


if __name__ == '__main__':
    cwd = pathlib.Path().resolve()
    dataset = pathlib.Path(sys.argv[1]).resolve()
    if not dataset.exists():
        sys.exit(1)
    src_dir = dataset.joinpath("src")
    loc_file = dataset.joinpath("code_loc.json")

    if not loc_file.exists():
        get_logical_loc(dataset, src_dir, loc_file)

    show_loc(loc_file)
    sys.exit(0)
