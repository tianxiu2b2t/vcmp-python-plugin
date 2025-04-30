import threading
from typing import Any, Awaitable, Callable, TypeVarTuple, Unpack

import anyio

from .__callbacks import callbacks


PosArgsT = TypeVarTuple("PosArgsT")

thread = None
event = threading.Event() # we need wait for thread start and anyio run

async def _wrapper(
    func: Callable[[Unpack[PosArgsT]], Awaitable[Any]],
    *args: Unpack[PosArgsT],
):
    global event
    async with anyio.create_task_group() as task_group:
        task_group.start_soon(func, *args)

        task_group.start_soon(
            callbacks.__call__,
        )

        event.set()
        task_group.start_soon(anyio.sleep_forever)


def run(
    func: Callable[[Unpack[PosArgsT]], Awaitable[Any]],
    *args: Unpack[PosArgsT],
    backend: str = "asyncio",
    backend_options: dict[str, Any] | None = None,
) -> None:
    global thread


    thread = threading.Thread(
        target=lambda: anyio.run(_wrapper, func, *args, backend=backend, backend_options=backend_options),
        daemon=True,
        name="RunnerThread"
    )

    thread.start()
    event.wait()