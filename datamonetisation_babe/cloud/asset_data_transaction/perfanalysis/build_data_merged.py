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
conf_figsize=(20, 10)
# conf_figsize=(6.4, 4.8)
filter_out_all_with_elements= [] #["30tps|4_nodes", "30tps|6_nodes", "30tps|12_nodes", "30tps|18_nodes"] #["50tps|6_nodes", "40tps|6_nodes", "30tps|6_nodes", "50tps|12_nodes", "40tps|12_nodes", "30tps|12_nodes"] #"50tps", "40tps", "30tps"
filter_out_reverse=False
export_data_path = "./datas_csv_paper_ready/"

export_img_figs = True #save img to file if true
export_img_filename = "./img/" + sys.argv[1] if len(sys.argv)>1 else None #"./img/18_nodes" # change as desired


if not os.path.exists(export_data_path):
    os.makedirs(export_data_path)
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

    # #https://stackoverflow.com/a/53380374/13187605
    # keep_same = {'time'} #keep time column 
    # all_df[field_name].columns = ['{}{}'.format(c, '' if c in keep_same else '_' + field_name) for c in df.columns]

print(all_df[next(iter(all_df))].columns.values)
#%%
#
# Get highest index
#

longest_data_length=0
all_df_iter=iter(all_df)
for field_key in all_df_iter:
    if longest_data_length< len(all_df[field_key]):
        longest_data_length = len(all_df[field_key])
        
#%%
#
# Make new index to scale all graphs
#

all_df_iter=iter(all_df)
for field_name in all_df_iter:
    #create scaled x axis for better comparaison
    all_df[field_name]["x_scaled"] = all_df[field_name].index.values*longest_data_length/len(all_df[field_name])
    
    #export for paper ready csv
    filename=export_data_path +field_name+".csv"
    all_df[field_name].to_csv(filename, index=False)


#%%

#Format for latex
def formatStrLatex(text):
    return text.replace('_', '\_')

def filter_out_all_with(text):
    for a in filter_out_all_with_elements:
        if a in text:
            return True if filter_out_reverse else False
    return False if filter_out_reverse else True
#%%
#
# Plot fft
#

# fig, axs = pl.subplots(figsize=conf_figsize)
# show="commits_rate"
# all_df_iter=iter(all_df)
# for field_name in all_df_iter:
#     if filter_out_all_with(field_name):
#         # Number of samplepoints
#         N = len(all_df[field_name][show].values) - 1
#         # sample spacing
#         T = 10 #group by 10s in influx #1.0 / 10

#         yf = scipy.fftpack.fft(all_df[field_name][show].values)
#         xf = scipy.fftpack.fftfreq(N, T)[:N//2]

#         axs.plot(xf, 2.0/N * np.abs(yf[0:N//2]), label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
#         break
# axs.set_title(formatStrLatex("FFT {}".format(show)))
# axs.legend()

#%%

# fig, axs = pl.subplots(2,1,figsize=conf_figsize)
# show="avg_tot_finished"
# all_df_iter=iter(all_df)
# for field_name in all_df_iter:
#     if filter_out_all_with(field_name):
#         axs[0].plot(all_df[field_name]["x_scaled"].values, butter_lowpass_filter(all_df[field_name][show].values, 0.008, 1/10, 2), label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
#         axs[0].set_title(formatStrLatex("{}".format(show)))
#         axs[0].legend()
   
# show="avg_tot_finished"
# all_df_iter=iter(all_df)
# for field_name in all_df_iter:
#     if filter_out_all_with(field_name):
#         axs[1].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
#         axs[1].set_title(formatStrLatex("{}".format(show)))
#         axs[1].legend()

#%%

#
# Print stats used in gnuplots
#

print("tps,nodes,avg_finished_rate mean,avg_finished_rate max,avg_finished_rate var,avg_invalid_rate mean,avg_tot_finished max")
show="avg_finished_rate"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        print("{},{},{:.1f},{:.1f},{:.1f},{:.1f},{:.1f}".format(field_name.split('|')[0].replace("tps",""), field_name.split('|')[1].replace("_nodes",""), np.mean(all_df[field_name]["avg_finished_rate"].values), np.max(all_df[field_name]["avg_finished_rate"].values), np.var(all_df[field_name]["avg_finished_rate"].values), np.mean(all_df[field_name]["avg_invalid_rate"].values), np.max(all_df[field_name]["avg_tot_finished"].values)))

exit()
#%%

fig, axs = pl.subplots(2,2,figsize=conf_figsize)
show="avg_finished_rate"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[0][0].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[0][0].set_title(formatStrLatex("{}".format(show)))
        axs[0][0].legend()

show="avg_queue_size"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[1][0].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[1][0].set_title(formatStrLatex("{}".format(show)))
        axs[1][0].legend()

show="block_construct_time"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[1][1].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[1][1].set_title(formatStrLatex("{}".format(show)))
        axs[1][1].legend()

show="avg_invalid_rate"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[0][1].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[0][1].set_title(formatStrLatex("{}".format(show)))
        axs[0][1].legend()

if export_img_figs:
    fig.savefig(export_img_filename + "_1")

fig, axs = pl.subplots(2,2,figsize=conf_figsize)
show="avg_forks"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[0][0].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[0][0].set_title(formatStrLatex("{}".format(show)))
        axs[0][0].legend()

show="avg_finalized_block_per_min"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[1][0].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[1][0].set_title(formatStrLatex("{}".format(show)))
        axs[1][0].legend()

show="avg_block_per_min"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[1][1].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[1][1].set_title(formatStrLatex("{}".format(show)))
        axs[1][1].legend()

show="avg_rpc_calls_rate"
all_df_iter=iter(all_df)
for field_name in all_df_iter:
    if filter_out_all_with(field_name):
        axs[0][1].plot(all_df[field_name]["x_scaled"].values, all_df[field_name][show].values, label=formatStrLatex("{}".format(field_name.replace(show+'_',''))))
        axs[0][1].set_title(formatStrLatex("{}".format(show)))
        axs[0][1].legend()

if export_img_figs:
    fig.savefig(export_img_filename + "_2")

# pl.show()