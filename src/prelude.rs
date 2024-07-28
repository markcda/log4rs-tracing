//! Utils for fast imports.

pub use crate::append::console::ConsoleAppender;
pub use crate::append::file::FileAppender;
pub use crate::append::rolling_file::{
  RollingFileAppender,
  policy::compound::CompoundPolicy,
  policy::compound::{
    trigger::{
      Trigger,
      onstartup::{OnStartUpTrigger, OnStartUpTriggerConfig},
      size::{SizeTrigger, SizeTriggerConfig},
      time::{TimeTrigger, TimeTriggerConfig},
    },
    roll::{
      Roll,
      delete::{DeleteRoller, DeleteRollerConfig},
      fixed_window::{FixedWindowRoller, FixedWindowRollerConfig},
    },
  }
};
pub use crate::encode::pattern::PatternEncoder;
pub use crate::config::{Appender, Config, Root};
pub use crate::filter::threshold::ThresholdFilter;
