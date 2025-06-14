use std::collections::HashMap;
use std::sync::LazyLock;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict};
use pyo3::{
    Bound, PyResult, Python, pyfunction, types::PyModule, wrap_pyfunction,
};


pub static SKINS: LazyLock<HashMap<i32, &str>> = LazyLock::new(|| {
    let mut skins = HashMap::new();
    skins.insert(0,   "Tommy Vercetti");
    skins.insert(1,   "Cop");
    skins.insert(2,   "SWAT");
    skins.insert(3,   "FBI");
    skins.insert(4,   "Army");
    skins.insert(5,   "Paramedic");
    skins.insert(6,   "Firefighter");
    skins.insert(7,   "Golf Guy #1");
    skins.insert(9,   "Bum Lady #1");
    skins.insert(10,  "Bum Lady #2");
    skins.insert(11,  "Punk #1");
    skins.insert(12,  "Lawyer");
    skins.insert(13,  "Spanish Lady #1");
    skins.insert(14,  "Spanish Lady #2");
    skins.insert(15,  "Cool Guy #1");
    skins.insert(16,  "Arabic Guy");
    skins.insert(17,  "Beach Lady #1");
    skins.insert(18,  "Beach Lady #2");
    skins.insert(19,  "Beach Guy #1");
    skins.insert(20,  "Beach Guy #2");
    skins.insert(21,  "Office Lady #1");
    skins.insert(22,  "Waitress #1");
    skins.insert(23,  "Food Lady");
    skins.insert(24,  "Prostitute #1");
    skins.insert(25,  "Bum Lady #3");
    skins.insert(26,  "Bum Guy #1");
    skins.insert(27,  "Garbageman #1");
    skins.insert(28,  "Taxi Driver #1");
    skins.insert(29,  "Haitian #1");
    skins.insert(30,  "Criminal #1");
    skins.insert(31,  "Hood Lady");
    skins.insert(32,  "Granny #1");
    skins.insert(33,  "Businessman #1");
    skins.insert(34,  "Church Guy");
    skins.insert(35,  "Club Lady");
    skins.insert(36,  "Church Lady");
    skins.insert(37,  "Pimp");
    skins.insert(38,  "Beach Lady #3");
    skins.insert(39,  "Beach Guy #3");
    skins.insert(40,  "Beach Lady #4");
    skins.insert(41,  "Beach Guy #4");
    skins.insert(42,  "Businessman #2");
    skins.insert(43,  "Prostitute #2");
    skins.insert(44,  "Bum Lady #4");
    skins.insert(45,  "Bum Guy #2");
    skins.insert(46,  "Haitian #2");
    skins.insert(47,  "Construction Worker #1");
    skins.insert(48,  "Punk #2");
    skins.insert(49,  "Prostitute #3");
    skins.insert(50,  "Granny #2");
    skins.insert(51,  "Punk #3");
    skins.insert(52,  "Businessman #3");
    skins.insert(53,  "Spanish Lady #3");
    skins.insert(54,  "Spanish Lady #4");
    skins.insert(55,  "Cool Guy #2");
    skins.insert(56,  "Businessman #4");
    skins.insert(57,  "Beach Lady #5");
    skins.insert(58,  "Beach Guy #5");
    skins.insert(59,  "Beach Lady #6");
    skins.insert(60,  "Beach Guy #6");
    skins.insert(61,  "Construction Worker #2");
    skins.insert(62,  "Golf Guy #2");
    skins.insert(63,  "Golf Lady");
    skins.insert(64,  "Golf Guy #3");
    skins.insert(65,  "Beach Lady #7");
    skins.insert(66,  "Beach Guy #7");
    skins.insert(67,  "Office Lady #2");
    skins.insert(68,  "Businessman #5");
    skins.insert(69,  "Businessman #6");
    skins.insert(70,  "Prostitute #2");
    skins.insert(71,  "Bum Lady #4");
    skins.insert(72,  "Bum Guy #3");
    skins.insert(73,  "Spanish Guy");
    skins.insert(74,  "Taxi Driver #2");
    skins.insert(75,  "Gym Lady");
    skins.insert(76,  "Gym Guy");
    skins.insert(77,  "Skate Lady");
    skins.insert(78,  "Skate Guy");
    skins.insert(79,  "Shopper #1");
    skins.insert(80,  "Shopper #2");
    skins.insert(81,  "Tourist #1");
    skins.insert(82,  "Tourist #2");
    skins.insert(83,  "Cuban #1");
    skins.insert(84,  "Cuban #2");
    skins.insert(85,  "Haitian #3");
    skins.insert(86,  "Haitian #4");
    skins.insert(87,  "Shark #1");
    skins.insert(88,  "Shark #2");
    skins.insert(89,  "Diaz Guy #1");
    skins.insert(90,  "Diaz Guy #2");
    skins.insert(91,  "DBP Security #1");
    skins.insert(92,  "DBP Security #2");
    skins.insert(93,  "Biker #1");
    skins.insert(94,  "Biker #2");
    skins.insert(95,  "Vercetti Guy #1");
    skins.insert(96,  "Vercetti Guy #2");
    skins.insert(97,  "Undercover Cop #1");
    skins.insert(98,  "Undercover Cop #2");
    skins.insert(99,  "Undercover Cop #3");
    skins.insert(100, "Undercover Cop #4");
    skins.insert(101, "Undercover Cop #5");
    skins.insert(102, "Undercover Cop #6");
    skins.insert(103, "Rich Guy");
    skins.insert(104, "Cool Guy #3");
    skins.insert(105, "Prostitute #3");
    skins.insert(106, "Prostitute #4");
    skins.insert(107, "Love Fist #1");
    skins.insert(108, "Ken Rosenburg");
    skins.insert(109, "Candy Suxx");
    skins.insert(110, "Hilary");
    skins.insert(111, "Love Fist #2");
    skins.insert(112, "Phil");
    skins.insert(113, "Rockstar Guy");
    skins.insert(114, "Sonny");
    skins.insert(115, "Lance");
    skins.insert(116, "Mercedes");
    skins.insert(117, "Love Fist #3");
    skins.insert(118, "Alex Shrub");
    skins.insert(119, "Lance (Cop)");
    skins.insert(120, "Lance");
    skins.insert(121, "Cortez");
    skins.insert(122, "Love Fist #4");
    skins.insert(123, "Columbian Guy #1");
    skins.insert(124, "Hilary (Robber)");
    skins.insert(125, "Mercedes");
    skins.insert(126, "Cam");
    skins.insert(127, "Cam (Robber)");
    skins.insert(128, "Phil (One Arm)");
    skins.insert(129, "Phil (Robber)");
    skins.insert(130, "Cool Guy #4");
    skins.insert(131, "Pizza Man");
    skins.insert(132, "Taxi Driver #1");
    skins.insert(133, "Taxi Driver #2");
    skins.insert(134, "Sailor #1");
    skins.insert(135, "Sailor #2");
    skins.insert(136, "Sailor #3");
    skins.insert(137, "Chef");
    skins.insert(138, "Criminal #2");
    skins.insert(139, "French Guy");
    skins.insert(140, "Garbageman #2");
    skins.insert(141, "Haitian #5");
    skins.insert(142, "Waitress #2");
    skins.insert(143, "Sonny Guy #1");
    skins.insert(144, "Sonny Guy #2");
    skins.insert(145, "Sonny Guy #3");
    skins.insert(146, "Columbian Guy #2");
    skins.insert(147, "Haitian #6");
    skins.insert(148, "Beach Guy #8");
    skins.insert(149, "Garbageman #3");
    skins.insert(150, "Garbageman #4");
    skins.insert(151, "Garbageman #5");
    skins.insert(152, "Tranny");
    skins.insert(153, "Thug #5");
    skins.insert(154, "SpandEx Guy #1");
    skins.insert(155, "SpandEx Guy #2");
    skins.insert(156, "Stripper #1");
    skins.insert(157, "Stripper #2");
    skins.insert(158, "Stripper #3");
    skins.insert(159, "Store Clerk");
    skins.insert(161, "Tommy with Suit");
    skins.insert(162, "Worker Tommy");
    skins.insert(163, "Golfer Tommy");
    skins.insert(164, "Cuban Tommy");
    skins.insert(165, "VCPD Tommy");
    skins.insert(166, "Bank Robber Tommy");
    skins.insert(167, "Street Tommy");
    skins.insert(168, "Mafia Tommy");
    skins.insert(169, "Jogger Tommy #1");
    skins.insert(170, "Jogger Tommy #2");
    skins.insert(171, "Guy With Suit #1");
    skins.insert(172, "Guy With Suit #3");
    skins.insert(173, "Prostitute #5");
    skins.insert(174, "Rico");
    skins.insert(175, "Prostitute #3");
    skins.insert(176, "Club Lady");
    skins.insert(177, "Prostitute #2");
    skins.insert(178, "Skull T-Shirt Guy");
    skins.insert(179, "Easter Egg Tommy");
    skins.insert(180, "Diaz Gangster #1");
    skins.insert(181, "Diaz Gangster #2");
    skins.insert(182, "Hood Lady");
    skins.insert(183, "Punk #1");
    skins.insert(184, "Tray Lady");
    skins.insert(185, "Kent Paul");
    skins.insert(186, "Taxi Driver #1");
    skins.insert(187, "Deformed Ken Rosenberg");
    skins.insert(188, "Deformed Woman");
    skins.insert(189, "Deformed Man");
    skins.insert(190, "Deformed Cortez");
    skins.insert(191, "Deformed Lance Vance");
    skins.insert(192, "Thief #1");
    skins.insert(193, "Thief #2");
    skins.insert(194, "Thief #3");
    skins
});

