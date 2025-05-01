try:
    import __vcmp
except ImportError:
    print("You are not running python in vcmp server.")
    exit(1)

from .__runner import run
from .events import *
from .__callbacks import *