from vcmp.types import WeaponField as Weapon
from vcmp.__export import funcs

WeaponField = int | Weapon

def set_weapon_data_value(
    weapon_id: int,
    field: WeaponField,
    value: float
):
    """
    Sets the value of a weapon data field.

    :param weapon_id: The weapon ID.
    :param field: The weapon data field.
    :param value: The value to set.
    """
    funcs.set_weapon_data_value(weapon_id, field, value)

def get_weapon_data_value(
    weapon_id: int,
    field: WeaponField
):
    """
    Gets the value of a weapon data field.

    :param weapon_id: The weapon ID.
    :param field: The weapon data field.
    :return: The value of the weapon data field.
    """
    return funcs.get_weapon_data_value(weapon_id, field)

def reset_weapon_data_value(
    weapon_id: int,
    field: WeaponField
):
    """
    Resets the value of a weapon data field to its default value.

    :param weapon_id: The weapon ID.
    :param field: The weapon data field.
    """
    funcs.reset_weapon_data_value(weapon_id, field)

def is_weapon_data_value_modified(
    weapon_id: int,
    field: WeaponField
):
    """
    Checks if a weapon data field has been modified.

    :param weapon_id: The weapon ID.
    :param field: The weapon data field.
    :return: True if the weapon data field has been modified, False otherwise.
    """
    return funcs.is_weapon_data_value_modified(weapon_id, field)


def reset_weapon_data(
    weapon_id: int
):
    """
    Resets all weapon data fields to their default values.

    :param weapon_id: The weapon ID.
    """
    funcs.reset_weapon_data(weapon_id)

def reset_all_weapon_data():
    """
    Resets all weapon data fields for all weapons to their default values.
    """
    funcs.reset_all_weapon_data()

    