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
from scipy.ndimage.filters import gaussian_filter1d
from datetime import datetime
from scipy.stats import linregress

#%%
#global variables:
export_data_path = "./datas_csv/"
export_data_filename=sys.argv[1] if len(sys.argv)>1 else "6_nodes|5tps"
data_path = "./datas/"
image_directory="./images/"
final_df = None
conf_figsize=(15,8)
#detect_benchmarks_on="commits"
detect_benchmarks_on="avg_finished_rate"

#jump of N detect_benchmarks_on (= change of test)
detect_benchmark_threshold=10
detect_benchmark_elements=5

#~300 blocks for init in our case:
detect_benchmark_start=False
initialization_threshold=150

#benchmark ended if consecutive elements are equal.
detect_benchmark_stop=False
#Delete all data between start of detected consecutive element and jump of detect_benchmark_threshold
#Using 2, stop detected when 2 consecutive elements are strictly equal
#Note: minimum=2, recommended=4
detect_benchmark_stop_elements=10 #use something low (<5)
detect_benchmark_stop_elements_std=0.4 #use something low (<0.5)
detect_benchmark_stop_previous_elements=0

if not os.path.exists(image_directory):
    os.makedirs(image_directory)
    
if not os.path.exists(export_data_path):
    os.makedirs(export_data_path)
#%%
#
# Get all CSV files and put it inside dataframes
#
number_of_benchmarks=0

all_files = glob.glob(data_path + "/*.csv")
tmp_df = {}
for filename in all_files:
    df = pd.read_csv(filename, index_col=None, header=0, usecols=["time", "mean"],)
    field_name = os.path.basename(filename)[3:-4] #remove extension and test nb
    test = os.path.basename(filename)[:2] #get test nb

    if test in tmp_df:
        tmp_df[test][field_name] = df
    else:
        number_of_benchmarks = number_of_benchmarks + 1
        tmp_df[test]={}
        tmp_df[test][field_name] = df
    
    #rename columns
    tmp_df[test][field_name].columns = ["time", field_name]
    tmp_df[test][field_name]["test#"] = int(test)


tmp_df_concated = {}
for test_key in sorted(tmp_df.keys()):
    for field_key in sorted(tmp_df[test_key].keys()):
        if field_key in tmp_df_concated:
            tmp_df_concated[field_key] = pd.concat([ tmp_df_concated[field_key], tmp_df[test_key][field_key]], ignore_index=True, sort=False, axis=0)
        else:
            tmp_df_concated[field_key] = tmp_df[test_key][field_key]

#%%
#
# Merge data into one table
#
final_df = tmp_df_concated[detect_benchmarks_on] #init with commits
for field_key in tmp_df_concated.keys():
    #skip first because already in final_df
    if field_key == detect_benchmarks_on:
        continue
    #merge tables
    #help, see https://pandas.pydata.org/docs/user_guide/merging.html#brief-primer-on-merge-methods-relational-algebra
    
    final_df = final_df.merge(tmp_df_concated[field_key], on="time", how="left")


#remove useless cols:
cols = [c for c in final_df.columns if 'test#_' not in c]
final_df=final_df[cols]


#fill NaN with previous value
final_df = final_df.fillna(method='ffill').fillna(0)

# pl.plot(final_df["time"], final_df[detect_benchmarks_on])

#%%
#
# Print available columns
#
print("Columns available are:")
final_df_cols=[] #contains the columns in the same order then final_df columns
for col in final_df.columns:
    print("{}, ".format(col), end='') #print the columns to know whats available
    final_df_cols.append(col)
print("")


#%%
#
# Filter out initialization of the test: only keep data where commits>1000
#

#detect benchmark start
#thanks to https://stackoverflow.com/a/27360130/13187605
if detect_benchmark_start:
    final_df = final_df.drop(final_df[(final_df[detect_benchmarks_on] >= 1) & (final_df[detect_benchmarks_on] <= initialization_threshold)].index)
    final_df = final_df.reset_index(drop=True)


print("Benchmark detected = {}".format(number_of_benchmarks))
#end benchmark detection


# print(final_df)
#%%
#remove all 5 consecutive zeros

# previous=0 #previous value to compare to
# duplicates=0
# for index, row in final_df.iterrows():
#     if row[col_detect_index] < 5 and previous == row[col_detect_index]:
#         duplicates=duplicates+1
#     if duplicates>=5:
#         for i in range(0, duplicates):
#             final_df = final_df.drop(index-i)
#         duplicates=0

# final_df = final_df.reset_index(drop=True)

#%%
#remove tests with only zeros

