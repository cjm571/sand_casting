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
    thread,
    time::Duration,
};

use ggez::{
    Context as GgEzContext,
    timer as ggez_timer,
};


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Number of MetricContainer enum variants
pub const METRIC_CONTAINER_TYPE_COUNT: usize = 5;


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
    AvgFps(Duration, f64),
    FrameDeltaTime(Duration, f64),
    DrawDeltaTime(Duration, f64),
    UpdateDeltaTime(Duration, f64),
    CustomDeltaTime(Duration, String, f64),
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
     *  Accesspr Methods  *
     *  *  *  *  *  *  *  */
    
    pub fn avg_fps(&self) -> f64 {
        self.cached_metrics.avg_fps
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    pub fn update_avg_fps(&mut self, ggez_ctx: &GgEzContext) -> Result<(), mpsc::SendError<MetricContainer>> {
        // Get elapsed time
        let elapsed_time = ggez_timer::time_since_start(ggez_ctx);
        
        // Update cached avg. FPS
        self.cached_metrics.avg_fps = ggez_timer::fps(ggez_ctx);
        
        // Pack up FPS in a container and send
        let metric = MetricContainer::AvgFps(elapsed_time, self.cached_metrics.avg_fps);
        self.sender.send_metric(metric)
    }

    pub fn send_frame_delta(&self, ggez_ctx: &GgEzContext) -> Result<(), mpsc::SendError<MetricContainer>> {
        // Get elapsed time
        let elapsed_time = ggez_timer::time_since_start(ggez_ctx);

        // Get frame delta and convert to f64
        let frame_delta = ggez_timer::duration_to_f64(ggez_timer::delta(ggez_ctx));

        // Pack up frame delta in a container and send
        let metric = MetricContainer::FrameDeltaTime(elapsed_time, frame_delta);
        self.sender.send_metric(metric)
    }
}



///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/*  *  *  *  *  *  *  *
 *      Instance      *
 *  *  *  *  *  *  *  */
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


/*  *  *  *  *  *  *  *
 *  MetricContainer   *
 *  *  *  *  *  *  *  */
impl From<MetricContainer> for usize {
    fn from(src: MetricContainer) -> Self {
        match src {
            MetricContainer::AvgFps(_dur, _val)                     => 0,
            MetricContainer::FrameDeltaTime(_dur, _val)             => 1,
            MetricContainer::DrawDeltaTime(_dur, _val)              => 2,
            MetricContainer::UpdateDeltaTime(_dur, _val)            => 3,
            MetricContainer::CustomDeltaTime(_dur, _label, _val)    => 4,
        }
    }
}