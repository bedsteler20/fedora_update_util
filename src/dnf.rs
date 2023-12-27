use crate::utils::{Result, UnwrapOr, FEDORA_COLLECTION_URL};
use async_process::Command;
use std::process::Command as SyncCommand;

use crate::{
    package_manager::{Package, PackageManager},
    utils::parse_os_release,
};

pub struct DnfPackageManager {
    packages: Vec<Package>,
    os_name: String,
    os_version: String,
    os_version_num: i32,
    dist_update_command: Vec<String>,
    update_command: Vec<String>,
    latest_version: FedoraUpdate,
}

impl DnfPackageManager {
    pub async fn init() -> Result<Self> {
        // get packages from dnf
        let packages = {
            let cmd = Command::new("dnf5")
                .arg("list")
                .arg("--updates")
                .output()
                .await
                .expect("Failed to execute command")
                .stdout;
            let output = String::from_utf8(cmd).unwrap();
            let lines = output.split("\n");
            let mut packages = Vec::new();
            for line in lines.skip(2) {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                if parts.len() > 2 {
                    let pkg = Package {
                        name: parts[0].to_string(),
                        version: parts[1].to_string(),
                    };
                    packages.push(pkg);
                }
            }
            packages
        };

        let os_version = parse_os_release()
            .get("VERSION_ID")
            .as_result("VERSION_ID not found")?
            .to_owned();

        let os_version_num = os_version.parse::<i32>()?;

        let update_command = vec![
            "pkexec".to_string(),
            "bash".to_string(),
            "-c".to_string(),
            "dnf --refresh upgrade".to_string(),
        ];


        let os_name = "Fedora".to_string();

        let latest_version = surf::get(FEDORA_COLLECTION_URL)
            .recv_json::<FedoraUpdateCollection>()
            .await?
            .collections
            .into_iter()
            .filter(|update| update.status == Some("Active".to_string()))
            .filter(|update| update.koji_name.is_some())
            .filter(|update| update.version.is_some())
            .filter(|update| update.koji_name.as_ref().unwrap().starts_with("f"))
            .filter(|update| update.version.as_ref().unwrap().parse::<i32>().is_ok())
            .max_by(|a, b| {
                let a = a
                    .version
                    .as_ref()
                    .unwrap_or(&"0".to_string())
                    .parse::<i32>()
                    .unwrap_or(0);
                let b = b
                    .version
                    .as_ref()
                    .unwrap_or(&"0".to_string())
                    .parse::<i32>()
                    .unwrap_or(0);
                a.cmp(&b)
            })
            .as_result("Failed to get latest version")?;

        let dist_update_command = vec![
            "pkexec".to_string(),
            "bash".to_string(),
            "-c".to_string(),
            format!(
                "dnf --refresh upgrade && dnf system-upgrade download --releasever={}",
                latest_version.version.as_ref().unwrap()
            ),
        ];
        Ok(DnfPackageManager {
            packages,
            os_name,
            os_version_num,
            os_version,
            dist_update_command,
            update_command,
            latest_version,
        })
    }
}

impl PackageManager for DnfPackageManager {
    fn dist_update_desertion(&self) -> String {
        format!(
            "Your system is ready to update to Fedora {}.",
            self.get_dist_upgrade_version()
        )
    }

    fn dist_update_msg(&self) -> String {
        "Your system is ready to update to the latest version of Fedora.".to_string()
    }

    fn has_dist_upgrade(&self) -> bool {
        let v = self.latest_version.version.clone().unwrap();
        let v = v.parse::<i32>().unwrap();
        return v > self.os_version_num;
    }

    fn get_dist_upgrade_version(&self) -> &String {
        return self.latest_version.version.as_ref().unwrap();
    }

    fn list_updates(&self) -> &Vec<Package> {
        &self.packages
    }

    fn os_release(&self) -> &String {
        &self.os_version
    }

    fn reboot(&self) -> () {
        if self.has_dist_upgrade() {
            SyncCommand::new("pkexec")
                .arg("dnf")
                .arg("system-upgrade")
                .arg("reboot")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            return;
        } else {
            SyncCommand::new("pkexec")
                .arg("systemctl")
                .arg("reboot")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }

    fn os_name(&self) -> &String {
        &self.os_name
    }

    fn has_update(&self) -> bool {
        self.packages.len() > 0
    }

    fn get_dist_update_command(&self) -> Vec<&str> {
        self.dist_update_command
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    fn get_update_command(&self) -> Vec<&str> {
        self.update_command.iter().map(|s| s.as_str()).collect()
    }
}

#[derive(Debug, serde::Deserialize)]
struct FedoraUpdateCollection {
    pub collections: Vec<FedoraUpdate>,
}

#[derive(Debug, serde::Deserialize)]
struct FedoraUpdate {
    version: Option<String>,
    koji_name: Option<String>,
    status: Option<String>,
}