# previous=0 #previous value to compare to
# previous_test=1
# found_other_than_zero=False
# for index, row in final_df.iterrows():
#     if previous_test == row["test#"] and row[col_detect_index] != 0.0:
#         found_other_than_zero=True
    
#     if row[col_detect_index] == 0.0 and found_other_than_zero == False:
#         print(index)
#         final_df = final_df.drop(index)
#         if index>0:
#             final_df = final_df.drop(index-1)
    
#     if previous_test != row["test#"]:
#         found_other_than_zero=False

#     previous_test=row["test#"]

# final_df = final_df.reset_index(drop=True)
# number_of_benchmarks=int(final_df['test#'].max(axis=0))
# print("Benchmark detected = {}".format(number_of_benchmarks))


#%%
#
# Plot function
# (auto color and auto benchmark detection based on "test#" column)
#
#color map here:
cm_subsection = np.linspace(0.0, 1.0, number_of_benchmarks+1)
colors = [ cm.jet(x) for x in cm_subsection ]
#self inc on each myplot() call:
total_plots=0
#general plot fct to make it simple:
def myplot(plot_type, X_colomn_name, Y_colomn_name, display=False, smooth = False):
    global total_plots
    #help on legend placement here: https://stackoverflow.com/a/4701285/13187605
#    pl.figure(total_plots)
    fig, ax = pl.subplots(1,1,figsize=conf_figsize)
    for i in range(1, number_of_benchmarks+1):
        #print lines for each test, using diff colors
        if X_colomn_name == "index":
            X_values=final_df.loc[final_df['test#'] == i].index
        else:
            X_values=final_df.loc[final_df['test#'] == i].values[:,final_df_cols.index(X_colomn_name)]
        Y_values=final_df.loc[final_df['test#'] == i].values[:,final_df_cols.index(Y_colomn_name)]
        if smooth:
            Y_values= gaussian_filter1d(Y_values, sigma=1) # make more smooth: BE CARFUL
        if plot_type == "line":
            ax.plot(X_values,Y_values, color=colors[i], label="Test#{}".format(i))
        elif plot_type == "scatter":
            ax.scatter(X_values,Y_values, color=colors[i], label="Test#{}".format(i))
        elif plot_type == "bar":
            if len(X_values) == 0: #fix an value error create by "bar" if empty data
                X_values= [0]
                Y_values= [0]
            ax.bar(X_values, Y_values, color=colors[i], alpha=0.5, label="Test#{}".format(i))
            # pl.hist(Y_values, color=colors[i], label="Test#{} ({})".format(i, Y_colomn_name))
        else:
            print("ERROR: Unknown plot type: {}".format(plot_type))
            exit(1)
    ax.set_title("f({})={}".format(X_colomn_name, Y_colomn_name))
    ax.set_xlabel(X_colomn_name)
    ax.set_ylabel(Y_colomn_name)
    box = ax.get_position()
    ax.set_position([box.x0, box.y0, box.width * 0.8, box.height])
    ax.legend(loc='center left', bbox_to_anchor=(1, 0.5))
    pl.savefig(image_directory + "X=" + X_colomn_name + "|Y=" + Y_colomn_name + '.png', bbox_inches='tight', dpi=200)
    total_plots+=1
    if not display:
        pl.close(fig)

def getVariance(data):
    # info on axis selection: https://stackoverflow.com/a/46223968/13187605
    #Axis 1 will act on all the COLUMNS in each ROW
    #Axis 0 will act on all the ROW in each COLUMNS
    return data.mean(axis=0).var(axis=0,ddof=1)
def getStd(data):
    # info on axis selection: https://stackoverflow.com/a/46223968/13187605
    #Axis 1 will act on all the COLUMNS in each ROW
    #Axis 0 will act on all the ROW in each COLUMNS
    return data.mean(axis=0).std(axis=0,ddof=1)

def myplot_merged(X_colomn_name, Y_colomn_name, display=False):
    global total_plots

    #Note: X_colomn_name should be time !!

    #Some results of:
    #Variance: describes how much a random variable differs from its expected value.
    #Standard deviation is the measure of dispersion of a set of data from its mean.
    
    Y_colomn_merged_array = merge_tests_data_from(Y_colomn_name).values
    Y_colomn_merged_array_mean = Y_colomn_merged_array.mean(axis=1)

    X_colomn_merged_array = merge_tests_data_from(X_colomn_name).values.mean(axis=1) #avg times
    # utcfromtimestamp need 10 digit timestamp
    start=datetime.utcfromtimestamp(X_colomn_merged_array[0]) #/1e9
    end=datetime.utcfromtimestamp(X_colomn_merged_array[-1]) #/1e9
    avg_time_sec=(end-start).total_seconds()
    avg_time_min=avg_time_sec/60
    
    #show some data
