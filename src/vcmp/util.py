import math
import random
from vcmp.types import RGB, AreaPoints, MAX_PLAYERS
from vcmp.instance import Player, get_player_from_id
from vcmp.functions.player import is_player_connected

POLY = tuple[float, float]
WEAPON_NAMES = {
    0: 'Fists', 
    1: 'Brass Knuckles', 
    2: 'Screwdriver', 
    3: 'Golfclub', 
    4: 'Nitestick', 
    5: 'Knife', 
    6: 'Baseball bat', 
    7: 'Hammer', 
    8: 'Meat Cleaver', 
    9: 'Machete', 
    10: 'Katana', 
    11: 'Chainsaw', 
    12: 'Grenades', 
    13: 'Remote Grenades', 
    14: 'Teargas',
    15: 'Molotov Cocktails',
    16: 'Missile',
    17: 'Colt .45',
    18: 'Python',
    19: 'Shotgun',
    20: 'Spaz Shotgun',
    21: 'Stubby Shotgun',
    22: 'Tec 9',
    23: 'Uzi',
    24: 'Ingram',
    25: 'MP5',
    26: 'M4',
    27: 'Ruger',
    28: 'Sniper Rifle',
    29: 'Laser Sniper',
    30: 'Rocket Launcher',
    31: 'Flame Thrower',
    32: 'M60',
    33: 'Minigun',
    34: 'Bomb',
    35: 'Helicannon',
    36: 'Camera',
    39: 'Vehicle',
    40: 'Heli-blades',
    41: 'Explosion',
    42: 'Drive-By',
    43: 'Drown',
    44: 'Fall',
    51: 'Explosion',
    60: 'Heli-blades',
    70: 'Suicide'
}

WEAPON_MODELS = {
    0: 293,
    1: 259,
    2: 260,
    3: 261,
    4: 262,
    5: 263,
    6: 264,
    7: 265,
    8: 266,
    9: 267,
    10: 268,
    11: 269,
    12: 270,
    13: 291,
    14: 271,
    15: 272,
    16: 273,
    17: 274,
    18: 275,
    19: 277,
    20: 278,
    21: 279,
    22: 281,
    23: 282,
    24: 283,
    25: 284,
    26: 280,
    27: 276,
    28: 285,
    29: 286,
    30: 287,
    31: 288,
    32: 289,
    33: 290,
    36: 292
}

