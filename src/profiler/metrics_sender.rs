/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : profiler/metrics_sender.rs

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
    This module will provide data structures and functions to send metrics data
    to the receiver.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc;

use crate::profiler;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct MetricsSender {
    metrics_tx: mpsc::Sender<profiler::MetricContainer>,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl MetricsSender {
    /// Fully-qualified constructor
    pub fn new(metrics_tx: mpsc::Sender<profiler::MetricContainer>) -> Self {
        Self { metrics_tx }
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    /// Sends a metric to be recorded by the receiver
    pub fn send_metric(
        &self,
        metric: profiler::MetricContainer,
    ) -> Result<(), mpsc::SendError<profiler::MetricContainer>> {
        self.metrics_tx.send(metric)
    }
}
