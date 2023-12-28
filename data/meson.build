conf_data = configuration_data()
conf_data.set('exec', get_option('prefix') / get_option('bindir') / 'fedora_update_utility')

configure_file(input: 'dev.bedsteler20.FedoraUpdateUtility.desktop',
                output: 'dev.bedsteler20.FedoraUpdateUtility.desktop',
                configuration: conf_data,
                install_dir: get_option('datadir') / 'applications'
)

install_data(
    'dev.bedsteler20.FedoraUpdateUtility.metainfo.xml',
    install_dir: get_option('datadir') / 'metainfo'
)

# configure_file(input : 'fedora_update_utility.service',
#                output : 'fedora_update_utility.service',
#                configuration : conf_data,
#                install_dir : systemd_userunitdir
# )

# install_data(
#     'fedora_update_utility.timer',
#     install_dir: systemd_userunitdir
# )