VEHICLE_NAMES = {
    130: 'Landstalker',
    131: 'Idaho',
    132: 'Stinger',
    133: 'Linerunner',
    134: 'Perennial',
    135: 'Sentinel',
    136: 'Rio',
    137: 'Firetruck',
    138: 'Trashmaster',
    139: 'Stretch',
    140: 'Manana',
    141: 'Infernus',
    142: 'Voodoo',
    143: 'Pony',
    144: 'Mule',
    145: 'Cheetah #1',
    146: 'Ambulance',
    147: 'FBI Washington',
    148: 'Moonbeam',
    149: 'Esperanto',
    150: 'Taxi',
    151: 'Washington',
    152: 'Bobcat',
    153: 'Mr Whoopee',
    154: 'BF Injection',
    155: 'Hunter',
    156: 'Police',
    157: 'Enforcer',
    158: 'Securicar',
    159: 'Banshee',
    160: 'Predator',
    161: 'Bus',
    162: 'Rhino',
    163: 'Barracks OL',
    164: 'Cuban Hermes',
    166: 'Angel',
    167: 'Coach',
    168: 'Cabbie',
    169: 'Stallion',
    170: 'Rumpo',
    171: 'RC Bandit',
    172: "Romero's Hearse",
    173: 'Packer',
    174: 'Sentinel XS',
    175: 'Admiral',
    176: 'Squalo',
    177: 'Sea Sparrow',
    178: 'Pizza boy',
    179: 'Gang Burrito',
    182: 'Speeder',
    183: 'Reefer',
    184: 'Tropic',
    185: 'Flatbed',
    186: 'Yankee',
    187: 'Caddy',
    188: 'Zebra Cab',
    189: 'Top Fun',
    190: 'Skimmer',
    191: 'PCJ 600',
    192: 'Faggio',
    193: 'Freeway',
    194: 'RC Baron',
    195: 'RC Raider',
    196: 'Glendale',
    197: 'Oceanic',
    198: 'Sanchez',
    199: 'Sparrow',
    200: 'Patriot',
    201: 'Love Fist',
    202: 'Coast Guard',
    203: 'Dinghy',
    204: 'Hermes',
    205: 'Sabre',
    206: 'Sabre Turbo',
    207: 'Phoenix',
    208: 'Walton',
    209: 'Regina',
    210: 'Comet',
    211: 'Deluxo',
    212: 'Burrito',
    213: 'Spand Express',
    214: 'Marquis',
    215: 'Baggage Handler',
    216: 'Kaufman Cab',
    217: 'Maverick',
    218: 'VCN Maverick',
    219: 'Rancher',
    220: 'FBI Rancher',
    221: 'Virgo',
    222: 'Greenwood',
    223: 'Cuban Jetmax',
    224: 'Hotring Racer #1',
    225: 'Sandking',
    226: 'Blista Compact',
    227: 'Police Maverick',
    228: 'Boxville',
    229: 'Benson',
    230: 'Mesa Grande',
    231: 'RC Goblin',
    232: 'Hotring Racer #2',
    233: 'Hotring Racer #3',
    234: 'Bloodring Banger #1',
    235: 'Bloodring Banger #2',
    236: 'Cheetah #2'
}
VEHICLE_CLASSIC_CAR = [
    130, 131, 132, 133, 134, 135, 137, 138, 139, 
    140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 
    150, 151, 152, 153, 154, 156, 157, 158, 159, 
    161, 162, 163, 164, 167, 168, 169, 
    170, 172, 173, 174, 175, 179, 
    185, 186, 187, 188, 189, 
    196, 197, 
    200, 201, 204, 205, 206, 207, 208, 209, 
    210, 211, 212, 213, 215, 216, 219, 
    220, 221, 222, 224, 225, 226, 228, 229, 
    230, 232, 233, 234, 235, 236
]
VEHICLE_CLASSIC_BOAT = [136, 160, 176, 182, 183, 184, 190, 202, 203, 214, 223]
VEHICLE_CLASSIC_AIR = [155, 177, 199, 217, 218, 227]
VEHICLE_CLASSIC_BIKE = [166, 178, 191, 192, 193, 198]
VEHICLE_CLASSIC_RC = [171, 194, 195, 231]

