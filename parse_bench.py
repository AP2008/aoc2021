#! env python3
import os

def format_file(file, name):
    day_format = "\n| {} | {:.3f} |"
    with open(file) as f:
        l = f.readlines()[-1].split(',')
        return day_format.format(name, (float(l[5])/float(l[7])) / 10**6)

def main():
    dirs = list(filter(lambda x: x!='report', os.listdir("target/criterion/")))
    days = sorted([int(n[4:]) for n in filter(lambda x: x.startswith('Day'), dirs)])
    markdown = """| Day | Time (ms) |\n| --- | --- |"""
    day_format = "\n| {} | {:.3f} |"
    file = "target/criterion/Day {}/new/raw.csv"
    for day in days:
        markdown += format_file(file.format(day), "Day " + str(day))
    markdown += format_file("target/criterion/All Days/new/raw.csv", "All Days")

    print(markdown)

if __name__ == '__main__':
    main()
