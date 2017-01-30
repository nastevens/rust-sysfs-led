// Copyright (c) 2017 Nick Stevens <nick@bitcurry.com>

use errors::*;
use super::{SysfsLed, SysfsRgbLed};

pub trait TriggerNone {
    fn none(&mut self) -> Result<()>;
}

pub trait TriggerTimer {
    fn timer(&mut self, delay_on: u64, delay_off: u64) -> Result<()>;
}

pub trait TriggerHeartbeat {
    fn heartbeat(&mut self, invert: bool) -> Result<()>;
}

pub trait TriggerCpu {
    fn cpu(&mut self, cpu: u32) -> Result<()>;
}

impl TriggerNone for SysfsLed {
    fn none(&mut self) -> Result<()> {
        self.sysfs_write_file("trigger", "none")
    }
}

impl TriggerNone for SysfsRgbLed {
    fn none(&mut self) -> Result<()> {
        self.red.sysfs_write_file("trigger", "none")
            .and_then(|()| self.green.sysfs_write_file("trigger", "none"))
            .and_then(|()| self.blue.sysfs_write_file("trigger", "none"))
    }
}
