use crate::dnf::DnfPackageManager;
use std::collections::HashMap;

pub const FEDORA_COLLECTION_URL: &str = "https://admin.fedoraproject.org/pkgdb/api/collections/";
pub const APP_ID: &str = "dev.bedsteler20.FedoraUpdateUtil";

// Change this to use the package manager of your choice
pub type IPackageManager = DnfPackageManager;

pub fn load_img(by: &[u8]) -> gtk::gdk::Texture {
    let paint = gtk::gdk::Texture::from_bytes(&gtk::glib::Bytes::from(&by[..])).unwrap();
    return paint;
}

pub fn parse_os_release() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let os_release = std::fs::read_to_string("/etc/os-release");
    if os_release.is_err() {
        return map;
    }
    let os_release = os_release.unwrap();
    let os_release = os_release.split("\n");

    for line in os_release {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let line = line.split("=");
        if line.clone().count() != 2 {
            continue;
        }
        let mut line = line.collect::<Vec<&str>>();
        let key = line.remove(0).to_string();
        let mut value = line.remove(0).to_string();
        if value.starts_with("\"") && value.ends_with("\"") {
            value.remove(0);
            value.pop();
        }

        map.insert(key, value);
    }

    return map;
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Surf(surf::Error),
    IntParse(std::num::ParseIntError),
    Undraped(String),
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        Error::Surf(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::IntParse(err)
    }
}

pub trait UnwrapOr<T> {
    fn as_result(self, msg: &str) -> Result<T>;
}

impl<T> UnwrapOr<T> for Option<T> {
    fn as_result(self, msg: &str) -> Result<T> {
        match self {
            Some(val) => Ok(val),
            None => Err(Error::Undraped(msg.to_string())),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Surf(err) => write!(f, "Surf error: {}", err),
            Error::IntParse(err) => write!(f, "Int parse error: {}", err),
            Error::Undraped(msg) => write!(f, "Undraped error: {}", msg),
        }
    }
}