SKINS = {
	0:   "Tommy Vercetti",
	1:   "Cop",
	2:   "SWAT",
	3:   "FBI",
	4:   "Army",
	5:   "Paramedic",
	6:   "Firefighter",
	7:   "Golf Guy #1",
	9:   "Bum Lady #1",
	10:  "Bum Lady #2",
	11:  "Punk #1",
	12:  "Lawyer",
	13:  "Spanish Lady #1",
	14:  "Spanish Lady #2",
	15:  "Cool Guy #1",
	16:  "Arabic Guy",
	17:  "Beach Lady #1",
	18:  "Beach Lady #2",
	19:  "Beach Guy #1",
	20:  "Beach Guy #2",
	21:  "Office Lady #1",
	22:  "Waitress #1",
	23:  "Food Lady",
	24:  "Prostitute #1",
	25:  "Bum Lady #3",
	26:  "Bum Guy #1",
	27:  "Garbageman #1",
	28:  "Taxi Driver #1",
	29:  "Haitian #1",
	30:  "Criminal #1",
	31:  "Hood Lady",
	32:  "Granny #1",
	33:  "Businessman #1",
	34:  "Church Guy",
	35:  "Club Lady",
	36:  "Church Lady",
	37:  "Pimp",
	38:  "Beach Lady #3",
	39:  "Beach Guy #3",
	40:  "Beach Lady #4",
	41:  "Beach Guy #4",
	42:  "Businessman #2",
	43:  "Prostitute #2",
	44:  "Bum Lady #4",
	45:  "Bum Guy #2",
	46:  "Haitian #2",
	47:  "Construction Worker #1",
	48:  "Punk #2",
	49:  "Prostitute #3",
	50:  "Granny #2",
	51:  "Punk #3",
	52:  "Businessman #3",
	53:  "Spanish Lady #3",
	54:  "Spanish Lady #4",
	55:  "Cool Guy #2",
	56:  "Businessman #4",
	57:  "Beach Lady #5",
	58:  "Beach Guy #5",
	59:  "Beach Lady #6",
	60:  "Beach Guy #6",
	61:  "Construction Worker #2",
	62:  "Golf Guy #2",
	63:  "Golf Lady",
	64:  "Golf Guy #3",
	65:  "Beach Lady #7",
	66:  "Beach Guy #7",
	67:  "Office Lady #2",
	68:  "Businessman #5",
	69:  "Businessman #6",
	70:  "Prostitute #2",
	71:  "Bum Lady #4",
	72:  "Bum Guy #3",
	73:  "Spanish Guy",
	74:  "Taxi Driver #2",
	75:  "Gym Lady",
	76:  "Gym Guy",
	77:  "Skate Lady",
	78:  "Skate Guy",
	79:  "Shopper #1",
	80:  "Shopper #2",
	81:  "Tourist #1",
	82:  "Tourist #2",
	83:  "Cuban #1",
	84:  "Cuban #2",
	85:  "Haitian #3",
	86:  "Haitian #4",
	87:  "Shark #1",
	88:  "Shark #2",
	89:  "Diaz Guy #1",
	90:  "Diaz Guy #2",
	91:  "DBP Security #1",
	92:  "DBP Security #2",
	93:  "Biker #1",
	94:  "Biker #2",
	95:  "Vercetti Guy #1",
	96:  "Vercetti Guy #2",
	97:  "Undercover Cop #1",
	98:  "Undercover Cop #2",
	99:  "Undercover Cop #3",
	100: "Undercover Cop #4",
	101: "Undercover Cop #5",
	102: "Undercover Cop #6",
	103: "Rich Guy",
	104: "Cool Guy #3",
	105: "Prostitute #3",
	106: "Prostitute #4",
	107: "Love Fist #1",
	108: "Ken Rosenburg",
	109: "Candy Suxx",
	110: "Hilary",
	111: "Love Fist #2",
	112: "Phil",
	113: "Rockstar Guy",
	114: "Sonny",
	115: "Lance",
	116: "Mercedes",
	117: "Love Fist #3",
	118: "Alex Shrub",
	119: "Lance (Cop)",
	120: "Lance",
	121: "Cortez",
	122: "Love Fist #4",
	123: "Columbian Guy #1",
	124: "Hilary (Robber)",
	125: "Mercedes",
	126: "Cam",
	127: "Cam (Robber)",
	128: "Phil (One Arm)",
	129: "Phil (Robber)",
	130: "Cool Guy #4",
	131: "Pizza Man",
	132: "Taxi Driver #1",
	133: "Taxi Driver #2",
	134: "Sailor #1",
	135: "Sailor #2",
	136: "Sailor #3",
	137: "Chef",
	138: "Criminal #2",
	139: "French Guy",
	140: "Garbageman #2",
	141: "Haitian #5",
	142: "Waitress #2",
	143: "Sonny Guy #1",
	144: "Sonny Guy #2",
	145: "Sonny Guy #3",
	146: "Columbian Guy #2",
	147: "Haitian #6",
	148: "Beach Guy #8",
	149: "Garbageman #3",
	150: "Garbageman #4",
	151: "Garbageman #5",
	152: "Tranny",
	153: "Thug #5",
	154: "SpandEx Guy #1",
	155: "SpandEx Guy #2",
	156: "Stripper #1",
	157: "Stripper #2",
	158: "Stripper #3",
	159: "Store Clerk",
	161: "Tommy with Suit",
	162: "Worker Tommy",
	163: "Golfer Tommy",
	164: "Cuban Tommy",
	165: "VCPD Tommy",
	166: "Bank Robber Tommy",
	167: "Street Tommy",
	168: "Mafia Tommy",
	169: "Jogger Tommy #1",
	170: "Jogger Tommy #2",
	171: "Guy With Suit #1",
	172: "Guy With Suit #3",
	173: "Prostitute #5",
	174: "Rico",
	175: "Prostitute #3",
	176: "Club Lady",
	177: "Prostitute #2",
	178: "Skull T-Shirt Guy",
	179: "Easter Egg Tommy",
	180: "Diaz Gangster #1",
	181: "Diaz Gangster #2",
	182: "Hood Lady",
	183: "Punk #1",
	184: "Tray Lady",
	185: "Kent Paul",
	186: "Taxi Driver #1",
	187: "Deformed Ken Rosenberg",
	188: "Deformed Woman",
	189: "Deformed Man",
	190: "Deformed Cortez",
	191: "Deformed Lance Vance",
	192: "Thief #1",
	193: "Thief #2",
	194: "Thief #3",
}
  
