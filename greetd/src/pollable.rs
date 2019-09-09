use std::cell::RefCell;
use std::error::Error;
use std::os::unix::io::RawFd;
use std::rc::Rc;

use nix::poll::PollFlags;

use crate::context::Context;

pub enum PollRunResult {
    Uneventful,
    Dead,
    NewPollable(Rc<RefCell<Box<dyn Pollable>>>),
}

pub trait Pollable {
    fn fd(&self) -> RawFd;
    fn poll_flags(&self) -> PollFlags;
    fn run(&mut self, ctx: &mut Context) -> Result<PollRunResult, Box<dyn Error>>;
}