pub static VEHICLE_NAMES: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
    let mut vehicles = HashMap::new();
    vehicles.insert(130, "Landstalker");
    vehicles.insert(131, "Idaho");
    vehicles.insert(132, "Stinger");
    vehicles.insert(133, "Linerunner");
    vehicles.insert(134, "Perennial");
    vehicles.insert(135, "Sentinel");
    vehicles.insert(136, "Rio");
    vehicles.insert(137, "Firetruck");
    vehicles.insert(138, "Trashmaster");
    vehicles.insert(139, "Stretch");
    vehicles.insert(140, "Manana");
    vehicles.insert(141, "Infernus");
    vehicles.insert(142, "Voodoo");
    vehicles.insert(143, "Pony");
    vehicles.insert(144, "Mule");
    vehicles.insert(145, "Cheetah #1");
    vehicles.insert(146, "Ambulance");
    vehicles.insert(147, "FBI Washington");
    vehicles.insert(148, "Moonbeam");
    vehicles.insert(149, "Esperanto");
    vehicles.insert(150, "Taxi");
    vehicles.insert(151, "Washington");
    vehicles.insert(152, "Bobcat");
    vehicles.insert(153, "Mr Whoopee");
    vehicles.insert(154, "BF Injection");
    vehicles.insert(155, "Hunter");
    vehicles.insert(156, "Police");
    vehicles.insert(157, "Enforcer");
    vehicles.insert(158, "Securicar");
    vehicles.insert(159, "Banshee");
    vehicles.insert(160, "Predator");
    vehicles.insert(161, "Bus");
    vehicles.insert(162, "Rhino");
    vehicles.insert(163, "Barracks OL");
    vehicles.insert(164, "Cuban Hermes");
    vehicles.insert(166, "Angel");
    vehicles.insert(167, "Coach");
    vehicles.insert(168, "Cabbie");
    vehicles.insert(169, "Stallion");
    vehicles.insert(170, "Rumpo");
    vehicles.insert(171, "RC Bandit");
    vehicles.insert(172, "Romero's Hearse");
    vehicles.insert(173, "Packer");
    vehicles.insert(174, "Sentinel XS");
    vehicles.insert(175, "Admiral");
    vehicles.insert(176, "Squalo");
    vehicles.insert(177, "Sea Sparrow");
    vehicles.insert(178, "Pizza boy");
    vehicles.insert(179, "Gang Burrito");
    vehicles.insert(182, "Speeder");
    vehicles.insert(183, "Reefer");
    vehicles.insert(184, "Tropic");
    vehicles.insert(185, "Flatbed");
    vehicles.insert(186, "Yankee");
    vehicles.insert(187, "Caddy");
    vehicles.insert(188, "Zebra Cab");
    vehicles.insert(189, "Top Fun");
    vehicles.insert(190, "Skimmer");
    vehicles.insert(191, "PCJ 600");
    vehicles.insert(192, "Faggio");
    vehicles.insert(193, "Freeway");
    vehicles.insert(194, "RC Baron");
    vehicles.insert(195, "RC Raider");
    vehicles.insert(196, "Glendale");
    vehicles.insert(197, "Oceanic");
    vehicles.insert(198, "Sanchez");
    vehicles.insert(199, "Sparrow");
    vehicles.insert(200, "Patriot");
    vehicles.insert(201, "Love Fist");
    vehicles.insert(202, "Coast Guard");
    vehicles.insert(203, "Dinghy");
    vehicles.insert(204, "Hermes");
    vehicles.insert(205, "Sabre");
    vehicles.insert(206, "Sabre Turbo");
    vehicles.insert(207, "Phoenix");
    vehicles.insert(208, "Walton");
    vehicles.insert(209, "Regina");
    vehicles.insert(210, "Comet");
    vehicles.insert(211, "Deluxo");
    vehicles.insert(212, "Burrito");
    vehicles.insert(213, "Spand Express");
    vehicles.insert(214, "Marquis");
    vehicles.insert(215, "Baggage Handler");
    vehicles.insert(216, "Kaufman Cab");
    vehicles.insert(217, "Maverick");
    vehicles.insert(218, "VCN Maverick");
    vehicles.insert(219, "Rancher");
    vehicles.insert(220, "FBI Rancher");
    vehicles.insert(221, "Virgo");
    vehicles.insert(222, "Greenwood");
    vehicles.insert(223, "Cuban Jetmax");
    vehicles.insert(224, "Hotring Racer #1");
    vehicles.insert(225, "Sandking");
    vehicles.insert(226, "Blista Compact");
    vehicles.insert(227, "Police Maverick");
    vehicles.insert(228, "Boxville");
    vehicles.insert(229, "Benson");
    vehicles.insert(230, "Mesa Grande");
    vehicles.insert(231, "RC Goblin");
    vehicles.insert(232, "Hotring Racer #2");
    vehicles.insert(233, "Hotring Racer #3");
    vehicles.insert(234, "Bloodring Banger #1");
    vehicles.insert(235, "Bloodring Banger #2");
    vehicles.insert(236, "Cheetah #2");
    vehicles

});

