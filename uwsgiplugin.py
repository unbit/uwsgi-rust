import os
import os.path
import inspect

base_path = os.path.dirname(inspect.getframeinfo(inspect.currentframe())[0])

NAME = 'rust'
GCC_LIST = ['rust', '%s/plugin.a' % base_path]

CFLAGS = []

if os.uname()[0] == 'Darwin':
    CFLAGS.append('-mmacosx-version-min=10.7')


if os.system("rustc -o %s/plugin.a --crate-type staticlib %s/plugin.rs" % (base_path, base_path)) != 0:
    os._exit(1)
