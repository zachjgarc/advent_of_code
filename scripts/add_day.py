from datetime import datetime
import os
import sys

BASE_DIR = "../src"
YEAR_DIR_TEMPLATE = "advent_of_code_{year}"
DAYS_DIR = "days"
INPUTS_DIR = "inputs"
TEST_INPUTS_DIR = "test inputs"
DAY_TEMPLATE = """
pub fn run(input: &String) -> u32 {
    todo!();
}
"""

def add_day(year: int, day: int):
    print(f"Adding {year} day {day}")
    
    current_year = datetime.now().year()
    if not 2015 <= year <= current_year:
        print(f"Advent of Code {year} does not exist.")
        exit(1)
    
    if not day <= 25:
        print("Advent of Code has only 25 days.")
        exit(1)
        
    if not day >= 0:
        print("Please enter a valid positive day number.")
        exit(1)
    
    # define paths
    year_dir = os.path.join(BASE_DIR, YEAR_DIR_TEMPLATE.format(year=year))
    days_dir = os.path.join(year_dir, DAYS_DIR)
    inputs_dir = os.path.join(year_dir, INPUTS_DIR)
    test_inputs_dir = os.path.join(year_dir, TEST_INPUTS_DIR)
    
    # ensure all directories exist
    os.makedirs(days_dir, exist_ok=True)
    os.makedirs(inputs_dir, exist_ok=True)
    os.makedirs(test_inputs_dir, exist_ok=True)
    
    # create the solution files
    one_path = os.path.join(days_dir, f"one.rs")
    two_path = os.path.join(days_dir, f"two.rs")
    
    for file_path in [one_path, two_path]:
        if not os.path.exists(file_path):
            with open(file_path, "w") as f:
                f.write(DAY_TEMPLATE.format(day=day))
            print(f"Created: {file_path}")
        else:
            print(f"Day file already exists: {file_path}")
    
    # create mod.rs
    mod_path = os.path.join(days_dir, f"mod.rs")
    if not os.path.exists(mod_path):
        with open(mod_path, "w") as f:
            f.write("pub mod one;\npub mod two;")
        print(f"Created: {mod_path}")
    else:
        print(f"Mod file already exists: {mod_path}")

    # create input files
    input_file_path = os.path.join(inputs_dir, f"day_{day:02}.txt")
    test_input_file_path = os.path.join(test_inputs_dir, f"day_{day:02}.txt")
    
    for file_path in [input_file_path, test_input_file_path]:
        if not os.path.exists(file_path):
            with open(file_path, "w") as f:
                f.write("")
            print(f"Created: {file_path}")
        else:
            print(f"Input file already exists: {file_path}")
    
    # update mod.rs
    mod_file_path = os.path.join(days_dir, "mod.rs")
    if not os.path.exists(mod_file_path):
        with open(mod_file_path, "w") as f:
            f.write("")
    with open(mod_file_path, "r+") as f:
        content = f.read()
        mod_line = f"pub mod day_{day:02};\n"
        if mod_line not in content:
                f.write(mod_line)
                print(f"Updated: {mod_file_path}")