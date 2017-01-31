// Copyright (c) 2017 Nick Stevens <nick@bitcurry.com>

use errors::*;
use super::{SysfsLed, SysfsRgbLed};

pub trait TriggerNone {
    fn none(&mut self) -> Result<()>;
}

impl TriggerNone for SysfsLed {
    fn none(&mut self) -> Result<()> {
        self.sysfs_write_file("trigger", "none")
    }
}

impl TriggerNone for SysfsRgbLed {
    fn none(&mut self) -> Result<()> {
        self.red.sysfs_write_file("trigger", "none")
            .and(self.green.sysfs_write_file("trigger", "none"))
            .and(self.blue.sysfs_write_file("trigger", "none"))
    }
}

pub trait TriggerTimer {
    fn timer(&mut self, delay_on: u64, delay_off: u64) -> Result<()>;
}

impl TriggerTimer for SysfsLed {
    fn timer(&mut self, delay_on: u64, delay_off: u64) -> Result<()> {
        self.sysfs_write_file("trigger", "timer")
            .and(self.sysfs_write_file("delay_on", &format!("{}", delay_on)))
            .and(self.sysfs_write_file("delay_off", &format!("{}", delay_off)))
    }
}

pub trait TriggerHeartbeat {
    fn heartbeat(&mut self, invert: bool) -> Result<()>;
}

impl TriggerHeartbeat for SysfsLed {
    fn heartbeat(&mut self, invert: bool) -> Result<()> {
        self.sysfs_write_file("trigger", "heartbeat")
            .and(self.sysfs_write_file("invert", if invert { "1" } else { "0" }))
    }
}

pub trait TriggerCpu {
    fn cpu(&mut self, cpu: u32) -> Result<()>;
}

impl TriggerCpu for SysfsLed {
    fn cpu(&mut self, cpu: u32) -> Result<()> {
        self.sysfs_write_file("trigger", &format!("cpu{}", cpu))
    }
}
