import io
from pathlib import Path

INPUT_DIR = "new_data"
OUTPUT_DIR = "new_data_result"
FILE_LENGTH = 41700

subjects = [
    "opwvtd9h",
    "pgdrzql6",
    "phepjqh5",
    "q9zk9d60",
    "qox2w7zs",
    "qu5g9dgl",
    "qyxt7aso",
    "rrp3kgd0",
    "rvdodovm",
    "a68d5xp5",
    "aoiyzwiy",
    "b2scmyvu",
    "bszjeu9t",
    "cybu8sao",
    "d85ay07w",
    "dvpmnk0o",
    "efhumn1s",
    "f1exphe7",
    "fa7dlyiy",
    "h01ew7gu",
    "ja50o035",
    "on3kplto",
    "soshxei0",
    "thkr6bpz",
    "ubtkiuvp",
    "um0bvhu4",
    "ur5xvo5v",
    "vui8ua9x",
    "wi1lmhbs",
    "wxki41ba",
    "y6ykwq70",
    "yi3hkj7g",
    "zn6x1no3",
    "zzb3eyyq",
]

for sub in subjects:
    Path(f"{OUTPUT_DIR}/{sub}").mkdir(parents=True, exist_ok=True) 


print(subjects)

def make_files(start, end, file_number):
    for dir in subjects:
        for filename in [
           "meg0111.csv",     
           "meg0121.csv",      
           "meg0131.csv",      
           "meg0211.csv",      
           "meg0221.csv",      
           "meg0231.csv",
           "meg0311.csv",
           "meg0321.csv",
           "meg0331.csv",
           "meg1511.csv",
           "meg1521.csv",
           "meg1531.csv",
        ]:
            count = 0
            filebase = filename.split(".csv")[0]
            new_filename = f"{OUTPUT_DIR}/{dir}/{filebase}_{file_number}.csv"
            print(new_filename)
            f = io.open(f"{INPUT_DIR}/{dir}/{filename}")
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
