try:
    import __vcmp
except ImportError:
    print("You are not running python in vcmp server.")
    exit(1)

# first must import vcmp module
from .instance import *
from .events import *
from .callbacks import *
from .common import (
    scheduler,
    run
)