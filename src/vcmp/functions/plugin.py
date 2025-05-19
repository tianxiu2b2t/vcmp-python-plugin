from dataclasses import dataclass
from vcmp.__export import funcs

@dataclass
class PluginInfo:
    """
    Represents information about a plugin.
    """
    name: str
    plugin_version: str
    plugin_id: int
    api_major_version: int
    api_minor_version: int
    struct_size: int


def get_plugin_count():
    """
    Returns the number of plugins loaded.
    """
    return funcs.get_number_of_plugins()

def get_plugins() -> list[PluginInfo]:
    """
    Returns a list of all loaded plugins.
    """
    plugins = []
    for i in range(get_plugin_count()):
        try:
            plugins.append(PluginInfo(**funcs.get_plugin_info(i)))
        except:
            pass
    return plugins

def send_plugin_command(
    command_id: int,
    command: str,
):
    """
    Sends a command to a plugin.
    """
    return funcs.send_plugin_command(command_id, command)
