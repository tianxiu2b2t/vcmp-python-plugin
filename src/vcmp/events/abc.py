from typing import Any


class Event:
    ...

    @property
    def kwargs(self) -> dict[str, Any]:
        ...

    @kwargs.setter
    def kwargs(self, value: dict[str, Any]) -> None:
        ...