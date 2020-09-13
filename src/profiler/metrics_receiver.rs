/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : profiler/metrics_receiver.rs

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
    This module will provide data structures and functions to recieve and
    record metrics data.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc;

use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;

use crate::profiler;

use chrono::{
    DateTime,
    Local
};

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct MetricsReceiver {
    metrics_rx: mpsc::Receiver<profiler::MetricContainer>,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl MetricsReceiver {
    /// Generic constructor
    pub fn new(metrics_rx: mpsc::Receiver<profiler::MetricContainer>) -> Self {
        Self {metrics_rx}
    }

    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    /// Main loop for receiving and recording metrics data
    pub fn main(&mut self) {
        let start_time = Local::now();
        println!("{}: Entered MetricsReceiver thread.", start_time.format("%Y-%m-%d %T%.3f"));

        //FIXME: Genericize this by returning a tuple or something
        let mut avg_fps_file = Self::set_up_metrics_dir(start_time);

        loop {
            // Check channel for metrics
            if let Ok(metric_container) = self.metrics_rx.recv() {
                // Handle metric based on container type
                match metric_container {
                    profiler::MetricContainer::AvgFps(avg_fps) => {
                        Self::add_f64_to_csv(avg_fps, &mut avg_fps_file);
                    },
                    _ => {},
                };
            }
        }
    }
    

    /*  *  *  *  *  *  *
     * Helper Methods  *
     *  *  *  *  *  *  */

    //FIXME: Genericize this by returning a tuple or something
    fn set_up_metrics_dir(start_time: DateTime<Local>) -> fs::File {
        let metrics_tld = "metrics";
        let metrics_cur = format!("{}", start_time.format("%F_%H_%M_%S%.3f"));

        // Create top-level 'metrics' directory if necessary
        let mut metrics_path_buf = PathBuf::from(metrics_tld);
        if !metrics_path_buf.as_path().exists() {
            match fs::create_dir(metrics_path_buf.as_path()) {
                Ok(()) => (),
                Err(e) => panic!("Failed to create top-level metrics directory. Error: {}", e),
            }
        }

        // Create directory for current run
        metrics_path_buf.push(metrics_cur);
        match fs::create_dir(metrics_path_buf.as_path()) {
            Ok(()) => (),
            Err(e) => panic!("Failed to create current-run metrics directory. Error: {}", e),
        }

        //OPT: *DESIGN* Iterate through collection of standard metrics?
        // Create standard metrics files
        metrics_path_buf.push("avg_fps.csv");
        match fs::File::create(metrics_path_buf.as_path()) {
            Ok(file) => file,
            Err(err) => panic!("Failed to create metrics file at {}. Error: {}", metrics_path_buf.as_path().display(), err),
        }
    }

    fn add_f64_to_csv(item: f64, csv_file: &mut fs::File) {
        // Format item for writing
        let item_formatted = format!("{:.0},", item);

        // Write to given file
        csv_file.write_all(item_formatted.as_bytes()).unwrap();
    }
}


