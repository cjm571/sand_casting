""" " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " "
Filename : tools/metrics_receiver.rs

Copyright (C) 2020 CJ McAllister
    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software Foundation,
    Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA

Purpose: 
    This file is used to parse the large CSV files generated by the
    SandCasting performance profiler.

" " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " " """

import csv
import sys

import matplotlib.pyplot as plt
import numpy as np

def usage():
    print("Usage: python profiler_plotter.py PROFILER_DATA_FILE1.csv [PROFILER_DATA_FILE2.csv]")


def parse_numerical_data(filename):
    # Retrieve float data from csv
    timestamps = []
    values = []

    # Open data file for parsing
    with open(filename) as csvDataFile:

        # Use ; as delimiter to expose the (timestamp, value) tuples
        csvReader = csv.reader(csvDataFile, delimiter=';')
        for row in csvReader:
            for data_tuple in row:
                # Break once we encounter an empty column
                if data_tuple == '':
                    break

                # Split data tuples on ','
                (timestamp, value) = data_tuple.split(',')

                # Determine data type and cast accordingly
                if '.' in timestamp:
                    timestamps.append(float(timestamp))
                else:
                    timestamps.append(int(timestamp))

                # Determine data type and cast accordingly
                if '.' in value:
                    values.append(float(value))
                else:
                    values.append(int(value))
    
    # Return collated data tuple
    return (timestamps, values)


def parse_string_data(filename):
    # Retrieve float data from csv
    timestamps = []
    dummy_vals = []
    labels = []

    # Open data file for parsing
    with open(filename) as csvDataFile:

        # Use ; as delimiter to expose the (timestamp, value) tuples
        csvReader = csv.reader(csvDataFile, delimiter=';')
        for row in csvReader:
            for data_tuple in row:
                # Break once we encounter an empty column
                if data_tuple == '':
                    break

                # Split data tuples on ','
                (timestamp, label) = data_tuple.split(',')

                # Determine data type and cast accordingly
                if '.' in timestamp:
                    timestamps.append(float(timestamp))
                else:
                    timestamps.append(int(timestamp))
                    
                # Add data to arrays
                labels.append(label)
                dummy_vals.append(1)
    
    # Return collated data tuple
    return (timestamps, labels, dummy_vals)


def populate_axis(axis, color, filepath):
    # Parse CSV file based on metric type
    filename = filepath.split('\\')[-1]

    if filename == "avg_fps.csv":
        axis.set_ylabel('Avg FPS', color=color)
        (timestamps, values) = parse_numerical_data(filepath)
        axis.plot(timestamps, values, color=color)
        
    elif filename == "frame_delta.csv":
        axis.set_ylabel('Frame Delta (sec)', color=color)
        (timestamps, values) = parse_numerical_data(filepath)
        axis.plot(timestamps, values, color=color)

    elif filename == "event_marker.csv":
        event_offset_dict = {}
        offset = 0.0
        (timestamps, labels, dummy_vals) = parse_string_data(filepath)

        # Use timestamps and dummy values for the bar chart
        axis.bar(timestamps, dummy_vals, color=color, width=0.5)
        axis.set_yticks([])


        # Annotate chart with event labels
        for i in range(len(timestamps)):
            # Assign offsets for each event type
            if labels[i] not in event_offset_dict:
                event_offset_dict[labels[i]] = offset
                offset += 0.1
            
            # Place "STOP" events slightly below "START" for readability
            if  "START" in labels[i]:
                axis.annotate(labels[i], xy=[timestamps[i], dummy_vals[i] - event_offset_dict.get(labels[i])])
            elif "STOP" in labels[i]:
                axis.annotate(labels[i], xy=[timestamps[i], dummy_vals[i] - event_offset_dict.get(labels[i]) - 0.05])
            else:
                axis.annotate(labels[i], xy=[timestamps[i], dummy_vals[i] - event_offset_dict.get(labels[i])])

    else:
        print("Invalid file provided:" + filepath)
        sys.exit(3)


if __name__ == "__main__":
    # Sanity check command-line arguments
    if len(sys.argv) < 2:
        usage()
        sys.exit(2)

    # Intialize chart boilerplate
    fig, ax0 = plt.subplots()
    color = 'tab:blue'
    ax0.set_xlabel('time (ms)')

    # Populate the first chart axis
    populate_axis(ax0, color, sys.argv[1])

    # Populate the second chart axis, if file provided
    if len(sys.argv) > 2:
        ax1 = ax0.twinx()
        color = 'tab:red'
        
        populate_axis(ax1, color, sys.argv[2])

    # Populate the third chart axis, if file provided
    if len(sys.argv) > 3:
        ax2 = ax0.twinx()
        color = 'tab:green'
        
        populate_axis(ax2, color, sys.argv[3])

    plt.show()