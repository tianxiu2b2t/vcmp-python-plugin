from vcmp.__export import funcs

def ban_ip(
    address: str
):
    """Ban an IP address."""
    funcs.ban_ip(address)

def pardon_ip(
    address: str
):
    """Pardon an IP address."""
    funcs.unban_ip(address)

def ip_banned(
    address: str
) -> bool:
    """Check if an IP address is banned."""
    return funcs.is_ip_banned(address)