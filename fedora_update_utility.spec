%global srcname copr-tito-quickdoc

Name: fedora-update-utility
Version: 1.0.0
Release: 1%{?dist}
License: GPLv3
Summary: A gui for updating fedora
Url: https://pagure.io/%{srcname}
# Sources can be obtained by
# git clone https://pagure.io/copr-tito-quickdoc
# cd copr-tito-quickdoc
# tito build --tgz
Source0: %{name}-%{version}.tar.gz

BuildArch: x86_64

BuildRequires: systemd-rpm-macros
BuildRequires: blueprint-compiler
BuildRequires: cargo
BuildRequires: rustc
BuildRequires: meson
BuildRequires: ninja-build
BuildRequires: flatpak-devel
BuildRequires: libadwaita-devel
BuildRequires: gtk4-devel
BuildRequires: vte291-gtk4-devel

Requires: vte291-gtk4
Requires: libadwaita
Requires: dnf5

%description
A gui for updating fedora

#-- PREP, BUILD & INSTALL -----------------------------------------------------#
%prep
%autosetup

%build
%meson
%meson_build

%install
%meson_install

%post
%systemd_user_post fedora_update_utility.service
%systemd_user_post fedora_update_utility.timer

%preun
%systemd_user_preun fedora_update_utility.service
%systemd_user_preun fedora_update_utility.timer 

%postun
%systemd_user_postun_with_restart fedora_update_utility.service
%systemd_user_postun_with_restart fedora_update_utility.timer


#-- FILES ---------------------------------------------------------------------#
%files
%doc README.md
%license LICENSE
%{_bindir}/fedora_update_utility
%{_datadir}/applications/dev.bedsteler20.FedoraUpdateUtility.desktop
%{_datadir}/metainfo/dev.bedsteler20.FedoraUpdateUtility.metainfo.xml
%{%_userunitdir}/

#-- CHANGELOG -----------------------------------------------------------------#
%changelog
* Tue Dec 26 2023 Cameron Dehning <bedsteler2.0@gmail.com> 1.0.0-1
- new package built with tito

