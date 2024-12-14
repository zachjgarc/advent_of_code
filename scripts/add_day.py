from datetime import datetime
import os
import re
import sys

# declare paths
SRC_DIR = "src"

# validate arguments

num_arguments = len(sys.argv)

if num_arguments == 1 or num_arguments > 3:
    print("Usage:\n\t1. python scripts/add_day.py <year> <day>\n\t2. python scripts/add_day.py <day>")
    sys.exit(1)
    
current_year = datetime.now().year

if len(sys.argv) == 3:
    try:
        year = int(sys.argv[1])
        day = int(sys.argv[2])
    except ValueError:
        print("<year> and <day> must be integers")
        sys.exit(1)
else:
    year = current_year
    try:
        day = int(sys.argv[1])
    except ValueError:
        print("<day> must be an integer")
        sys.exit(1)

if not 2015 <= year <= current_year:
    print(f"Advent of Code {year} does not exist. 2015 ≤ <year> ≤ {current_year}")
    sys.exit(1)
if not 1 <= day <= 25:
    print("<day> between 1 and 25.")
    sys.exit(1)

print(f"Adding {year} day {day}")


# declare year module if nonexistent

year_dir = os.path.join(SRC_DIR, f"advent_of_code_{year}")

if not os.path.exists(year_dir):
    os.makedirs(year_dir, exist_ok=False)
    
    # create src/advent_of_code_<year>/mod.rs
    year_mod_path = os.path.join(year_dir, "mod.rs")
    if os.path.exists(year_mod_path):
        print(f"{year_dir} doesn't exist, but {year_mod_path} does... somehow.")
        sys.exit(1)
    with open(year_mod_path, "w") as f:
        f.write("pub mod days;")
    print(f"Created {year_mod_path}")
    
    # update main.rs
    main_path = os.path.join(SRC_DIR, "main.rs")
    main_addition = f"mod advent_of_code_{year};\n"
    if not os.path.exists(main_path):
        print(f"{main_path} doesn't exist... somehow.")
        sys.exit(1)
    with open(main_path, "r") as f:
        main_rs = f.read()
    mod_pattern = r"mod\sadvent_of_code_\d{4};\n"
    mod_matches = re.findall(mod_pattern, main_rs)
    if main_addition in mod_matches:
        print(f"{year_dir} doesn't exist but mod advent_of_code_{year} does... somehow.")
        sys.exit(1)
    mod_matches.append(main_addition)
    mod_matches.sort(key=lambda line: int(line[-6:-2]))
    updated_main_rs = re.sub(
        r"(mod\sadvent_of_code_\d{4};\n)*?(?=\nmod\sutils)",
        "".join(mod_matches),
        main_rs,
        count=1
    )
    with open(main_path, "w") as f:
        f.write(updated_main_rs)
    print(f"Updated {main_path} for year module")
    
    # update base.rs
    base_path = os.path.join(SRC_DIR, "utils", "base.rs")
    base_addition = f"use crate::advent_of_code_{year}::days as days_{year};\n"
    if not os.path.exists(base_path):
        print(f"{base_path} doesn't exist... somehow.")
        sys.exit(1)
    with open(base_path, "r") as f:
        base_rs = f.read()
    use_pattern = r"use\scrate\:\:advent_of_code_\d{4}\:\:days\sas\sdays_\d{4};\n"
    use_matches = re.findall(use_pattern, base_rs)
    if base_addition in use_matches:
        print(f"{year_dir} doesn't exist but advent_of_code_{year}::days does... somehow.")
        sys.exit(1)
    use_matches.append(base_addition)
    use_matches.sort(key=lambda line: int(line[-6:-2]))
    updated_base_rs = re.sub(
        r"(use\scrate\:\:advent_of_code_\d{4}\:\:days\sas\sdays_\d{4};\n)*?(?=\npub\sstatic\sCURRENT_YEAR)",
        "".join(use_matches),
        base_rs,
        count=1
    )
    with open(base_path, "w") as f:
        f.write(updated_base_rs)
    print(f"Updated {base_path} for year module")


# ensure dirs exist

day_str = f"day_{day:02}"

days_dir = os.path.join(year_dir, "days")
day_dir = os.path.join(days_dir, day_str)
inputs_dir = os.path.join(year_dir, "inputs")
input_path = os.path.join(inputs_dir, f"{day_str}.txt")
test_inputs_dir = os.path.join(year_dir, "test inputs")
test_input_path = os.path.join(test_inputs_dir, f"{day_str}.txt")

