import click
import sys
from pathlib import Path
from urllib.parse import urlparse
import subprocess


@click.command()
@click.argument("url", type=str)
def clone_problem(url: str):
    script_dir = Path(sys.argv[0]).parent
    problems_dir = script_dir / "problems"

    problem_name = urlparse(url).path.strip("/").split("/")[-1]
    print(f"clonning {problem_name}")


    problem_dir = problems_dir / problem_name
    init_cargo_project(problem_dir)

    with open(problem_dir / "link.txt", "w") as f:
        f.write(url)


def init_cargo_project(problem_dir: Path):
    try:
        problem_dir.mkdir()
    except FileExistsError:
        print("problem is already cloned")
        return

    def run_command(cmd: str):
        subprocess.run(cmd, shell=True,
                       check=True, cwd=problem_dir.absolute())

    run_command("cargo init --lib")
    run_command("cargo add --dev rstest")
    run_command("cargo add --dev --path=../../common")


if __name__ == "__main__":
    clone_problem()