pub static WEAPON_NAMES: LazyLock<HashMap<i32, &str>> = LazyLock::new(|| {
    let mut weapons = HashMap::new();
    weapons.insert(0, "Fists");
    weapons.insert(1, "Brass Knuckles");
    weapons.insert(2, "Screwdriver");
    weapons.insert(3, "Golfclub");
    weapons.insert(4, "Nitestick");
    weapons.insert(5, "Knife");
    weapons.insert(6, "Baseball bat");
    weapons.insert(7, "Hammer");
    weapons.insert(8, "Meat Cleaver");
    weapons.insert(9, "Machete");
    weapons.insert(10, "Katana");
    weapons.insert(11, "Chainsaw");
    weapons.insert(12, "Grenades");
    weapons.insert(13, "Remote Grenades");
    weapons.insert(14, "Teargas");
    weapons.insert(15, "Molotov Cocktails");
    weapons.insert(16, "Missile");
    weapons.insert(17, "Colt .45");
    weapons.insert(18, "Python");
    weapons.insert(19, "Shotgun");
    weapons.insert(20, "Spaz Shotgun");
    weapons.insert(21, "Stubby Shotgun");
    weapons.insert(22, "Tec 9");
    weapons.insert(23, "Uzi");
    weapons.insert(24, "Ingram");
    weapons.insert(25, "MP5");
    weapons.insert(26, "M4");
    weapons.insert(27, "Ruger");
    weapons.insert(28, "Sniper Rifle");
    weapons.insert(29, "Laser Sniper");
    weapons.insert(30, "Rocket Launcher");
    weapons.insert(31, "Flame Thrower");
    weapons.insert(32, "M60");
    weapons.insert(33, "Minigun");
    weapons.insert(34, "Bomb");
    weapons.insert(35, "Helicannon");
    weapons.insert(36, "Camera");
    weapons.insert(39, "Vehicle");
    weapons.insert(40, "Heli-blades");
    weapons.insert(41, "Explosion");
    weapons.insert(42, "Drive-By");
    weapons.insert(43, "Drown");
    weapons.insert(44, "Fall");
    weapons.insert(51, "Explosion");
    weapons.insert(60, "Heli-blades");
    weapons.insert(70, "Suicide");
    weapons
});

