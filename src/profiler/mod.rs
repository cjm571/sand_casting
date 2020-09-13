/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : profiler/mod.rs

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
    This module will provide data structures and functions that provide
    performance profiling functionality.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::{
    sync::mpsc,
    thread
};

use ggez::{
    Context as GgEzContext,
    timer as ggez_timer,
};


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

pub mod metrics_sender;
use self::metrics_sender::MetricsSender;
pub mod metrics_receiver;
use self::metrics_receiver::MetricsReceiver;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Enumeration for the various kinds of performance metrics that can be recorded.
pub enum MetricContainer {
    AvgFps(f64),
    FrameDeltaTime(f64),
    DrawDeltaTime(f64),
    UpdateDeltaTime(f64),
    CustomDeltaTime(String, f64),
}

#[derive(Default)]
struct CachedMetrics {
    pub avg_fps:    f64,
}

pub struct Instance {
    sender:         MetricsSender,
    cached_metrics: CachedMetrics,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Instance {

    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */
    
    pub fn avg_fps(&self) -> f64 {
        self.cached_metrics.avg_fps
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    pub fn update_avg_fps(&mut self, ggez_ctx: &GgEzContext) -> Result<(), mpsc::SendError<MetricContainer>> {
        // Update cached avg. FPS
        self.cached_metrics.avg_fps = ggez_timer::fps(ggez_ctx);
        
        // Pack up FPS in a container and send
        let metric = MetricContainer::AvgFps(self.cached_metrics.avg_fps);
        self.sender.send_metric(metric)
    }
}



///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for Instance {
    fn default() -> Self {
        // Create the metrics data channel
        let (metrics_tx, metrics_rx) = mpsc::channel::<MetricContainer>();

        //OPT: *PERFORMANCE* Would be better to set the receiver thread's priority as low as possible
        // Initialize receiver struct, build and spawn thread
        let mut metrics_receiver = MetricsReceiver::new(metrics_rx);
        thread::Builder::new()
            .name("metrics_receiver".to_owned())
            .spawn(move || metrics_receiver.main())
            .unwrap();

        Self {
            sender:         MetricsSender::new(metrics_tx),
            cached_metrics: CachedMetrics::default(),
        }
    }
}