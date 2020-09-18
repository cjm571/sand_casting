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

use std::{
    fs,
    io::prelude::*,
    path::PathBuf,
    sync::mpsc,
    time::Duration,
};

use crate::profiler;

use chrono::Local;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct MetricsReceiver {
    metrics_rx: mpsc::Receiver<profiler::MetricContainer>,
    files:      Vec<fs::File>,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl MetricsReceiver {
    /// Generic constructor
    pub fn new(metrics_rx: mpsc::Receiver<profiler::MetricContainer>) -> Self {
        let mut files = Vec::new();
        Self::create_files(&mut files);
        
        Self {
            metrics_rx,
            files,
        }
    }


    /*  *  *  *  *  *  *  *
     *  Accessor Methods  *
     *  *  *  *  *  *  *  */

    fn file_handle(&mut self, metric: &profiler::MetricContainer) -> &mut fs::File {
        &mut self.files[usize::from(metric)]
    }
     

    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    /// Main loop for receiving and recording metrics data
    pub fn main(&mut self) {
        println!("{}: Entered MetricsReceiver thread.", Local::now().format("%Y-%m-%d %T%.3f"));

        loop {
            // Check channel for metrics
            if let Ok(metric_container) = self.metrics_rx.recv() {
                // Get the appropriate file handle
                let file_handle = self.file_handle(&metric_container);

                // Handle metric based on container type
                match metric_container {
                    profiler::MetricContainer::AvgFps(timestamp, avg_fps) => {
                        Self::add_f64_to_csv(timestamp, avg_fps, 0, file_handle);
                    }
                    profiler::MetricContainer::FrameDeltaTime(timestamp, delta) => {
                        Self::add_f64_to_csv(timestamp, delta, 7, file_handle);
                    },
                    profiler::MetricContainer::EventMarker(timestamp, event_label) => {
                        Self::add_string_to_csv(timestamp, event_label, file_handle);
                    },
                };
            }
        }
    }
    

    /*  *  *  *  *  *  *
     * Helper Methods  *
     *  *  *  *  *  *  */

    fn create_files(files: &mut Vec<fs::File>) {
        let start_time = Local::now();
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

        //OPT: *DESIGN* Would be cleaner if this were an iterator
        // Create standard metrics files
        for metric_idx in 0 .. profiler::MetricContainer::VARIANT_COUNT {
            // Get the current metric's filename
            let filename = profiler::MetricContainer::from(metric_idx).filename();

            // Push onto the filepath buffer and create the file
            metrics_path_buf.push(filename);
            match fs::File::create(metrics_path_buf.as_path()) {
                Ok(file) => files.push(file),
                Err(err) => panic!("Failed to create metrics file at {}. Error: {}", metrics_path_buf.as_path().display(), err),
            }

            // Pop the filename off the path buffer for the next iteration
            metrics_path_buf.pop();
        }
    }

    fn add_f64_to_csv(timestamp: Duration, item: f64, precision: usize, csv_file: &mut fs::File) {
        // Format item for writing
        let item_formatted = format!(
            "{timestamp},{item:.precision$};",
            timestamp = timestamp.as_millis(),
            item = item,
            precision = precision
        );

        // Write to given file
        csv_file.write_all(item_formatted.as_bytes()).unwrap();
    }

    fn add_string_to_csv(timestamp: Duration, label: String, csv_file: &mut fs::File) {
        // Format label for writing
        let label_formatted = format!(
            "{timestamp},{label};",
            timestamp = timestamp.as_millis(),
            label = label
        );

        // Write to given file
        csv_file.write_all(label_formatted.as_bytes()).unwrap();
    }
}