#    pl.figure(total_plots)
    total_plots+=1
    fig, axs = pl.subplots(2,1,figsize=conf_figsize)
    i=1
    for col_arr in Y_colomn_merged_array.T:
        axs[0].plot(col_arr, label="Test {}".format(i))
        i+=1
    axs[0].plot(Y_colomn_merged_array_mean, label="Mean of all tests")
    
    axs[0].plot([], [], ' ', label="Mean={:.2f}".format(Y_colomn_merged_array.mean()))
    axs[0].plot([], [], ' ', label="Variance={:.2f} (?)".format(getVariance(Y_colomn_merged_array)))
    axs[0].plot([], [], ' ', label="Std={:.2f}".format(getStd(Y_colomn_merged_array)))
    axs[0].plot([], [], ' ', label="Time={:.1f}min".format(avg_time_min))
    axs[0].set_title("Tests with mean ({})".format(Y_colomn_name))
    axs[0].legend()
    

#    ax.plot(merged_array.mean(axis=1)-merged_array.var(axis=1,ddof=1), label="variance of all tests")
    axs[1].plot(Y_colomn_merged_array.var(axis=1,ddof=1), label="f({})=Var({})".format(X_colomn_name, Y_colomn_name))
    axs[1].plot(Y_colomn_merged_array.std(axis=1,ddof=1), label="f({})=Std({})".format(X_colomn_name, Y_colomn_name))
    axs[1].legend()
    pl.savefig(image_directory + "merged|X=" + X_colomn_name + "|Y=" + Y_colomn_name + '.png', bbox_inches='tight', dpi=200)
    if not display:
        pl.close(fig)
#%%
#
# Merge function of multiple signals
#

def merge_tests_data_from(colomn_name):
    tmp={}
    for i in range(1, number_of_benchmarks+1):
        values=final_df.loc[final_df['test#'] == i].values[:,final_df_cols.index(colomn_name)]
        tmp2= {}
        tmp2[i]=values
        tmp[i]=pd.DataFrame(tmp2)
    #for help, see https://pandas.pydata.org/docs/user_guide/merging.html
    temp_elements = pd.concat(tmp, axis=1, join="outer")
    temp_elements = temp_elements.fillna(method='ffill')
    return temp_elements

def merge_and_mean_tests_data_from(colomn_name):
    d=merge_tests_data_from(colomn_name).values.mean(axis=1) #mean all rows
    return d

#%%
#
# Some fix on time values
# And export to csv
from datetime import datetime
if len(sys.argv)>1:
    #print(final_df)
    print("Data shape: {}".format(final_df.shape))
    filename=export_data_path +export_data_filename+".csv"
    # print(final_df[['time']])
    # final_df[['time']] = (df[['time']]).astype('int64')#/1e9
    # print(final_df[['time']])
    final_df.to_csv(filename, index=False)
    print("Data exported to '{}'".format(filename))

    #merge and export all merged data:
    final_merged_df = {
            "time": merge_and_mean_tests_data_from("time").astype(int) #init with time
            }
    for col in final_df_cols:
            if col != "time":
                final_merged_df[col] = merge_and_mean_tests_data_from(col)
    final_merged_df = pd.DataFrame(final_merged_df) #to dataframe
    final_merged_df["test_name"]=export_data_filename #set test name in cas we need it
    print("Merged shape: {}".format(final_merged_df.values.shape))
    filename=export_data_path +"merged_"+export_data_filename+".csv"
    final_merged_df.to_csv(filename, index=False)
    print("Merged data exported to '{}'".format(filename))
# exit() #for debug

#%%
#
# Start plotting from here
#

myplot("line", "time", "avg_finished_rate", True, False)
myplot("line", "time", "avg_invalid_rate", True, True)
#myplot("scatter", "index", "commits", True, True)
#myplot("scatter", "time", "commits", True, True)
#myplot("scatter", "commits_rate", "rest_api_batch_rate", True, False)

# myplot_merged("time", "avg_finished_rate", True)
#myplot_merged("time", "tx_exec_rate", True)

#%%
#
# Generate all image data posible
# (CARFUL: creates ~100 images)
#
#for col in final_df_cols:
#    for col2 in final_df_cols:
#        if col != col2 and col2 != "time":
#            print("Generating f({})={}".format(col, col2))
#            myplot("line",col, col2)

#for col in final_df_cols:
#        if col != "time":
#            print("Generating f({})={}".format("time", col))
#            myplot_merged("time", col)

# pl.show()