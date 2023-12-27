static mut PACKAGE_MANAGER: Option<Box<dyn PackageManager>> = None;

pub trait PackageManager: Send + Sync {
    fn has_dist_upgrade(&self) -> bool;
    fn get_dist_upgrade_version(&self) -> &String;
    fn list_updates(&self) -> &Vec<Package>;
    fn os_release(&self) -> &String;
    fn reboot(&self) -> ();
    fn os_name(&self) -> &String;
    fn has_update(&self) -> bool;
    fn get_dist_update_command(&self) -> Vec<&str>;
    fn get_update_command(&self) -> Vec<&str>;

    fn dist_update_msg(&self) -> String;
    fn dist_update_desertion(&self) -> String;
}

pub fn package_manager() -> &'static dyn PackageManager {
    unsafe {
        PACKAGE_MANAGER
            .as_ref()
            .expect("Package manager not set")
            .as_ref()
    }
}

pub fn set_package_manager<T: PackageManager + 'static>(package_manager: T) {
    unsafe {
        PACKAGE_MANAGER = Some(Box::new(package_manager));
    }
}

pub struct Package {
    pub name: String,
    pub version: String,
}
