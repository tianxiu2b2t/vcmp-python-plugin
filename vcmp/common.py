import threading
from typing import Callable
from apscheduler.schedulers.background import BackgroundScheduler


scheduler = BackgroundScheduler()
scheduler.start()

thread = None

def run(
    func: Callable[..., None],
    *args,
    **kwargs,
) -> None:
    global thread
    def wrapper():
        global thread
        try:
            func(*args, **kwargs)
        except:
            raise
        finally:
            thread = None

    if thread is not None:
        raise Exception("Thread already running")

    thread = threading.Thread(
        target=wrapper,
        daemon=True,
        name="RunnerThread"
    )
    thread.start()
    