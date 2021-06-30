use std::any::{Any};
use std::process::{Child, ChildStdin, ChildStdout, ChildStderr};
use std::io::{ErrorKind, Write, Read};

pub type ResourceError<T = ()> = Result<T, ErrorKind>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResourceKind {
    Io,
    Child,
    None
}

pub trait Resource: Any + 'static {
    fn kind(&self) -> ResourceKind;
    fn close(&self) -> ResourceError { 
        Ok(()) 
    }
}

pub trait IoResource: Resource + Any + 'static {
    fn read(&self, _buf: &mut [u8]) -> ResourceError<usize> { 
        Err(ErrorKind::Interrupted)
    }

    fn write(&self, _buf: &[u8]) -> ResourceError<usize> {
        Err(ErrorKind::Interrupted)
    }

    fn flush(&self) -> ResourceError {
        Err(ErrorKind::Interrupted)
    }
}

pub struct ChildResource(pub Box<Child>);
pub struct ChildStdinResource(pub Box<ChildStdin>);
pub struct ChildStdoutResource(pub Box<ChildStdout>);
pub struct ChildStderrResource(pub Box<ChildStderr>);

impl Resource for ChildResource {
    fn kind(&self) -> ResourceKind {
        ResourceKind::Child
    }

    fn close(&self) -> ResourceError {
        match unwrap_ref_as_mut(self.0.as_ref()).kill() {
            Ok(_) => Ok(()),
            Err(error) => Err(error.kind())
        }
    }
}

impl Resource for ChildStdinResource {
    fn kind(&self) -> ResourceKind {
        ResourceKind::Io
    }
}

impl Resource for ChildStdoutResource {
    fn kind(&self) -> ResourceKind {
        ResourceKind::Io
    }
}

impl Resource for ChildStderrResource {
    fn kind(&self) -> ResourceKind {
        ResourceKind::Io
    }
}

impl IoResource for ChildStdinResource {
    fn write(&self, buf: &[u8]) -> ResourceError<usize> {
        match unwrap_ref_as_mut(self.0.as_ref()).write(buf) {
            Ok(n) => Ok(n),
            Err(error) => Err(error.kind())
        }
    }

    fn flush(&self) -> ResourceError {
        match unwrap_ref_as_mut(self.0.as_ref()).flush() {
            Ok(_) => Ok(()),
            Err(error) => Err(error.kind())
        }
    }
}

impl IoResource for ChildStdoutResource {
    fn read(&self, buf: &mut [u8]) -> ResourceError<usize> {
        match unwrap_ref_as_mut(self.0.as_ref()).read(buf) {
            Ok(n) => Ok(n),
            Err(error) => Err(error.kind())
        }
    }
}

impl IoResource for ChildStderrResource {
    fn read(&self, buf: &mut [u8]) -> ResourceError<usize> {
        match unwrap_ref_as_mut(self.0.as_ref()).read(buf) {
            Ok(n) => Ok(n),
            Err(error) => Err(error.kind())
        }
    }
}

fn unwrap_ref_as_mut<'a, T>(ref_: &T) -> &'a mut T {
    unsafe { &mut *(ref_ as *const T as *mut T) }
}