import os
import sys
from pathlib import Path
import logging
import json
import mne
import numpy as np
import scipy.stats as stats
from matplotlib import pyplot
import matplotlib.pyplot as plt
import matplotlib.pyplot
import glob
import re

DEBUG=False
OUTPUT_DIR = "new_result"
FILE_LENGTH = 41700
DATA_DIR = "../../neuro_data"
N_CHUNCKS = 4
N_SUBJECTS = 11
#  DATASET_DIR = "ds003352-download"
DATASET_DIR = "ds003703-download"

def ds003352_namer(filename):
    result = filename.split("download/")[1]
    result = result.split("/meg")[0]
    result = result.split("/")
    result = "_".join(result)
    result = result.replace("-", "_")
    return result

def ds003703_namer(filename):
    result = filename.split("download/")[1]
    result = result.lower()
    result = result.split("/meg")[1]
    result = result.split("/sub-")[1]
    result = result.split("_epo.fif")[0]
    result = result.replace("-", "_")
    result = result.replace("_task", "")
    result = result.replace("_run", "")
    result = result.replace("_meg", "")
    result = result.replace("ingtospeech", "")
    result = result.split(".fif")[0]
    return result
    
def walker(input_dir):
    pathname = input_dir + "/**/*.fif"
    files = glob.glob(pathname, recursive=True)
    #  filemap = {}
    #  for (i, file) in enumerate(files):
        #  filemap[f"sub_{i:03}"] = file
    return files

def process_fif(filename: str, namer):
    raw = mne.io.read_raw_fif(filename)

    if not DEBUG:
        print(raw)
        print(raw.info)
        print(raw.ch_names)
        ica = mne.preprocessing.ICA(n_components=20, random_state=97, max_iter=800)
        ica.fit(raw)
        ica.exclude = [1, 2]  # details on how we picked these are omitted here

        #  orig_raw = raw.copy()
        raw.load_data()
        ica.apply(raw)

    chs = raw.ch_names

    print("printing...")
    for ch in chs:
        channel = str(ch)
        m = np.array(raw[channel][0])
        output_name = namer(filename)
        make_files(m, channel, f"{output_name}")


def make_files(raw: np.array, channel, subject):
    data = raw[0]
    print(data.size)
    chunks_full = np.array(data[0:FILE_LENGTH * N_CHUNCKS])
    print(chunks_full.shape)
    chunks = np.array([data[i:i+FILE_LENGTH] for i in range(0, data.shape[0], FILE_LENGTH)])[0:N_CHUNCKS]
    result_dir = f"{OUTPUT_DIR}/{DATASET_DIR}/{subject}".replace("-", "_")
    Path(result_dir).mkdir(parents=True, exist_ok=True) 
    for (file_number, chunk)in enumerate(chunks):
        print(file_number, chunk.shape)
        new_filename = f"{result_dir}/{channel}_{file_number}.csv".replace("-", "_")
        print(new_filename)
        result_file = open(new_filename, 'w')
        for datum in chunk:
            result_file.write(str(datum) + ",")
        result_file.write("0.0")

    print("full", chunk.shape)
    new_filename = f"{result_dir}/{channel}_full.csv".replace("-", "_")
    print(new_filename)
    result_file = open(new_filename, 'w')
    for datum in chunks_full:
        result_file.write(str(datum) + ",")
    result_file.write("0.0")
    

def process_dir():
    namers = {
        "ds003352-download": ds003352_namer,
        "ds003703-download": ds003703_namer
    }
    files = walker(f"{DATA_DIR}/{DATASET_DIR}")

    count = 0
    for filename in files:
        if count > N_SUBJECTS: 
            break
        print(filename)
        try:
            process_fif(filename, namers[DATASET_DIR])
        except Exception as exception:
            logging.error(exception, exc_info=True)
            #  sys.exit(1)
            
        count += 1

if __name__ == "__main__":
    process_dir()