pub static WEAPON_MODELS: LazyLock<HashMap<i32, i32>> = LazyLock::new(|| {
    let mut models = HashMap::new();
    models.insert(0, 293);
    models.insert(1, 259);
    models.insert(2, 260);
    models.insert(3, 261);
    models.insert(4, 262);
    models.insert(5, 263);
    models.insert(6, 264);
    models.insert(7, 265);
    models.insert(8, 266);
    models.insert(9, 267);
    models.insert(10, 268);
    models.insert(11, 269);
    models.insert(12, 270);
    models.insert(13, 291);
    models.insert(14, 271);
    models.insert(15, 272);
    models.insert(16, 273);
    models.insert(17, 274);
    models.insert(18, 275);
    models.insert(19, 277);
    models.insert(20, 278);
    models.insert(21, 279);
    models.insert(22, 281);
    models.insert(23, 282);
    models.insert(24, 283);
    models.insert(25, 284);
    models.insert(26, 280);
    models.insert(27, 276);
    models.insert(28, 285);
    models.insert(29, 286);
    models.insert(30, 287);
    models.insert(31, 288);
    models.insert(32, 289);
    models.insert(33, 290);
    models.insert(36, 292);
    models
});

#[pyfunction(signature = (x, y, *polies))]
pub fn in_poly(x: f64, y: f64, polies: Vec<(f64, f64)>) -> bool {
    let mut crossings = 0;
    let n = polies.len();

    if n < 3 {
        return false; // Not a polygon
    }

    for i in 0..n {
        let (x1, y1) = polies[i];
        let (x2, y2) = polies[(i + 1) % n];

        // Ensure segment is checked from left to right
        let (left_x, right_x, left_y, right_y) = if x1 < x2 {
            (x1, x2, y1, y2)
        } else {
            (x2, x1, y2, y1)
        };

        // Check if ray can potentially cross segment
        if x > left_x && x <= right_x && (y < left_y || y <= right_y) {
            let dx = right_x - left_x;
            let dy = right_y - left_y;

            // Avoid division by zero for vertical lines
            let k = if dx.abs() < 1e-6 {
                std::f64::INFINITY
            } else {
                dy / dx
            };

            let m = left_y - k * left_x;

            // Calculate intersection y-coordinate
            let y_intersect = k * x + m;

            // Check if ray crosses segment
            if y <= y_intersect {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

#[pyfunction]
pub fn distance_from_point(x: f64, y: f64, x1: f64, y1: f64) -> f64 {
    f64::sqrt((x - x1).powi(2) + (y - y1).powi(2))
}

#[pyfunction]
pub fn get_district_name(x: f64, y: f64) -> String {
    if (-213.73..=-1613.03).contains(&x) && (413.218..=1677.32).contains(&y) {
        "Downtown Vice City".to_string()
    } else if (163.656..=-213.73).contains(&x) && (-351.153..=-930.526).contains(&y) {
        "Vice Point".to_string()
    } else if (-103.97..=-213.73).contains(&x) && (-930.526..=-1805.37).contains(&y) {
        "Washington Beach".to_string()
    } else if (-253.206..=-1888.21).contains(&x) && (-1805.37..=-1779.61).contains(&y) {
        "Ocean Beach".to_string()
    } else if (-748.206..=-104.505).contains(&x) && (-818.266..=-241.467).contains(&y) {
        "Starfish Island".to_string()
    } else if (-213.73..=-104.505).contains(&x) && (797.605..=-241.467).contains(&y) {
        "Prawn Island".to_string()
    } else if (-213.73..=-104.505).contains(&x) && (-241.429..=797.605).contains(&y) {
        "Leaf Links".to_string()
    } else if (-1396.76..=-1208.21).contains(&x) && (-42.9113..=-1779.61).contains(&y) {
        "Junkyard".to_string()
    } else if (-1208.21..=-748.206).contains(&x) && (-898.738..=-241.467).contains(&y) {
        "Little Havana".to_string()
    } else if (-1208.21..=-578.289).contains(&x) && (-241.467..=412.66).contains(&y) {
        "Little Haiti".to_string()
    } else {
        "Vice City".to_string()
    }
}

/*
    Const for python side
*/

pub fn submodule_util(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_district_name, m)?)?;
    m.add_function(wrap_pyfunction!(distance_from_point, m)?)?;
    m.add_function(wrap_pyfunction!(in_poly, m)?)?;
    m.add("SKINS", SKINS.clone().into_py_dict(py)?)?;
    m.add("VEHICLE_NAMES", VEHICLE_NAMES.clone().into_py_dict(py)?)?;
    m.add("WEAPON_NAMES", WEAPON_NAMES.clone().into_py_dict(py)?)?;
    m.add("WEAPON_MODELS", WEAPON_MODELS.clone().into_py_dict(py)?)?;
    m.add("VEHICLE_CLASSIC_CAR", [
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
    ])?;
    m.add("VEHICLE_CLASSIC_BOAT", [136, 160, 176, 182, 183, 184, 190, 202, 203, 214, 223])?;
    m.add("VEHICLE_CLASSIC_AIR", [155, 177, 199, 217, 218, 227])?;
    m.add("VEHICLE_CLASSIC_BIKE", [166, 178, 191, 192, 193, 198])?;
    m.add("VEHICLE_CLASSIC_RC", [171, 194, 195, 231])?;

    Ok(())
}
