use std::fmt::{Debug, Display};

use bevy::app::AppExit::Success;
enum LogKindEnum {
	Error(&'static str),
	Success(&'static str),
	Warn(&'static str),
	Info(&'static str),
}

pub struct Logger;

impl Logger {
	const YELLOW: &str = "33";
	const RED: &str = "31";
	const BLUE: &str = "32";
	const GREEN: &str = "34";
    const GRAY:&str = "35";

	pub fn panic(message: impl Into<String>)
	where
		String: Debug,
	{
		panic!("\x1b[31m [ERR]: {:#?} \x1b[0m", message.into());
	}

	pub fn error(mssg: &'static str) {
		Self::_log(Self::RED, mssg)
	}
	pub fn success(mssg: &'static str) {
		Self::_log(Self::GREEN, mssg)
	}
	pub fn warn(mssg: &'static str) {
		Self::_log(Self::YELLOW, mssg)
	}
	pub fn info(mssg: &'static str) {
		Self::_log(Self::BLUE, mssg)
	}

	pub fn log(mssg: &'static str) {
		Self::_log(Self::GRAY, mssg)
	}

	fn _log(color: &'static str, mssg: &'static str) {
		println!("\x1b[{}m[ERR]: {}\x1b[0m", color, mssg)
	}
}
