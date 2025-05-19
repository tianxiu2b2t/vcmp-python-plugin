from typing import Optional
from vcmp.__export import funcs
from vcmp.vec import Vector
from vcmp.instance import RGB, CheckPoint, Player

def create_checkpoint(
    world: int,
    sphere: bool,
    pos: Vector,
    color: RGB,
    radius: float,
    player: Optional[Player | int]
):
    """

    Creates a checkpoint.

    :param model: The model of the checkpoint.
    :param world: The world of the checkpoint.
    :param quantity: The quantity of the checkpoint.
    :param pos: The position of the checkpoint.
    :param alpha: The alpha of the checkpoint.
    :param automatic: Whether the checkpoint is automatic or not.
    :return: The created checkpoint.
    """
    
    id = player.id if isinstance(player, Player) else (
        player if isinstance(player, int) else -1
    )
    return CheckPoint(
        funcs.create_check_point(
            id,
            world,
            sphere,
            pos.x,
            pos.y,
            pos.z,
            color.red,
            color.green,
            color.blue,
            color.alpha,
            radius,
        )
    )