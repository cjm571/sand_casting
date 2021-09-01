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

use std::{sync::mpsc, thread, time::Duration};

use ggez::{graphics as ggez_gfx, mint as ggez_mint, timer as ggez_timer, Context as GgEzContext};

use variant_count::VariantCount;

use crate::game_assets::colors;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Placeholder for bound Durations
pub const PLACEHOLDER_DURATION: Duration = Duration::from_secs(0);

/// Placeholder for bound f64s
pub const PLACEHOLDER_F64: f64 = 0.0;

/// Placeholder for bound Strings
pub const PLACEHOLDER_STRING: String = String::new();

/// Placeholder for bound Strings
pub const PLACEHOLDER_STACKED_DRAW_VEC: Vec<StackedTime> = Vec::new();


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

/// Instance of the SandCasting profiler module
#[derive(Clone)]
pub struct Instance {
    enabled: bool,
    sender: MetricsSender,
    cached_metrics: CachedMetrics,
}

#[derive(Clone, Default)]
struct CachedMetrics {
    pub avg_fps: f64,
    pub peak_fps: f64,
}

/// Enumeration for the various kinds of performance metrics that can be recorded.
#[derive(VariantCount)]
pub enum MetricContainer {
    AvgFps(Duration, f64),
    FrameDeltaTime(Duration, f64),
    EventMarker(Duration, String),
    StackedDrawTime(Duration, Vec<StackedTime>),
}

pub struct StackedTime {
    pub label: String,
    pub time: Duration,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementations
///////////////////////////////////////////////////////////////////////////////

impl Instance {
    //OPT: *DESIGN* Would be cool to make a Disablable trait
    pub fn disabled() -> Self {
        // Create dummy channel handles
        let (dummy_tx, _dummy_rx) = mpsc::channel::<MetricContainer>();

        // Initialize dummy sender struct
        let dummy_sender = MetricsSender::new(dummy_tx);

        Self {
            enabled: false,
            sender: dummy_sender,
            cached_metrics: CachedMetrics::default(),
        }
    }


    /*  *  *  *  *  *  *  *
     *  Accessor Methods  *
     *  *  *  *  *  *  *  */

    pub fn avg_fps(&self) -> f64 {
        self.cached_metrics.avg_fps
    }