os.makedirs(day_dir, exist_ok=True)

os.makedirs(inputs_dir, exist_ok=True)
if os.path.exists(input_path):
    print(f"{input_path} already exists")
else:
    with open(input_path, "w") as f:
        f.write("")
    print(f"Created {input_path}")

os.makedirs(test_inputs_dir, exist_ok=True)
if os.path.exists(test_input_path):
    print(f"{test_input_path} already exists")
else:
    with open(test_input_path, "w") as f:
        f.write("")
    print(f"Created {test_input_path}")


# create one.rs and two.rs

one_path = os.path.join(day_dir, "one.rs")
two_path = os.path.join(day_dir, "two.rs")
for solution_path in [one_path, two_path]:
    if not os.path.exists(solution_path):
        with open(solution_path, "w") as f:
            f.write(
"""#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(_input: &str) -> u32 {
    todo!();
}"""
            )
        print(f"Created {solution_path}")
    else:
        print(f"{solution_path} already exists")
        # TODO: create option to overwrite existing code


# create src/advent_of_code_<year>/days/day_<day>/mod.rs

day_mod_path = os.path.join(day_dir, "mod.rs")
day_mod_file_exists = os.path.exists(day_mod_path)
with open(day_mod_path, "w") as f:
    f.write("pub mod one;\npub mod two;")
if not day_mod_file_exists:
    print(f"Created {day_mod_path}")
else:
    print(f"Overwrote {day_mod_path}")


# update src/advent_of_code_<year>/days/mod.rs

days_mod_path = os.path.join(days_dir, "mod.rs")
days_mod_addition = f"pub mod {day_str};\n"
if not os.path.exists(days_mod_path):
    with open(days_mod_path, "w") as f:
        f.write(days_mod_addition)
        print(f"Created {days_mod_path}")
else:
    with open(days_mod_path, "r") as f:
        lines = f.readlines()
    existing_mods = [line for line in lines if line.startswith("pub mod day_")]
    day_mod_declaration_exists = days_mod_addition in existing_mods
    if not day_mod_declaration_exists:
        existing_mods.append(days_mod_addition)
    existing_mods.sort(key=lambda line: int(line.strip()[-3:-1]))
    with open(days_mod_path, "w") as f:
        f.writelines(existing_mods)
    if not day_mod_declaration_exists:
        print(f"Updated {days_mod_path}")
    else:
        print(f"Declaration for mod {day_str} already exists")


# update base.rs

base_path = os.path.join(SRC_DIR, "utils", "base.rs")
if not os.path.exists(base_path):
    print(f"{base_path} doesn't exist... somehow.")
    sys.exit(1)
with open(base_path, "r") as f:
    base_rs = f.read()
year_pattern = r"(\d{4})\s*=>\s*match\s*day\s*\{(.*?),\n\s*_\s*=>\s*None"
year_matches = {
    int(year): re.split(r",\s*(?=\d)", block.strip())
    for year, block in re.findall(year_pattern, base_rs, re.DOTALL)
}

base_addition = f"{day} => Some((\n                get_solution(days_{year}::{day_str}::one::run, input, run_case),\n                get_solution(days_{year}::{day_str}::two::run, input, run_case)\n            ))"
year_arm_exists = year in year_matches.keys()
day_arm_exists = False
if not year_arm_exists:
    year_matches[year] = [base_addition]
else:
    day_arm_exists = base_addition in year_matches[year]
    if not day_arm_exists:
        year_matches[year].append(base_addition)

sorted_years = sorted(year_matches.keys())

for y in sorted_years:
    year_matches[y].sort(key=lambda arm: int(arm.split(" ")[0]))
    year_matches[y].append("_ => None")

updated_match_block = "\n".join(
    f"        {y} => match day {{\n            {",\n            ".join(year_matches[y])}\n        }}" for y in sorted_years
)

updated_base_rs = re.sub(
    r"(?<=match\syear\s\{\n).*?(?=\n\s*_\s*=>\s*None\n\s*\}\n\})",
    updated_match_block,
    base_rs,
    count=1,
    flags=re.DOTALL
)
    
with open(base_path, "w") as f:
    f.write(updated_base_rs)

if not year_arm_exists:
    print(f"Created {year} arm in {base_path}")
if not day_arm_exists:
    print(f"Created day {day} sub-arm in {base_path}")
else:
    print(f"Day {day} sub-arm already exists in {base_path}")