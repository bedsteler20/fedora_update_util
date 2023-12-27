
project('fedora_update_utility', 'rust',
    version         : '0.1.0',
    license         : 'GPL3')

i18n = import('i18n')
gnome = import('gnome')

prefix = get_option('prefix')
systemd_userunitdir = join_paths(prefix, 'lib', 'systemd', 'user')

subdir('src')
subdir('data')