    pub fn peak_fps(&self) -> f64 {
        self.cached_metrics.peak_fps
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    pub fn draw_fps_stats(&self, ggez_ctx: &mut GgEzContext) {
        //OPT: *PERFORMANCE* "static" storage of these local variables would probably be quicker
        // Draw avg. FPS
        let avg_fps_pos = ggez_mint::Point2 { x: 0.0, y: 0.0 };
        let avg_fps_str = format!("Avg. FPS: {:.0}", self.cached_metrics.avg_fps);
        let avg_fps_display = ggez_gfx::Text::new((
            avg_fps_str,
            ggez_gfx::Font::default(),
            crate::DEFAULT_TEXT_SIZE,
        ));
        ggez_gfx::draw(
            ggez_ctx,
            &avg_fps_display,
            (avg_fps_pos, 0.0, colors::GREEN),
        )
        .unwrap();

        // Draw peak FPS
        let peak_fps_pos = ggez_mint::Point2 { x: 0.0, y: 20.0 };
        let peak_fps_str = format!("Peak FPS: {:.0}", self.cached_metrics.peak_fps);
        let peak_fps_display = ggez_gfx::Text::new((
            peak_fps_str,
            ggez_gfx::Font::default(),
            crate::DEFAULT_TEXT_SIZE,
        ));
        ggez_gfx::draw(
            ggez_ctx,
            &peak_fps_display,
            (peak_fps_pos, 0.0, colors::GREEN),
        )
        .unwrap();
    }

    pub fn update_fps_stats(
        &mut self,
        ggez_ctx: &GgEzContext,
    ) -> Result<(), mpsc::SendError<MetricContainer>> {
        // Get elapsed time
        let elapsed_time = ggez_timer::time_since_start(ggez_ctx);

        // Update cached avg. FPS
        self.cached_metrics.avg_fps = ggez_timer::fps(ggez_ctx);

        // Update cached peak FPS if appropriate
        if self.cached_metrics.avg_fps > self.cached_metrics.peak_fps {
            self.cached_metrics.peak_fps = self.cached_metrics.avg_fps;
        }

        if self.enabled {
            // Pack up FPS in a container and send
            let metric = MetricContainer::AvgFps(elapsed_time, self.cached_metrics.avg_fps);
            self.sender.send_metric(metric)
        } else {
            Ok(())
        }
    }

    pub fn send_frame_delta(
        &self,
        ggez_ctx: &GgEzContext,
    ) -> Result<(), mpsc::SendError<MetricContainer>> {
        if self.enabled {
            // Get elapsed time
            let elapsed_time = ggez_timer::time_since_start(ggez_ctx);

            // Get frame delta and convert to f64
            let frame_delta = ggez_timer::delta(ggez_ctx).as_secs_f64();

            // Pack up frame delta in a container and send
            let metric = MetricContainer::FrameDeltaTime(elapsed_time, frame_delta);
            self.sender.send_metric(metric)
        } else {
            Ok(())
        }
    }

    pub fn mark_event(
        &self,
        event_label: String,
        ggez_ctx: &GgEzContext,
    ) -> Result<(), mpsc::SendError<MetricContainer>> {
        if self.enabled {
            // Get elapsed time
            let elapsed_time = ggez_timer::time_since_start(ggez_ctx);

            // Pack up event label in a container and send
            let metric = MetricContainer::EventMarker(elapsed_time, event_label);
            self.sender.send_metric(metric)
        } else {
            Ok(())
        }
    }

    pub fn send_stacked_draw_time(
        &self,
        start_time: Duration,
        stacked_times: Vec<StackedTime>,
    ) -> Result<(), mpsc::SendError<MetricContainer>> {
        if self.enabled {
            // Pack up stacked times into container and send
            let metric = MetricContainer::StackedDrawTime(start_time, stacked_times);
            self.sender.send_metric(metric)
        } else {
            Ok(())
        }
    }
}


impl MetricContainer {
    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    /// Returns the filename that will store the metric's data
    pub fn filename(&self) -> String {
        match self {
            MetricContainer::AvgFps(_dur, _val) => String::from("avg_fps.csv"),
            MetricContainer::FrameDeltaTime(_dur, _val) => String::from("frame_delta.csv"),
            MetricContainer::EventMarker(_dur, _label) => String::from("event_marker.csv"),
            MetricContainer::StackedDrawTime(_dur, _vec) => String::from("stacked_draw_time.csv"),
        }
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
            .name(String::from("metrics_receiver"))
            .spawn(move || metrics_receiver.main())
            .unwrap();

        Self {
            enabled: true,
            sender: MetricsSender::new(metrics_tx),
            cached_metrics: CachedMetrics::default(),
        }
    }
}


/*  *  *  *  *  *  *  *
 *  MetricContainer   *
 *  *  *  *  *  *  *  */
impl From<&MetricContainer> for usize {
    fn from(src: &MetricContainer) -> Self {
        match src {
            MetricContainer::AvgFps(_dur, _val) => 0,
            MetricContainer::FrameDeltaTime(_dur, _val) => 1,
            MetricContainer::EventMarker(_dur, _label) => 2,
            MetricContainer::StackedDrawTime(_dur, _vec) => 3,
        }
    }
}
impl From<usize> for MetricContainer {
    fn from(src: usize) -> Self {
        match src {
            0 => MetricContainer::AvgFps(PLACEHOLDER_DURATION, PLACEHOLDER_F64),
            1 => MetricContainer::FrameDeltaTime(PLACEHOLDER_DURATION, PLACEHOLDER_F64),
            2 => MetricContainer::EventMarker(PLACEHOLDER_DURATION, PLACEHOLDER_STRING),
            3 => {
                MetricContainer::StackedDrawTime(PLACEHOLDER_DURATION, PLACEHOLDER_STACKED_DRAW_VEC)
            }
            _ => panic!(
                "Invalid value ({}) for usize -> MetricContainer conversion",
                src
            ),
        }
    }
}