def get_vehicle_random_color(
    color: int
):
    if color < 0:
        return random.randint(0, 94)
    return color

def in_poly(
    x: float,
    y: float,
    *polies: POLY
):
    """
    Determines if a point is inside a polygon using the Jordan Curve Theorem.

    Args:
        x (float): The x-coordinate of the point.
        y (float): The y-coordinate of the point.
        ui_points (unsigned int): The number of points in the polygon.
        f_points (list of AreaPoints): The points creating the polygon.

    Returns:
        bool: True if the point is inside the polygon, False otherwise.
    """

    # Initialize variables to track crossings
    crossings = 0

    # Iterate through each line segment of the polygon
    ui_points = len(polies)
    f_points = [
        AreaPoints(
            polies[i][0],
            polies[i][1]
        ) for i in range(ui_points)
    ]
    for i in range(ui_points):
        # Ensure the line segment is checked from left to right
        if f_points[i].x < f_points[(i + 1) % ui_points].x:
            x1 = f_points[i].x
            x2 = f_points[(i + 1) % ui_points].x
        else:
            x1 = f_points[(i + 1) % ui_points].x
            x2 = f_points[i].x

        # Check if the ray can potentially cross the line segment
        if x > x1 and x <= x2 and (y < f_points[i].y or y <= f_points[(i + 1) % ui_points].y):
            # Calculate the equation of the line segment
            dx = f_points[(i + 1) % ui_points].x - f_points[i].x
            dy = f_points[(i + 1) % ui_points].y - f_points[i].y

            # Avoid division by zero for vertical lines
            if abs(dx) < 0.000001:
                k = float('inf')
            else:
                k = dy / dx

            m = f_points[i].y - k * f_points[i].x

            # Determine if the ray crosses the line segment
            y2 = k * x + m
            if y <= y2:
                crossings += 1

    # Return True if the number of crossings is odd (point is inside)
    return crossings % 2 == 1

def distance_from_point(
    x: float,
    y: float,
    x1: float,
    y1: float,
):
    return math.sqrt((x - x1) ** 2 + (y - y1) ** 2)

def get_district_name(
    x: float, 
    y: float
):
    if -213.73 > x > -1613.03 and 413.218 > y > 1677.32:
        return "Downtown Vice City"
    elif 163.656 > x > -213.73 and -351.153 > y > -930.526:
        return "Vice Point"
    elif -103.97 > x > -213.73 and -930.526 > y > -1805.37:
        return "Washington Beach"
    elif -253.206 > x > -1888.21 and -1805.37 > y > -1779.61:
        return "Ocean Beach"
    elif -748.206 > x > -104.505 and -818.266 > y > -241.467:
        return "Starfish Island"
    elif -213.73 > x > -104.505 and 797.605 > y > -241.467:
        return "Prawn Island"
    elif -213.73 > x > -104.505 and -241.429 > y > 797.605:
        return "Leaf Links"
    elif -1396.76 > x > -1208.21 and -42.9113 > y > -1779.61:
        return "Junkyard"
    elif -1208.21 > x > -748.206 and -898.738 > y > -241.467:
        return "Little Havana"
    elif -1208.21 > x > -578.289 and -241.467 > y > 412.66:
        return "Little Haiti"
    return "Vice City"

def get_players() -> list[Player]:
    players = []
    for i in range(MAX_PLAYERS + 1):
        if not is_player_connected(i):
            continue
        players.append(get_player_from_id(i))
    return players

def announce_all(
    type: int,
    message: str
):
    for player in get_players():
        player.send_announce(type, message)

def message_all(message: str):
    for player in get_players():
        player.send_message(message)

def raw_message_all(
    color: RGB,
    message: str
):
    for player in get_players():
        player.send_raw_message(color, message)