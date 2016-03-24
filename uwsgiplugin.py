import os
import os.path

base_path = os.path.dirname(os.path.realpath("__file__"))
NAME = 'rust'
GCC_LIST = ['rust', '%s/target/release/libuwsgi_rust.a' % base_path]

CFLAGS = []

if os.uname()[0] == 'Darwin':
    CFLAGS.append('-mmacosx-version-min=10.7')

if os.system("cargo build --release") != 0:
    os._exit(1)

# To also build the example app:
#os.system("cargo build --release --manifest-path examples/Cargo.toml")
