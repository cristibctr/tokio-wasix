use std::fmt;
use std::path::Path;

/// An address associated with a Tokio Unix socket.
///
/// This type is a thin wrapper around [`std::os::unix::net::SocketAddr`]. You
/// can convert to and from the standard library `SocketAddr` type using the
/// [`From`] trait.
#[cfg(unix)]
pub struct SocketAddr(pub(super) std::os::unix::net::SocketAddr);
#[cfg(target_vendor = "wasmer")]
pub struct SocketAddr(pub(super) std::os::wasi::net::SocketAddr);

impl SocketAddr {
    /// Returns `true` if the address is unnamed.
    ///
    /// Documentation reflected in [`SocketAddr`]
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn is_unnamed(&self) -> bool {
        self.0.is_unnamed()
    }

    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// Documentation reflected in [`SocketAddr`]
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn as_pathname(&self) -> Option<&Path> {
        self.0.as_pathname()
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

#[cfg(unix)]
impl From<std::os::unix::net::SocketAddr> for SocketAddr {
    fn from(value: std::os::unix::net::SocketAddr) -> Self {
        SocketAddr(value)
    }
}
#[cfg(target_vendor = "wasmer")]
impl From<std::os::wasi::net::SocketAddr> for SocketAddr {
    fn from(value: std::os::wasi::net::SocketAddr) -> Self {
        SocketAddr(value)
    }
}

#[cfg(unix)]
impl From<SocketAddr> for std::os::unix::net::SocketAddr {
    fn from(value: SocketAddr) -> Self {
        value.0
    }
}

#[cfg(target_vendor = "wasmer")]
impl From<SocketAddr> for std::os::wasi::net::SocketAddr {
    fn from(value: SocketAddr) -> Self {
        value.0
    }
}
