#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
@author: luc
"""

import os
import sys
import pandas as pd
import numpy as np
import glob
import pylab as pl
from matplotlib import cm
# from scipy.ndimage.filters import gaussian_filter1d
from scipy.signal import butter, lfilter, freqz
import scipy.fftpack
from datetime import datetime
pl.style.use('science')
#pl.style.use('ieee')

#From: https://stackoverflow.com/a/25192640/13187605
def butter_lowpass(cutoff, fs, order=5):
    nyq = 0.5 * fs
    normal_cutoff = cutoff / nyq
    b, a = butter(order, normal_cutoff, btype='low', analog=False)
    return b, a

def butter_lowpass_filter(data, cutoff, fs, order=5):
    b, a = butter_lowpass(cutoff, fs, order=order)
    y = lfilter(b, a, data)
    return y

#%%
#global variables:
data_path = "./datas_csv/"
tot_transactions = 50000
conf_figsize=(20, 10)
# conf_figsize=(6.4, 4.8)
filter_out_all_with_elements= [] #["30tps|4_nodes", "30tps|6_nodes", "30tps|12_nodes", "30tps|18_nodes"] #["50tps|6_nodes", "40tps|6_nodes", "30tps|6_nodes", "50tps|12_nodes", "40tps|12_nodes", "30tps|12_nodes"] #"50tps", "40tps", "30tps"
filter_out_reverse=False
export_data_path = "./datas_csv_paper_ready/"

export_img_figs = True #save img to file if true
export_img_filename = "./img/" + sys.argv[1] if len(sys.argv)>1 else None #"./img/18_nodes" # change as desired



#%%
#
# Get all CSV files and put it inside dataframes
#
all_files = glob.glob(data_path + "merged_*")
all_df = {}
for filename in all_files:
    df = pd.read_csv(filename, index_col=None, header=0)
    field_name = os.path.basename(filename)[:-4].replace("merged_","") #remove extension
    
    all_df[field_name] = df    
    print("FINAL")
    fulltimeinterval = len(df)
    cfinaltps = tot_transactions/fulltimeinterval
    # invalid rate
    cinvalidrt = df['avg_invalid_rate'].max()
    cfork = df['avg_forks'].max()
    print("finaltps:")
    print(cfinaltps)
    print("invalidtx:")
    print(cinvalidrt)
    print("forks:")
    print(cfork)
    #for i in range(len(df)):
     #print(i)
     #print(df.loc[i, "time"], df.loc[i, "avg_finished_rate"])
    print('FINAL_FINISHED')
        
exit()

# pl.show()