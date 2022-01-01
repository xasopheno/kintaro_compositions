import io
from pathlib import Path
import all_channels

INPUT_DIR = "wi1lmhbs"
#  INPUT_DIR = "sub-1"
OUTPUT_DIR = "result"
N_MEGS = 5
FILE_LENGTH = 41700


for subject in subjects:
    new_dir = f'{subject}'
    Path(f"{OUTPUT_DIR}/{new_dir}").mkdir(parents=True, exist_ok=True) 


print(subjects)

def make_files(start, end, file_number):
    for dir in subjects:
        for filename in list(map(lambda x: x.lower(), all_channels.channels)): 
            count = 0
            filebase = filename.split(".csv")[0]
            new_filename = f"{OUTPUT_DIR}/{dir}/{filebase}_{file_number}.csv"
            print(new_filename)
            f = io.open(f"{INPUT_DIR}/{filename}.csv")
            line = f.readline()
            result_file = open(new_filename, 'w')
            for datum in line.split(","):
                if count > start:
                    result_file.write(datum + ",")

                count += 1
                if count >= end:
                    result_file.write("0.0")
                    break
            print(count, count/41700)

if __name__ == "__main__":
    file_count = 0
    start = 0

    for i in range(5):
        end = start + FILE_LENGTH
        print(start, end)        

        make_files(start, end, i)
        
        start += FILE_LENGTH

    make_files(0, FILE_LENGTH * 5, "full")
