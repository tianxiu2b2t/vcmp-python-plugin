#pragma once
#ifndef PYMODULE
#define PYMODULE
#include "plugin.h"
#include "logger.h"
#include <pybind11/embed.h>
#include "meta.hpp"

namespace py = pybind11;

py::object pfunctions = py::none();
py::object pcallbacks = py::none();

PluginFuncs* vfuncs;
PluginCallbacks* vcalls;

#define DEFAULT_RETURN py::int_(1)
map<int, string> vcmpErrorMappings = {
	{vcmpErrorNoSuchEntity, "No such entity."},
	{vcmpErrorBufferTooSmall, "Buffer too small."},
	{vcmpErrorTooLargeInput, "Too large input."},
	{vcmpErrorArgumentOutOfBounds, "Argument out of bounds."},
	{vcmpErrorNullArgument, "Null argument."},
	{vcmpErrorPoolExhausted, "Pool exhausted."},
	{vcmpErrorInvalidName, "Invalid name."},
	{vcmpErrorRequestDenied, "Request denied."},
	{forceSizeVcmpError, "Unknown Error"}
};

void raiseException(std::string extra = "") {
    try {
        throw; 
    } catch (py::error_already_set& e) {
        bool shutdown = false;
        if (e.matches(PyExc_KeyboardInterrupt) || e.matches(PyExc_SystemExit))
            shutdown = true;

        logger.error("Python error: " + std::string(e.what()) + (extra != "" ? " (Extra message: " + extra + ")" : ""));
        if (shutdown) {
            vfuncs->ShutdownServer();
        }
    } catch (std::exception& e) {
        logger.error("Python error: " + std::string(e.what()) + (extra != "" ? " (Extra message: " + extra + ")" : ""));
    } catch (...) {
        logger.error("Python error: Unknown error" + (extra != "" ? " (Extra message: " + extra + ")" : ""));
    }
}

py::dict createVector(
	float x,
	float y,
	float z
) {
	py::dict ret = py::dict();
	ret["x"] = x;
	ret["y"] = y;
	ret["z"] = z;
	return ret;
}

py::dict createQuaternion(
	float x,
	float y,
	float z,
	float w
) {
	py::dict ret = py::dict();
	ret["x"] = x;
	ret["y"] = y;
	ret["z"] = z;
	ret["w"] = w;
	return ret;
}

py::object handlePythonFunction(
    std::string function,
    py::object defaultValue = py::none(),
    std::function<py::object(py::object)> callbacks = [](py::object func) {
        return func();
    }
) {
    std::string name = "on_" + function;
    try {
		if (pcallbacks.is_none()) {
			logger.error("Callbacks not initialized");
			return defaultValue;
		}
		py::module m = pcallbacks.cast<py::module>();
		if (!py::hasattr(m, name.c_str()) || m.attr(name.c_str()).is_none() || !py::isinstance<py::function>(m.attr(name.c_str()))) {
            m.def(name.c_str(), [](py::args args, py::kwargs kwargs) {
                // Do nothing
            });
            logger.debug("Create empty callback " + name);
        }
        auto func = m.attr(name.c_str());
        if (py::isinstance<py::function>(func)) {
            py::object ret = callbacks(func);
			if (ret.is_none()) {
				return defaultValue;
			}
			return ret;
        }
    } catch (...) {
        raiseException("Failed to call Python callback " + name);
    }
    return defaultValue;
}

void throwVCMPError(vcmpError error, std::string extra = "") {
	if (error == vcmpErrorNone) return;
	std::string message = vcmpErrorMappings[(int)error];
    if (extra != "") message += " (Extra message: " + extra + ")";
    throw pybind11::value_error(message);
}
/* function */
string getSomethingFromVCMP(
	function<vcmpError(char*, size_t)> func,
	string extra = ""
) {
	vcmpError error = vcmpErrorBufferTooSmall;
	char buffer[256];
	while (error == vcmpErrorBufferTooSmall) {
		error = func(buffer, sizeof(buffer));
		if (error == vcmpErrorNone) {
			string ret = gbk_to_utf8(std::string(buffer));
			// remove ending \0
			if (ret.length() > 0 && ret[ret.length() - 1] == '\0') {
				ret = ret.substr(0, ret.length() - 1);
			}
			return ret;
		}
	}
    throwVCMPError(error, extra);
	return "";
}

void bindVCMPFunctions() {
    if (vfuncs == nullptr) {
        logger.error("Functions not initialized");
        return;
    }
    py::module m = pfunctions.cast<py::module>();
    logger.debug("Start bind VCMP functions to Python module");
	m.def("set_vcmp_python_debug", [](bool debug) {
		logger.setDebug(debug);
	});

	m.def("get_vcmp_python_debug", []() {
		return logger.DEBUG;
	});

	m.def("get_vcmp_python_version", []() {
		return PLUGIN_VERSION;
	});

	m.def("get_vcmp_python_author", []() {
		return AUTHOR;
	});
	
	m.def("get_vcmp_python_license", []() {
		return LICENSE;
	});

	m.def("get_vcmp_python_github", []() {
		return GITHUB;
	});

	m.def("get_vcmp_python_name", []() {
		return PLUGIN_NAME;
	});

	m.def("get_server_version", []() {
		return funcs->GetServerVersion();
	});

	m.def("get_server_settings", []() {
		ServerSettings settings;
		throwVCMPError(funcs->GetServerSettings(&settings), "Failed to get server settings.");
		pybind11::dict ret = pybind11::dict();
		ret["max_players"] = settings.maxPlayers;
		ret["port"] = settings.port;
		ret["flags"] = settings.flags;
		ret["servername"] = settings.serverName;
		return ret;
	});

	m.def("get_number_of_plugins", []() {
		return funcs->GetNumberOfPlugins();
	});

	m.def("get_plugin_info", [](int32_t pluginId) {
		PluginInfo info;
		throwVCMPError(funcs->GetPluginInfo(pluginId, &info), "Failed to get plugin info.");
		pybind11::dict ret = pybind11::dict();
		ret["name"] = info.name;
		ret["plugin_version"] = info.pluginVersion;
		ret["plugin_id"] = info.pluginId;
		ret["api_major_version"] = info.apiMajorVersion;
		ret["api_minor_version"] = info.apiMinorVersion;
		ret["struct_size"] = info.structSize;
		return ret;
	});

	m.def("find_plugin", [](const char* pluginName) {
		return funcs->FindPlugin(pluginName);
	});

	m.def("send_plugin_command", [](uint32_t commandIdentifier, const char* message) {
		throwVCMPError(
			funcs->SendPluginCommand(commandIdentifier, "%s", message), "Failed to send plugin command."
		);
		return;
	});

	m.def("get_time", []() {
		return funcs->GetTime();
	});

	m.def("get_last_error", []() {
		return (int32_t)funcs->GetLastError();
	});

	m.def("send_client_script_data", [](int32_t playerId, pybind11::bytes bytes) {
		std::string data = bytes;
		throwVCMPError(funcs->SendClientScriptData(playerId, data.c_str(), data.length()), "Failed to send client script data.");
	});

	m.def("send_client_message", [](int32_t playerId, uint32_t colour, const char* message) {
		throwVCMPError(
			funcs->SendClientMessage(playerId, colour, message), "Failed to send client message."
		);
	});

	m.def("send_game_message", [](int32_t playerId, int32_t type, const char* message) {
		throwVCMPError(
			funcs->SendGameMessage(playerId, type, message), "Failed to send game message."
		);
	});

	m.def("set_server_name", [](const char* text) {
		throwVCMPError(
			funcs->SetServerName(utf8_to_gbk(std::string(text)).c_str()), "Failed to set server name."
		);
	});

	m.def("get_server_name", []() {
		return getSomethingFromVCMP(funcs->GetServerName, "Failed to get server name.");
	});

	m.def("set_max_players", [](uint32_t maxPlayers) {
		throwVCMPError(
			funcs->SetMaxPlayers(maxPlayers), "Failed to set max players."
		);
	});

	m.def("get_max_players", []() {
		return funcs->GetMaxPlayers();
	});

	m.def("set_server_password", [](const char* password) {
		return funcs->SetServerPassword(password);
	});

	m.def("get_server_password", []() {
		return getSomethingFromVCMP(funcs->GetServerPassword, "Failed to get server password.");
	});

	m.def("set_game_mode_text", [](const char* gameMode) {
		throwVCMPError(
			funcs->SetGameModeText(utf8_to_gbk(std::string(gameMode)).c_str()), "Failed to set game mode text."
		);
	});

	m.def("get_game_mode_text", []() {
		return getSomethingFromVCMP(funcs->GetGameModeText, "Failed to get game mode text.");
	});

	m.def("shutdown_server", []() {
		return funcs->ShutdownServer();
	});


	m.def("set_server_option", [](int32_t option, bool toggle) {
		throwVCMPError(
			funcs->SetServerOption(static_cast<vcmpServerOption>(option), toggle), "Failed to set server option."
		);
	});

	m.def("get_server_option", [](int32_t option) {
		return pybind11::bool_(funcs->GetServerOption(static_cast<vcmpServerOption>(option)) != 0);
	});

	m.def("set_world_bounds", [](float maxX, float minX, float maxY, float minY) {
		funcs->SetWorldBounds(maxX, minX, maxY, minY);
	});

	m.def("get_world_bounds", []() {
		float maxXOut, minXOut, maxYOut, minYOut;
		funcs->GetWorldBounds(&maxXOut, &minXOut, &maxYOut, &minYOut);
		pybind11::dict ret = pybind11::dict();
		ret["max_x"] = maxXOut;
		ret["min_x"] = minXOut;
		ret["max_y"] = maxYOut;
		ret["min_y"] = minYOut;
		return ret;
	});

	m.def("set_wasted_settings", [](uint32_t deathTimer, uint32_t fadeTimer, float fadeInSpeed, float fadeOutSpeed, uint32_t fadeColour, uint32_t corpseFadeStart, uint32_t corpseFadeTime) {
		funcs->SetWastedSettings(deathTimer, fadeTimer, fadeInSpeed, fadeOutSpeed, fadeColour, corpseFadeStart, corpseFadeTime);
	});

	m.def("get_wasted_settings", []() {
		uint32_t deathTimer, fadeTimer, fadeColour, corpseFadeStart, corpseFadeTime;
		float fadeInSpeed, fadeOutSpeed;
		funcs->GetWastedSettings(&deathTimer, &fadeTimer, &fadeInSpeed, &fadeOutSpeed, &fadeColour, &corpseFadeStart, &corpseFadeTime);
		pybind11::dict ret = pybind11::dict();
		ret["death_timer"] = deathTimer;
		ret["fade_timer"] = fadeTimer;
		ret["fade_in_speed"] = fadeInSpeed;
		ret["fade_out_speed"] = fadeOutSpeed;
		ret["fade_colour"] = fadeColour;
		ret["corpse_fade_start"] = corpseFadeStart;
		ret["corpse_fade_time"] = corpseFadeTime;
		return ret;
	});

	m.def("set_time_rate", [](int32_t timeRate) {
		funcs->SetTimeRate(timeRate);
	});

	m.def("get_time_rate", []() {
		return funcs->GetTimeRate();
	});


	m.def("set_hour", [](int32_t hour) {
		funcs->SetHour(hour);
	});

	m.def("get_hour", []() {
		return funcs->GetHour();
	});

	m.def("set_minute", [](int32_t minute) {
		funcs->SetMinute(minute);
	});

	m.def("get_minute", []() {
		return funcs->GetMinute();
	});


	m.def("set_weather", [](int32_t weather) {
		funcs->SetWeather(weather);
	});

	m.def("get_weather", []() {
		return funcs->GetWeather();
	});

	m.def("set_gravity", [](float gravity) {
		funcs->SetGravity(gravity);
	});

	m.def("get_gravity", []() {
		return funcs->GetGravity();
	});

	m.def("set_game_speed", [](float gameSpeed) {
		funcs->SetGameSpeed(gameSpeed);
	});

	m.def("get_game_speed", []() {
		return funcs->GetGameSpeed();
	});

	m.def("set_water_level", [](float waterLevel) {
		funcs->SetWaterLevel(waterLevel);
	});

	m.def("get_water_level", []() {
		return funcs->GetWaterLevel();
	});

	m.def("set_maximum_flight_altitude", [](float height) {
		funcs->SetMaximumFlightAltitude(height);
	});

	m.def("get_maximum_flight_altitude", []() {
		return funcs->GetMaximumFlightAltitude();
	});

	m.def("set_kill_command_delay", [](int32_t delay) {
		funcs->SetKillCommandDelay(delay);
	});

	m.def("get_kill_command_delay", []() {
		return funcs->GetKillCommandDelay();
	});

	m.def("set_vehicles_forced_respawn_height", [](float height) {
		funcs->SetVehiclesForcedRespawnHeight(height);
	});

	m.def("get_vehicles_forced_respawn_height", []() {
		return funcs->GetVehiclesForcedRespawnHeight();
	});

	m.def("play_sound", [](int32_t worldId, int32_t soundId, float x, float y, float z) {
		throwVCMPError(
			funcs->PlaySound(worldId, soundId, x, y, z), "Failed to play sound."
		);
	});

	m.def("create_explosion", [](int32_t worldId, int32_t type, float x, float y, float z, int32_t responsiblePlayerId, uint8_t atGroundLevel) {
		throwVCMPError(
			funcs->CreateExplosion(worldId, type, x, y, z, responsiblePlayerId, atGroundLevel), "Failed to create explosion."
		);
	});

	m.def("hide_map_object", [](int32_t modelId, int16_t tenthX, int16_t tenthY, int16_t tenthZ) {
		funcs->HideMapObject(modelId, tenthX, tenthY, tenthZ);
	});

	m.def("show_map_object", [](int32_t modelId, int16_t tenthX, int16_t tenthY, int16_t tenthZ) {
		funcs->ShowMapObject(modelId, tenthX, tenthY, tenthZ);
	});

	m.def("show_all_map_objects", []() {
		funcs->ShowAllMapObjects();
	});

	m.def("set_weapon_data_value", [](int32_t weaponId, int32_t fieldId, double value) {
		throwVCMPError(funcs->SetWeaponDataValue(weaponId, fieldId, value), "Failed to set weapon data value.");
	});

	m.def("get_weapon_data_value", [](int32_t weaponId, int32_t fieldId) {
		return funcs->GetWeaponDataValue(weaponId, fieldId);
	});

	m.def("reset_weapon_data_value", [](int32_t weaponId, int32_t fieldId) {
		throwVCMPError(funcs->ResetWeaponDataValue(weaponId, fieldId), "Failed to reset weapon data value.");
	});

	m.def("is_weapon_data_value_modified", [](int32_t weaponId, int32_t fieldId) {
		return funcs->IsWeaponDataValueModified(weaponId, fieldId);
	});

	m.def("reset_weapon_data", [](int32_t weaponId) {
		throwVCMPError(funcs->ResetWeaponData(weaponId), "Failed to reset weapon data.");
	});

	m.def("reset_all_weapon_data", []() {
		funcs->ResetAllWeaponData();
	});

	m.def("get_key_bind_unused_slot", []() {
		return funcs->GetKeyBindUnusedSlot();
	});

	m.def("get_key_bind_data", [](int32_t bindId) {
		uint8_t isCalledOnReleaseOut;
		int32_t keyOneOut, keyTwoOut, keyThreeOut;
		throwVCMPError(
			funcs->GetKeyBindData(bindId, &isCalledOnReleaseOut, &keyOneOut, &keyTwoOut, &keyThreeOut), "Failed to get key bind data."
		);
		return pybind11::make_tuple(isCalledOnReleaseOut, keyOneOut, keyTwoOut, keyThreeOut);
	});

	m.def("register_key_bind", [](int32_t bindId, uint8_t isCalledOnRelease, int32_t keyOne, int32_t keyTwo, int32_t keyThree) {
		throwVCMPError(
			funcs->RegisterKeyBind(bindId, isCalledOnRelease, keyOne, keyTwo, keyThree), "Failed to register key bind."
		);
	});

	m.def("remove_key_bind", [](int32_t bindId) {
		throwVCMPError(
			funcs->RemoveKeyBind(bindId), "Failed to remove key bind."
		);
	});

	m.def("remove_all_key_binds", []() {
		funcs->RemoveAllKeyBinds();
	});

	m.def("create_coord_blip", [](int32_t index, int32_t world, float x, float y, float z, int32_t scale, uint32_t colour, int32_t sprite) {
		return funcs->CreateCoordBlip(index, world, x, y, z, scale, colour, sprite);
	});

	m.def("destroy_coord_blip", [](int32_t index) {
		throwVCMPError(
			funcs->DestroyCoordBlip(index), "Failed to destroy coord blip."
		);
	});

	m.def("get_coord_blip_info", [](int32_t index) {
		int32_t world, scale, sprite;
		uint32_t colour;
		float x, y, z;
		throwVCMPError(
			funcs->GetCoordBlipInfo(index, &world, &x, &y, &z, &scale, &colour, &sprite), "Failed to get coord blip info."
		);
		return pybind11::make_tuple(world, x, y, z, scale, colour, sprite);
	});

	m.def("add_radio_stream", [](int32_t radioId, const char* radioName, const char* radioUrl, uint8_t isListed) {
		throwVCMPError(funcs->AddRadioStream(radioId, radioName, radioUrl, isListed), "Failed to add radio stream");
	});

	m.def("remove_radio_stream", [](int32_t radioId) {
		throwVCMPError(
			funcs->RemoveRadioStream(radioId), "Failed to remove radio stream."
		);
	});

	m.def("add_player_class", [](int32_t teamId, uint32_t colour, int32_t modelIndex, float x, float y, float z, float angle, int32_t weaponOne, int32_t weaponOneAmmo, int32_t weaponTwo, int32_t weaponTwoAmmo, int32_t weaponThree, int32_t weaponThreeAmmo) {
		return funcs->AddPlayerClass(teamId, colour, modelIndex, x, y, z, angle, weaponOne, weaponOneAmmo, weaponTwo, weaponTwoAmmo, weaponThree, weaponThreeAmmo);
	});

	m.def("set_spawn_player_position", [](float x, float y, float z) {
		funcs->SetSpawnPlayerPosition(x, y, z);
	});

	m.def("set_spawn_camera_position", [](float x, float y, float z) {
		funcs->SetSpawnCameraPosition(x, y, z);
	});

	m.def("set_spawn_camera_look_at", [](float x, float y, float z) {
		funcs->SetSpawnCameraLookAt(x, y, z);
	});


	m.def("is_player_admin", [](int32_t playerId) {
		return funcs->IsPlayerAdmin(playerId);
	});

	m.def("set_player_admin", [](int32_t playerId, bool toggle) {
		throwVCMPError(funcs->SetPlayerAdmin(playerId, toggle), "Failed to set player admin.");
	});

	m.def("get_player_ip", [](int32_t playerId) {
		return getSomethingFromVCMP(
			[&playerId](char* buffer, size_t size) {
				return funcs->GetPlayerIP(playerId, buffer, size);
			}, "Failed to get player IP."
		);
	});

	m.def("get_player_uid", [](int32_t playerId) {
		return getSomethingFromVCMP(
			[&playerId](char* buffer, size_t size) {
				return funcs->GetPlayerUID(playerId, buffer, size);
			}, "Failed to get player UID."
		);
	});
	m.def("get_player_uid2", [](int32_t playerId) {
		return getSomethingFromVCMP(
			[&playerId](char* buffer, size_t size) {
				return funcs->GetPlayerUID2(playerId, buffer, size);
			}, "Failed to get player UID2."
		);
	});

	m.def("kick_player", [](int32_t playerId) {
		throwVCMPError(funcs->KickPlayer(playerId), "Failed to kick player.");
	});

	m.def("ban_player", [](int32_t playerId) {
		throwVCMPError(funcs->BanPlayer(playerId), "Failed to ban player.");
	});

	m.def("ban_ip", [](char* ipAddress) {
		funcs->BanIP(ipAddress);
	});

	m.def("unban_ip", [](char* ipAddress) {
		return funcs->UnbanIP(ipAddress);
	});

	m.def("is_ip_banned", [](char* ipAddress) {
		return funcs->IsIPBanned(ipAddress);
	});

	m.def("get_player_id_from_name", [](const char* name) {
		return funcs->GetPlayerIdFromName(name);
	});

	m.def("is_player_connected", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerConnected(playerId));
	});

	m.def("is_player_streamed_for_player", [](int32_t checkedPlayerId, int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerStreamedForPlayer(checkedPlayerId, playerId));
	});

	m.def("get_player_key", [](int32_t playerId) {
		return funcs->GetPlayerKey(playerId);
	});

	m.def("get_player_name", [](int32_t playerId) {
		return getSomethingFromVCMP(
			[&playerId](char* buffer, size_t size) {
				return funcs->GetPlayerName(playerId, buffer, size);
			}, "Failed to get player name."
		);
	});

	m.def("set_player_name", [](int32_t playerId, const char* name) {
		throwVCMPError(funcs->SetPlayerName(playerId, name), "Failed to set player name.");
	});

	m.def("get_player_state", [](int32_t playerId) {
		return funcs->GetPlayerState(playerId);
	});

	m.def("set_player_option", [](int32_t playerId, int32_t option, bool toggle) {
		throwVCMPError(funcs->SetPlayerOption(playerId, static_cast<vcmpPlayerOption>(option), toggle), "Failed to set player option.");
	});

	m.def("get_player_option", [](int32_t playerId, int32_t option) {
		return pybind11::bool_(funcs->GetPlayerOption(playerId, static_cast<vcmpPlayerOption>(option)));
	});

	m.def("set_player_world", [](int32_t playerId, int32_t world) {
		throwVCMPError(funcs->SetPlayerWorld(playerId, world), "Failed to set player world.");
	});

	m.def("get_player_world", [](int32_t playerId) {
		return funcs->GetPlayerWorld(playerId);
	});

	m.def("set_player_secondary_world", [](int32_t playerId, int32_t secondaryWorld) {
		throwVCMPError(funcs->SetPlayerSecondaryWorld(playerId, secondaryWorld), "Failed to set player secondary world.");
	});

	m.def("get_player_secondary_world", [](int32_t playerId) {
		return funcs->GetPlayerSecondaryWorld(playerId);
	});

	m.def("get_player_unique_world", [](int32_t playerId) {
		return funcs->GetPlayerUniqueWorld(playerId);
	});

	m.def("is_player_world_compatible", [](int32_t playerId, int32_t world) {
		return funcs->IsPlayerWorldCompatible(playerId, world);
	});

	m.def("get_player_class", [](int32_t playerId) {
		return funcs->GetPlayerClass(playerId);
	});

	m.def("set_player_team", [](int32_t playerId, int32_t teamId) {
		throwVCMPError(funcs->SetPlayerTeam(playerId, teamId), "Failed to set player team.");
	});

	m.def("get_player_team", [](int32_t playerId) {
		return funcs->GetPlayerTeam(playerId);
	});

	m.def("set_player_skin", [](int32_t playerId, int32_t skinId) {
		throwVCMPError(funcs->SetPlayerSkin(playerId, skinId), "Failed to set player skin.");
	});

	m.def("get_player_skin", [](int32_t playerId) {
		return funcs->GetPlayerSkin(playerId);
	});

	m.def("set_player_colour", [](int32_t playerId, uint32_t colour) {
		throwVCMPError(funcs->SetPlayerColour(playerId, colour), "Failed to set player colour.");
	});

	m.def("get_player_colour", [](int32_t playerId) {
		return funcs->GetPlayerColour(playerId);
	});

	m.def("is_player_spawned", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerSpawned(playerId));
	});

	m.def("force_player_spawn", [](int32_t playerId) {
		throwVCMPError(funcs->ForcePlayerSpawn(playerId), "Failed to force player spawn.");
	});

	m.def("force_player_select", [](int32_t playerId) {
		throwVCMPError(funcs->ForcePlayerSelect(playerId), "Failed to force player select.");
	});

	m.def("force_all_select", []() {
		funcs->ForceAllSelect();
	});

	m.def("is_player_typing", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerTyping(playerId));
	});

	m.def("give_player_money", [](int32_t playerId, int32_t amount) {
		throwVCMPError(funcs->GivePlayerMoney(playerId, amount), "Failed to give player money.");
	});

	m.def("set_player_money", [](int32_t playerId, int32_t amount) {
		throwVCMPError(funcs->SetPlayerMoney(playerId, amount), "Failed to set player money.");
	});

	m.def("get_player_money", [](int32_t playerId) {
		return funcs->GetPlayerMoney(playerId);
	});

	m.def("set_player_score", [](int32_t playerId, int32_t score) {
		throwVCMPError(funcs->SetPlayerScore(playerId, score), "Failed to set player score.");
	});

	m.def("get_player_score", [](int32_t playerId) {
		return funcs->GetPlayerScore(playerId);
	});

	m.def("set_player_wanted_level", [](int32_t playerId, int32_t level) {
		throwVCMPError(funcs->SetPlayerWantedLevel(playerId, level), "Failed to set player wanted level.");
	});

	m.def("get_player_wanted_level", [](int32_t playerId) {
		return funcs->GetPlayerWantedLevel(playerId);
	});

	m.def("get_player_ping", [](int32_t playerId) {
		return funcs->GetPlayerPing(playerId);
	});

	m.def("get_player_fps", [](int32_t playerId) {
		return funcs->GetPlayerFPS(playerId);
	});

	m.def("set_player_health", [](int32_t playerId, float health) {
		throwVCMPError(funcs->SetPlayerHealth(playerId, health), "Failed to set player health.");
	});

	m.def("get_player_health", [](int32_t playerId) {
		return funcs->GetPlayerHealth(playerId);
	});

	m.def("set_player_armour", [](int32_t playerId, float armour) {
		throwVCMPError(funcs->SetPlayerArmour(playerId, armour), "Failed to set player armour.");
	});

	m.def("get_player_armour", [](int32_t playerId) {
		return funcs->GetPlayerArmour(playerId);
	});

	m.def("set_player_immunity_flags", [](int32_t playerId, uint32_t flags) {
		throwVCMPError(funcs->SetPlayerImmunityFlags(playerId, flags), "Failed to set player immunity flags.");
	});

	m.def("get_player_immunity_flags", [](int32_t playerId) {
		return funcs->GetPlayerImmunityFlags(playerId);
	});

	m.def("set_player_position", [](int32_t playerId, float x, float y, float z) {
		throwVCMPError(funcs->SetPlayerPosition(playerId, x, y, z), "Failed to set player position.");
	});

	m.def("get_player_position", [](int32_t playerId) {
		float x, y, z;
		throwVCMPError(funcs->GetPlayerPosition(playerId, &x, &y, &z), "Failed to get player position.");
		return createVector(x, y, z);
	});

	m.def("set_player_speed", [](int32_t playerId, float x, float y, float z) {
		throwVCMPError(funcs->SetPlayerSpeed(playerId, x, y, z), "Failed to set player speed.");
	});

	m.def("get_player_speed", [](int32_t playerId) {
		float x, y, z;
		throwVCMPError(funcs->GetPlayerSpeed(playerId, &x, &y, &z), "Failed to get player speed.");
		return createVector(x, y, z);
	});

	m.def("add_player_speed", [](int32_t playerId, float x, float y, float z) {
		throwVCMPError(funcs->AddPlayerSpeed(playerId, x, y, z), "Failed to add player speed.");
	});

	m.def("set_player_heading", [](int32_t playerId, float angle) {
		throwVCMPError(funcs->SetPlayerHeading(playerId, angle), "Failed to set player heading.");
	});

	m.def("get_player_heading", [](int32_t playerId) {
		return funcs->GetPlayerHeading(playerId);
	});

	m.def("set_player_alpha", [](int32_t playerId, int32_t alpha, uint32_t fadeTime) {
		throwVCMPError(funcs->SetPlayerAlpha(playerId, alpha, fadeTime), "Failed to set player alpha.");
	});

	m.def("get_player_alpha", [](int32_t playerId) {
		return funcs->GetPlayerAlpha(playerId);
	});

	m.def("get_player_aim_position", [](int32_t playerId) {
		float x, y, z;
		throwVCMPError(funcs->GetPlayerAimPosition(playerId, &x, &y, &z), "Failed to get player aim position.");
		return createVector(x, y, z);
	});

	m.def("get_player_aim_direction", [](int32_t playerId) {
		float x, y, z;
		throwVCMPError(funcs->GetPlayerAimDirection(playerId, &x, &y, &z), "Failed to get player aim direction.");
		return createVector(x, y, z);
	});

	m.def("is_player_on_fire", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerOnFire(playerId));
	});

	m.def("is_player_crouching", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerCrouching(playerId));
	});

	m.def("get_player_action", [](int32_t playerId) {
		return funcs->GetPlayerAction(playerId);
	});

	m.def("get_player_game_keys", [](int32_t playerId) {
		return funcs->GetPlayerGameKeys(playerId);
	});

	m.def("put_player_in_vehicle", [](int32_t playerId, int32_t vehicleId, int32_t slotIndex, uint8_t makeRoom, uint8_t warp) {
		throwVCMPError(funcs->PutPlayerInVehicle(playerId, vehicleId, slotIndex, makeRoom, warp), "Failed to put player in vehicle.");
	});

	m.def("remove_player_from_vehicle", [](int32_t playerId) {
		throwVCMPError(funcs->RemovePlayerFromVehicle(playerId), "Failed to remove player from vehicle.");
	});

	m.def("get_player_in_vehicle_status", [](int32_t playerId) {
		return funcs->GetPlayerInVehicleStatus(playerId);
	});

	m.def("get_player_in_vehicle_slot", [](int32_t playerId) {
		return funcs->GetPlayerInVehicleSlot(playerId);
	});

	m.def("get_player_vehicle_id", [](int32_t playerId) {
		return funcs->GetPlayerVehicleId(playerId);
	});

	m.def("give_player_weapon", [](int32_t playerId, int32_t weaponId, int32_t ammo) {
		throwVCMPError(funcs->GivePlayerWeapon(playerId, weaponId, ammo), "Failed to give player weapon.");
	});

	m.def("set_player_weapon", [](int32_t playerId, int32_t weaponId, int32_t ammo) {
		throwVCMPError(funcs->SetPlayerWeapon(playerId, weaponId, ammo), "Failed to set player weapon.");
	});

	m.def("get_player_weapon", [](int32_t playerId) {
		return funcs->GetPlayerWeapon(playerId);
	});

	m.def("get_player_weapon_ammo", [](int32_t playerId) {
		return funcs->GetPlayerWeaponAmmo(playerId);
	});

	m.def("set_player_weapon_slot", [](int32_t playerId, int32_t slot) {
		throwVCMPError(funcs->SetPlayerWeaponSlot(playerId, slot), "Failed to set player weapon slot.");
	});

	m.def("get_player_weapon_slot", [](int32_t playerId) {
		return funcs->GetPlayerWeaponSlot(playerId);
	});

	m.def("get_player_weapon_at_slot", [](int32_t playerId, int32_t slot) {
		return funcs->GetPlayerWeaponAtSlot(playerId, slot);
	});

	m.def("get_player_ammo_at_slot", [](int32_t playerId, int32_t slot) {
		return funcs->GetPlayerAmmoAtSlot(playerId, slot);
	});

	m.def("remove_player_weapon", [](int32_t playerId, int32_t weaponId) {
		throwVCMPError(funcs->RemovePlayerWeapon(playerId, weaponId), "Failed to remove player weapon.");
	});

	m.def("remove_all_weapons", [](int32_t playerId) {
		throwVCMPError(funcs->RemoveAllWeapons(playerId), "Failed to remove all weapons.");
	});

	m.def("set_camera_position", [](int32_t playerId, float posX, float posY, float posZ, float lookX, float lookY, float lookZ) {
		throwVCMPError(funcs->SetCameraPosition(playerId, posX, posY, posZ, lookX, lookY, lookZ), "Failed to set camera position.");
	});

	m.def("restore_camera", [](int32_t playerId) {
		throwVCMPError(funcs->RestoreCamera(playerId), "Failed to restore camera.");
	});

	m.def("is_camera_locked", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsCameraLocked(playerId));
	});

	m.def("set_player_animation", [](int32_t playerId, int32_t groupId, int32_t animationId) {
		throwVCMPError(funcs->SetPlayerAnimation(playerId, groupId, animationId), "Failed to set player animation.");
	});

	m.def("get_player_standing_on_vehicle", [](int32_t playerId) {
		return funcs->GetPlayerStandingOnVehicle(playerId);
	});

	m.def("get_player_standing_on_object", [](int32_t playerId) {
		return funcs->GetPlayerStandingOnObject(playerId);
	});

	m.def("is_player_away", [](int32_t playerId) {
		return pybind11::bool_(funcs->IsPlayerAway(playerId));
	});

	m.def("get_player_spectate_target", [](int32_t playerId) {
		return funcs->GetPlayerSpectateTarget(playerId);
	});

	m.def("set_player_spectate_target", [](int32_t playerId, int32_t targetId) {
		throwVCMPError(funcs->SetPlayerSpectateTarget(playerId, targetId), "Failed to set player spectate target.");
	});

	m.def("redirect_player_to_server", [](int32_t playerId, const char* ip, uint32_t port, const char* nick, const char* serverPassword, const char* userPassword) {
		throwVCMPError(funcs->RedirectPlayerToServer(playerId, ip, port, nick, serverPassword, userPassword), "Failed to redirect player to server.");
	});

	m.def("check_entity_exists", [](int32_t entityPool, int32_t index) {
		return pybind11::bool_(funcs->CheckEntityExists(static_cast<vcmpEntityPool>(entityPool), index));
	});




	m.def("create_vehicle", [](int32_t modelIndex, int32_t world, float x, float y, float z, float angle, int32_t primaryColour, int32_t secondaryColour) {
		return funcs->CreateVehicle(modelIndex, world, x, y, z, angle, primaryColour, secondaryColour);
	});

	m.def("delete_vehicle", [](int32_t vehicleId) {
		throwVCMPError(funcs->DeleteVehicle(vehicleId), "Failed to delete vehicle.");
	});

	m.def("set_vehicle_option", [](int32_t vehicleId, int option, bool toggle) {
		throwVCMPError(funcs->SetVehicleOption(vehicleId, static_cast<vcmpVehicleOption>(option), toggle), "Failed to set vehicle option.");
	});

	m.def("get_vehicle_option", [](int32_t vehicleId, int option) {
		return funcs->GetVehicleOption(vehicleId, static_cast<vcmpVehicleOption>(option));
	});

	m.def("get_vehicle_sync_source", [](int32_t vehicleId) {
		return funcs->GetVehicleSyncSource(vehicleId);
	});

	m.def("get_vehicle_sync_type", [](int32_t vehicleId) {
		return funcs->GetVehicleSyncType(vehicleId);
	});

	m.def("is_vehicle_streamed_for_player", [](int32_t vehicleId, int32_t playerId) {
		return pybind11::bool_(funcs->IsVehicleStreamedForPlayer(vehicleId, playerId));
	});

	m.def("set_vehicle_world", [](int32_t vehicleId, int32_t world) {
		throwVCMPError(funcs->SetVehicleWorld(vehicleId, world), "Failed to set vehicle world.");
	});

	m.def("get_vehicle_world", [](int32_t vehicleId) {
		return funcs->GetVehicleWorld(vehicleId);
	});

	m.def("get_vehicle_model", [](int32_t vehicleId) {
		return funcs->GetVehicleModel(vehicleId);
	});

	m.def("get_vehicle_occupant", [](int32_t vehicleId, int32_t slotIndex) {
		return funcs->GetVehicleOccupant(vehicleId, slotIndex);
	});

	m.def("respawn_vehicle", [](int32_t vehicleId) {
		throwVCMPError(funcs->RespawnVehicle(vehicleId), "Failed to respawn vehicle.");
	});

	m.def("set_vehicle_immunity_flags", [](int32_t vehicleId, uint32_t immunityFlags) {
		throwVCMPError(funcs->SetVehicleImmunityFlags(vehicleId, immunityFlags), "Failed to set vehicle immunity flags.");
	});

	m.def("get_vehicle_immunity_flags", [](int32_t vehicleId) {
		return funcs->GetVehicleImmunityFlags(vehicleId);
	});

	m.def("explode_vehicle", [](int32_t vehicleId) {
		throwVCMPError(funcs->ExplodeVehicle(vehicleId), "Failed to explode vehicle.");
	});

	m.def("is_vehicle_wrecked", [](int32_t vehicleId) {
		return pybind11::bool_(funcs->IsVehicleWrecked(vehicleId));
	});

	m.def("set_vehicle_position", [](int32_t vehicleId, float x, float y, float z, uint8_t removeOccupants) {
		throwVCMPError(funcs->SetVehiclePosition(vehicleId, x, y, z, removeOccupants), "Failed to set vehicle position.");
	});

	m.def("get_vehicle_position", [](int32_t vehicleId) {
		float x, y, z;
		throwVCMPError(funcs->GetVehiclePosition(vehicleId, &x, &y, &z), "Failed to get vehicle position.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_rotation", [](int32_t vehicleId, float x, float y, float z, float w) {
		throwVCMPError(funcs->SetVehicleRotation(vehicleId, x, y, z, w), "Failed to set vehicle rotation.");
	});

	m.def("set_vehicle_rotation_euler", [](int32_t vehicleId, float x, float y, float z) {
		throwVCMPError(funcs->SetVehicleRotationEuler(vehicleId, x, y, z), "Failed to set vehicle rotation Euler.");
	});

	m.def("get_vehicle_rotation", [](int32_t vehicleId) {
		float x, y, z, w;
		throwVCMPError(funcs->GetVehicleRotation(vehicleId, &x, &y, &z, &w), "Failed to get vehicle rotation.");
		return createQuaternion(x, y, z, w);
	});

	m.def("get_vehicle_rotation_euler", [](int32_t vehicleId) {
		float x, y, z;
		throwVCMPError(funcs->GetVehicleRotationEuler(vehicleId, &x, &y, &z), "Failed to get vehicle rotation Euler.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_speed", [](int32_t vehicleId, float x, float y, float z, bool add, bool relative) {
		throwVCMPError(funcs->SetVehicleSpeed(vehicleId, x, y, z, add, relative), "Failed to set vehicle speed.");
	});

	m.def("get_vehicle_speed", [](int32_t vehicleId, bool relative) {
		float x, y, z;
		throwVCMPError(funcs->GetVehicleSpeed(vehicleId, &x, &y, &z, relative), "Failed to get vehicle speed.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_turn_speed", [](int32_t vehicleId, float x, float y, float z, bool add, bool relative) {
		throwVCMPError(funcs->SetVehicleTurnSpeed(vehicleId, x, y, z, add, relative), "Failed to set vehicle turn speed.");
	});

	m.def("get_vehicle_turn_speed", [](int32_t vehicleId, bool relative) {
		float x, y, z;
		throwVCMPError(funcs->GetVehicleTurnSpeed(vehicleId, &x, &y, &z, relative), "Failed to get vehicle turn speed.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_spawn_position", [](int32_t vehicleId, float x, float y, float z) {
		throwVCMPError(funcs->SetVehicleSpawnPosition(vehicleId, x, y, z), "Failed to set vehicle spawn position.");
	});

	m.def("get_vehicle_spawn_position", [](int32_t vehicleId) {
		float x, y, z;
		throwVCMPError(funcs->GetVehicleSpawnPosition(vehicleId, &x, &y, &z), "Failed to get vehicle spawn position.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_spawn_rotation", [](int32_t vehicleId, float x, float y, float z, float w) {
		throwVCMPError(funcs->SetVehicleSpawnRotation(vehicleId, x, y, z, w), "Failed to set vehicle spawn rotation.");
	});

	m.def("set_vehicle_spawn_rotation_euler", [](int32_t vehicleId, float x, float y, float z) {
		throwVCMPError(funcs->SetVehicleSpawnRotationEuler(vehicleId, x, y, z), "Failed to set vehicle spawn rotation Euler.");
	});

	m.def("get_vehicle_spawn_rotation", [](int32_t vehicleId) {
		float x, y, z, w;
		throwVCMPError(funcs->GetVehicleSpawnRotation(vehicleId, &x, &y, &z, &w), "Failed to get vehicle spawn rotation.");
		return createQuaternion(x, y, z, w);
	});

	m.def("get_vehicle_spawn_rotation_euler", [](int32_t vehicleId) {
		float x, y, z;
		throwVCMPError(funcs->GetVehicleSpawnRotationEuler(vehicleId, &x, &y, &z), "Failed to get vehicle spawn rotation Euler.");
		return createVector(x, y, z);
	});

	m.def("set_vehicle_idle_respawn_timer", [](int32_t vehicleId, uint32_t millis) {
		throwVCMPError(funcs->SetVehicleIdleRespawnTimer(vehicleId, millis), "Failed to set vehicle idle respawn timer.");
	});

	m.def("get_vehicle_idle_respawn_timer", [](int32_t vehicleId) {
		return funcs->GetVehicleIdleRespawnTimer(vehicleId);
	});

	m.def("set_vehicle_health", [](int32_t vehicleId, float health) {
		throwVCMPError(funcs->SetVehicleHealth(vehicleId, health), "Failed to set vehicle health.");
	});

	m.def("get_vehicle_health", [](int32_t vehicleId) {
		return funcs->GetVehicleHealth(vehicleId);
	});

	m.def("set_vehicle_colour", [](int32_t vehicleId, int32_t primaryColour, int32_t secondaryColour) {
		throwVCMPError(funcs->SetVehicleColour(vehicleId, primaryColour, secondaryColour), "Failed to set vehicle colour.");
	});

	m.def("get_vehicle_colour", [](int32_t vehicleId) {
		int32_t primaryColourOut, secondaryColourOut;
		throwVCMPError(funcs->GetVehicleColour(vehicleId, &primaryColourOut, &secondaryColourOut), "Failed to get vehicle colour.");
		return pybind11::make_tuple(primaryColourOut, secondaryColourOut);
	});

	m.def("set_vehicle_part_status", [](int32_t vehicleId, int32_t partId, int32_t status) {
		throwVCMPError(funcs->SetVehiclePartStatus(vehicleId, partId, status), "Failed to set vehicle part status.");
	});

	m.def("get_vehicle_part_status", [](int32_t vehicleId, int32_t partId) {
		return funcs->GetVehiclePartStatus(vehicleId, partId);
	});

	m.def("set_vehicle_tyre_status", [](int32_t vehicleId, int32_t tyreId, int32_t status) {
		throwVCMPError(funcs->SetVehicleTyreStatus(vehicleId, tyreId, status), "Failed to set vehicle tyre status.");
	});

	m.def("get_vehicle_tyre_status", [](int32_t vehicleId, int32_t tyreId) {
		return funcs->GetVehicleTyreStatus(vehicleId, tyreId);
	});

	m.def("set_vehicle_damage_data", [](int32_t vehicleId, uint32_t damageData) {
		throwVCMPError(funcs->SetVehicleDamageData(vehicleId, damageData), "Failed to set vehicle damage data.");
	});

	m.def("get_vehicle_damage_data", [](int32_t vehicleId) {
		return funcs->GetVehicleDamageData(vehicleId);
	});

	m.def("set_vehicle_radio", [](int32_t vehicleId, int32_t radioId) {
		throwVCMPError(funcs->SetVehicleRadio(vehicleId, radioId), "Failed to set vehicle radio.");
	});

	m.def("get_vehicle_radio", [](int32_t vehicleId) {
		return funcs->GetVehicleRadio(vehicleId);
	});

	m.def("get_vehicle_turret_rotation", [](int32_t vehicleId) {
		float horizontalOut, verticalOut;
		throwVCMPError(funcs->GetVehicleTurretRotation(vehicleId, &horizontalOut, &verticalOut), "Failed to get vehicle turret rotation.");
		return pybind11::make_tuple(horizontalOut, verticalOut);
	});

	m.def("reset_all_vehicle_handlings", []() {
		funcs->ResetAllVehicleHandlings();
	});

	m.def("exists_handling_rule", [](int32_t modelIndex, int32_t ruleIndex) {
		return pybind11::bool_(funcs->ExistsHandlingRule(modelIndex, ruleIndex));
	});

	m.def("set_handling_rule", [](int32_t modelIndex, int32_t ruleIndex, double value) {
		throwVCMPError(funcs->SetHandlingRule(modelIndex, ruleIndex, value), "Failed to set handling rule.");
	});

	m.def("get_handling_rule", [](int32_t modelIndex, int32_t ruleIndex) {
		return funcs->GetHandlingRule(modelIndex, ruleIndex);
	});

	m.def("reset_handling_rule", [](int32_t modelIndex, int32_t ruleIndex) {
		throwVCMPError(funcs->ResetHandlingRule(modelIndex, ruleIndex), "Failed to reset handling rule.");
	});

	m.def("reset_handling", [](int32_t modelIndex) {
		throwVCMPError(funcs->ResetHandling(modelIndex), "Failed to reset handling.");
	});

	m.def("exists_inst_handling_rule", [](int32_t vehicleId, int32_t ruleIndex) {
		return pybind11::bool_(funcs->ExistsInstHandlingRule(vehicleId, ruleIndex));
	});

	m.def("set_inst_handling_rule", [](int32_t vehicleId, int32_t ruleIndex, double value) {
		throwVCMPError(funcs->SetInstHandlingRule(vehicleId, ruleIndex, value), "Failed to set inst handling rule.");
	});

	m.def("get_inst_handling_rule", [](int32_t vehicleId, int32_t ruleIndex) {
		return funcs->GetInstHandlingRule(vehicleId, ruleIndex);
	});

	m.def("reset_inst_handling_rule", [](int32_t vehicleId, int32_t ruleIndex) {
		throwVCMPError(funcs->ResetInstHandlingRule(vehicleId, ruleIndex), "Failed to reset inst handling rule.");
	});

	m.def("reset_inst_handling", [](int32_t vehicleId) {
		throwVCMPError(funcs->ResetInstHandling(vehicleId), "Failed to reset inst handling.");
	});

	m.def("create_pickup", [](int32_t modelIndex, int32_t world, int32_t quantity, float x, float y, float z, int32_t alpha, uint8_t isAutomatic) {
		return funcs->CreatePickup(modelIndex, world, quantity, x, y, z, alpha, isAutomatic);
	});

	m.def("delete_pickup", [](int32_t pickupId) {
		throwVCMPError(funcs->DeletePickup(pickupId), "Failed to delete pickup.");
	});

	m.def("is_pickup_streamed_for_player", [](int32_t pickupId, int32_t playerId) {
		return pybind11::bool_(funcs->IsPickupStreamedForPlayer(pickupId, playerId));
	});

	m.def("set_pickup_world", [](int32_t pickupId, int32_t world) {
		throwVCMPError(funcs->SetPickupWorld(pickupId, world), "Failed to set pickup world.");
	});

	m.def("get_pickup_world", [](int32_t pickupId) {
		return funcs->GetPickupWorld(pickupId);
	});

	m.def("set_pickup_alpha", [](int32_t pickupId, int32_t alpha) {
		throwVCMPError(funcs->SetPickupAlpha(pickupId, alpha), "Failed to set pickup alpha.");
	});

	m.def("get_pickup_alpha", [](int32_t pickupId) {
		return funcs->GetPickupAlpha(pickupId);
	});

	m.def("set_pickup_is_automatic", [](int32_t pickupId, bool toggle) {
		throwVCMPError(funcs->SetPickupIsAutomatic(pickupId, toggle), "Failed to set pickup automatic.");
	});

	m.def("is_pickup_automatic", [](int32_t pickupId) {
		return pybind11::bool_(funcs->IsPickupAutomatic(pickupId));
	});

	m.def("set_pickup_auto_timer", [](int32_t pickupId, uint32_t durationMillis) {
		throwVCMPError(funcs->SetPickupAutoTimer(pickupId, durationMillis), "Failed to set pickup auto timer.");
	});

	m.def("get_pickup_auto_timer", [](int32_t pickupId) {
		return funcs->GetPickupAutoTimer(pickupId);
	});

	m.def("refresh_pickup", [](int32_t pickupId) {
		throwVCMPError(funcs->RefreshPickup(pickupId), "Failed to refresh pickup.");
	});

	m.def("set_pickup_position", [](int32_t pickupId, float x, float y, float z) {
		throwVCMPError(funcs->SetPickupPosition(pickupId, x, y, z), "Failed to set pickup position.");
	});

	m.def("get_pickup_position", [](int32_t pickupId) {
		float x, y, z;
		throwVCMPError(funcs->GetPickupPosition(pickupId, &x, &y, &z), "Failed to get pickup position.");
		return createVector(x, y, z);
	});

	m.def("get_pickup_model", [](int32_t pickupId) {
		return funcs->GetPickupModel(pickupId);
	});

	m.def("get_pickup_quantity", [](int32_t pickupId) {
		return funcs->GetPickupQuantity(pickupId);
	});

	m.def("create_check_point", [](int32_t playerId, int32_t world, uint8_t isSphere, float x, float y, float z, int32_t red, int32_t green, int32_t blue, int32_t alpha, float radius) {
		return funcs->CreateCheckPoint(playerId, world, isSphere, x, y, z, red, green, blue, alpha, radius);
	});

	m.def("delete_check_point", [](int32_t checkPointId) {
		throwVCMPError(funcs->DeleteCheckPoint(checkPointId), "Failed to delete check point.");
	});

	m.def("is_check_point_streamed_for_player", [](int32_t checkPointId, int32_t playerId) {
		return pybind11::bool_(funcs->IsCheckPointStreamedForPlayer(checkPointId, playerId));
	});

	m.def("is_check_point_sphere", [](int32_t checkPointId) {
		return pybind11::bool_(funcs->IsCheckPointSphere(checkPointId));
	});

	m.def("set_check_point_world", [](int32_t checkPointId, int32_t world) {
		throwVCMPError(funcs->SetCheckPointWorld(checkPointId, world), "Failed to set check point world.");
	});

	m.def("get_check_point_world", [](int32_t checkPointId) {
		return funcs->GetCheckPointWorld(checkPointId);
	});

	m.def("set_check_point_colour", [](int32_t checkPointId, int32_t red, int32_t green, int32_t blue, int32_t alpha) {
		throwVCMPError(funcs->SetCheckPointColour(checkPointId, red, green, blue, alpha), "Failed to set check point colour.");
	});

	m.def("get_check_point_colour", [](int32_t checkPointId) {
		int32_t red, green, blue, alpha;
		throwVCMPError(funcs->GetCheckPointColour(checkPointId, &red, &green, &blue, &alpha), "Failed to get check point colour.");
		pybind11::dict ret = pybind11::dict();
		ret["red"] = red;
		ret["green"] = green;
		ret["blue"] = blue;
		ret["alpha"] = alpha;
		return ret;
	});

	m.def("set_check_point_position", [](int32_t checkPointId, float x, float y, float z) {
		throwVCMPError(funcs->SetCheckPointPosition(checkPointId, x, y, z), "Failed to set check point position.");
	});

	m.def("get_check_point_position", [](int32_t checkPointId) {
		float x, y, z;
		throwVCMPError(funcs->GetCheckPointPosition(checkPointId, &x, &y, &z), "Failed to get check point position.");
		return createVector(x, y, z);
	});

	m.def("set_check_point_radius", [](int32_t checkPointId, float radius) {
		throwVCMPError(funcs->SetCheckPointRadius(checkPointId, radius), "Failed to set check point radius.");
	});

	m.def("get_check_point_radius", [](int32_t checkPointId) {
		return funcs->GetCheckPointRadius(checkPointId);
	});

	m.def("get_check_point_owner", [](int32_t checkPointId) {
		return funcs->GetCheckPointOwner(checkPointId);
	});

	m.def("create_object", [](int32_t modelIndex, int32_t world, float x, float y, float z, int32_t alpha) {
		return funcs->CreateObject(modelIndex, world, x, y, z, alpha);
	});

	m.def("delete_object", [](int32_t objectId) {
		throwVCMPError(funcs->DeleteObject(objectId), "Failed to delete object.");
	});

	m.def("is_object_streamed_for_player", [](int32_t objectId, int32_t playerId) {
		return pybind11::bool_(funcs->IsObjectStreamedForPlayer(objectId, playerId));
	});

	m.def("get_object_model", [](int32_t objectId) {
		return funcs->GetObjectModel(objectId);
	});

	m.def("set_object_world", [](int32_t objectId, int32_t world) {
		throwVCMPError(funcs->SetObjectWorld(objectId, world), "Failed to set object world.");
	});

	m.def("get_object_world", [](int32_t objectId) {
		return funcs->GetObjectWorld(objectId);
	});

	m.def("set_object_alpha", [](int32_t objectId, int32_t alpha, uint32_t duration) {
		throwVCMPError(funcs->SetObjectAlpha(objectId, alpha, duration), "Failed to set object alpha.");
	});

	m.def("get_object_alpha", [](int32_t objectId) {
		return funcs->GetObjectAlpha(objectId);
	});

	m.def("move_object_to", [](int32_t objectId, float x, float y, float z, uint32_t duration) {
		throwVCMPError(funcs->MoveObjectTo(objectId, x, y, z, duration), "Failed to move object to.");
	});

	m.def("move_object_by", [](int32_t objectId, float x, float y, float z, uint32_t duration) {
		throwVCMPError(funcs->MoveObjectBy(objectId, x, y, z, duration), "Failed to move object by.");
	});

	m.def("set_object_position", [](int32_t objectId, float x, float y, float z) {
		throwVCMPError(funcs->SetObjectPosition(objectId, x, y, z), "Failed to set object position.");
	});

	m.def("get_object_position", [](int32_t objectId) {
		float x, y, z;
		throwVCMPError(funcs->GetObjectPosition(objectId, &x, &y, &z), "Failed to get object position.");
		return createVector(x, y, z);
	});

	m.def("rotate_object_to", [](int32_t objectId, float x, float y, float z, float w, uint32_t duration) {
		throwVCMPError(funcs->RotateObjectTo(objectId, x, y, z, w, duration), "Failed to rotate object to.");
	});

	m.def("rotate_object_to_euler", [](int32_t objectId, float x, float y, float z, uint32_t duration) {
		throwVCMPError(funcs->RotateObjectToEuler(objectId, x, y, z, duration), "Failed to rotate object to Euler.");
	});

	m.def("rotate_object_by", [](int32_t objectId, float x, float y, float z, float w, uint32_t duration) {
		throwVCMPError(funcs->RotateObjectBy(objectId, x, y, z, w, duration), "Failed to rotate object by.");
	});

	m.def("rotate_object_by_euler", [](int32_t objectId, float x, float y, float z, uint32_t duration) {
		throwVCMPError(funcs->RotateObjectByEuler(objectId, x, y, z, duration), "Failed to rotate object by Euler.");
	});

	m.def("get_object_rotation", [](int32_t objectId) {
		float x, y, z, w;
		throwVCMPError(funcs->GetObjectRotation(objectId, &x, &y, &z, &w), "Failed to get object rotation.");
		return createQuaternion(x, y, z, w);
	});

	m.def("get_object_rotation_euler", [](int32_t objectId) {
		float x, y, z;
		throwVCMPError(funcs->GetObjectRotationEuler(objectId, &x, &y, &z), "Failed to get object rotation Euler.");
		return createVector(x, y, z);
	});

	m.def("set_object_shot_report_enabled", [](int32_t objectId, bool toggle) {
		throwVCMPError(funcs->SetObjectShotReportEnabled(objectId, toggle), "Failed to set object shot report enabled.");
	});

	m.def("is_object_shot_report_enabled", [](int32_t objectId) {
		return pybind11::bool_(funcs->IsObjectShotReportEnabled(objectId));
	});

	m.def("set_object_touched_report_enabled", [](int32_t objectId, bool toggle) {
		throwVCMPError(funcs->SetObjectTouchedReportEnabled(objectId, toggle), "Failed to set object touched report enabled.");
	});

	m.def("is_object_touched_report_enabled", [](int32_t objectId) {
		return pybind11::bool_(funcs->IsObjectTouchedReportEnabled(objectId));
	});

	m.def("get_player_module_list", [](int32_t playerId) {
		return funcs->GetPlayerModuleList(playerId);
	});

	m.def("set_pickup_option", [](int32_t pickupId, int option, bool toggle) {
		throwVCMPError(funcs->SetPickupOption(pickupId, static_cast<vcmpPickupOption>(option), toggle), "Failed to set pickup option.");
	});

	m.def("get_pickup_option", [](int32_t pickupId, int option) {
		return funcs->GetPickupOption(pickupId, static_cast<vcmpPickupOption>(option));
	});

	m.def("set_fall_timer", [](uint16_t timeRate) {
		funcs->SetFallTimer(timeRate);
	});

	m.def("get_fall_timer", []() {
		return funcs->GetFallTimer();
	});

	m.def("set_vehicle_lights_data", [](int32_t vehicleId, uint32_t lightsData) {
		throwVCMPError(funcs->SetVehicleLightsData(vehicleId, lightsData), "Failed to set vehicle lights data.");
	});

	m.def("get_vehicle_lights_data", [](int32_t vehicleId) {
		return funcs->GetVehicleLightsData(vehicleId);
	});

	/* Maybe 0.4.7.0 or high version */
	m.def("kill_player", [](int32_t player) {
		throwVCMPError(funcs->KillPlayer(player), "Failed to kill player.");
	});

	m.def("set_vehicle_3d_arrow_for_player", [](int32_t vehicleId, int32_t targetPlayerId, bool toggle) {
		throwVCMPError(funcs->SetVehicle3DArrowForPlayer(vehicleId, targetPlayerId, toggle), "Failed to set vehicle 3D arrow for player.");
	});

	m.def("get_vehicle_3d_arrow_for_player", [](int32_t vehicleId, int32_t targetPlayerId) {
		return py::bool_(funcs->GetVehicle3DArrowForPlayer(vehicleId, targetPlayerId));
	});

	m.def("set_player_3d_arrow_for_player", [](int32_t playerId, int32_t targetPlayerId, bool toggle) {
		throwVCMPError(funcs->SetPlayer3DArrowForPlayer(playerId, targetPlayerId, toggle), "Failed to set player 3D arrow for player.");
	});

	m.def("get_player_3d_arrow_for_player", [](int32_t playerId, int32_t targetPlayerId) {
		return py::bool_(funcs->GetPlayer3DArrowForPlayer(playerId, targetPlayerId));
	});

	m.def("set_player_drunk_handling", [](int32_t playerId, uint32_t drunkLevel) {
		throwVCMPError(funcs->SetPlayerDrunkHandling(playerId, drunkLevel), "Failed to set player drunk handling.");
	});

	m.def("get_player_drunk_handling", [](int32_t playerId) {
		return funcs->GetPlayerDrunkHandling(playerId);
	});

	m.def("set_player_drunk_visuals", [](int32_t playerId, uint8_t drunkLevel) {
		throwVCMPError(funcs->SetPlayerDrunkVisuals(playerId, drunkLevel), "Failed to set player drunk visuals.");
	});

	m.def("get_player_drunk_visuals", [](int32_t playerId) {
		return funcs->GetPlayerDrunkVisuals(playerId);
	});

	m.def("interpolate_camera_look_at", [](int32_t playerId, float lookX, float lookY, float lookZ, uint32_t interpTimeMS) {
		throwVCMPError(funcs->InterpolateCameraLookAt(playerId, lookX, lookY, lookZ, interpTimeMS), "Failed to interpolate camera look at.");
	});
    logger.debug("Bound VCMP functions to Python module.");
}

void bindVCMPCallbacks() {
	vcalls->OnServerInitialise = []() -> uint8_t
	{
		logger.info("Loaded " + std::string(PLUGIN_NAME) + " version " + std::string(PLUGIN_VERSION) + " by " + std::string(AUTHOR) + ". (" + std::string(LICENSE) + " LICENSE)");
		handlePythonFunction("server_initialise");
        return 1;
	};
	vcalls->OnServerShutdown = []() -> void
    {
        handlePythonFunction("server_shutdown", py::none(), [](py::object func) {
            return func();
        });
    };
    vcalls->OnServerFrame = [](float elapsedTime) -> void
    {
        handlePythonFunction("server_frame", py::none(), [&elapsedTime](py::object func) {
            return func(elapsedTime);
        });
        Py_BEGIN_ALLOW_THREADS Py_END_ALLOW_THREADS;
		if (PyErr_CheckSignals() == -1) {
			vfuncs->ShutdownServer();
		}
    };
    
    vcalls->OnPluginCommand = [](uint32_t commandIdentifier, const char* message) -> uint8_t
    {
        handlePythonFunction("plugin_command", py::none(), [&commandIdentifier, &message](py::object func) {
            return func(commandIdentifier, message);
        });
        return 1;
    };

    vcalls->OnIncomingConnection = [](char* playerName, size_t nameBufferSize, const char* userPassword, const char* ipAddress) -> uint8_t {
        try {
            auto ret = handlePythonFunction("incoming_connection", DEFAULT_RETURN, [&playerName, &nameBufferSize, &userPassword, &ipAddress](py::object func) {
                return func(playerName, nameBufferSize, userPassword, ipAddress);
            }).cast<uint8_t>();
        } catch (...) {
            raiseException("Failed to call Python function incoming_connection");
        }
        return 1;
    };
    vcalls->OnClientScriptData = [](int32_t playerId, const uint8_t* data, size_t size) -> void {
		std::vector<uint8_t> vec(data, data + size);
        py::bytes buffer = py::bytes(reinterpret_cast<const char *>(vec.data()), vec.size());

        handlePythonFunction("client_script_data", py::none(), [&playerId, &buffer, &size](py::object func) {
			return func(playerId, buffer, size);
		});
    };

    vcalls->OnPlayerConnect = [](int32_t playerId) -> void {
        // pre
        handlePythonFunction("pre_player_connect", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });

        handlePythonFunction("player_connect", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });
    };

    vcalls->OnPlayerDisconnect = [](int32_t playerId, vcmpDisconnectReason reason) -> void {
        handlePythonFunction("player_disconnect", py::none(), [&playerId, &reason](py::object func) {
            return func(playerId, (int)reason);
        });

        // post call into module and destroy player object
        handlePythonFunction("post_player_disconnect", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });
    };

    vcalls->OnPlayerRequestClass = [](int32_t playerId, int32_t offset) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_request_class", DEFAULT_RETURN, [&playerId, &offset](py::object func) {
                return func(playerId, offset);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_request_class");
        }
        return 1;
    };
    vcalls->OnPlayerRequestSpawn = [](int32_t playerId) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_request_spawn", DEFAULT_RETURN, [&playerId](py::object func) {
                return func(playerId);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_request_spawn");
        }
        return 1;
    };
    vcalls->OnPlayerSpawn = [](int32_t playerId) -> void {
        handlePythonFunction("player_spawn", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });
    };
    vcalls->OnPlayerDeath = [](int32_t playerId, int32_t killerId, int32_t reason, vcmpBodyPart bodyPart) -> void {
        handlePythonFunction("player_death", py::none(), [&playerId, &killerId, &reason, &bodyPart](py::object func) {
            return func(playerId, killerId, reason, (int)bodyPart);
        });
    };
    vcalls->OnPlayerUpdate = [](int32_t playerId, vcmpPlayerUpdate updateType) -> void {
        handlePythonFunction("player_update", py::none(), [&playerId, &updateType](py::object func) {
            return func(playerId, (int)updateType);
        });
    };
    vcalls->OnPlayerRequestEnterVehicle = [](int32_t playerId, int32_t vehicleId, int32_t slotIndex) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_request_enter_vehicle", DEFAULT_RETURN, [&playerId, &vehicleId, &slotIndex](py::object func) {
                return func(playerId, vehicleId, slotIndex);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_request_enter_vehicle");
        }
        return 1;
    };

    vcalls->OnPlayerEnterVehicle = [](int32_t playerId, int32_t vehicleId, int32_t slotIndex) -> void {
        handlePythonFunction("player_enter_vehicle", py::none(), [&playerId, &vehicleId, &slotIndex](py::object func) {
            return func(playerId, vehicleId, slotIndex);
        });
    };

    vcalls->OnPlayerExitVehicle = [](int32_t playerId, int32_t vehicleId) -> void {
        handlePythonFunction("player_exit_vehicle", py::none(), [&playerId, &vehicleId](py::object func) {
            return func(playerId, vehicleId);
        });
    };

    vcalls->OnPlayerNameChange = [](int32_t playerId, const char* oldName, const char* newName) -> void {
        handlePythonFunction("player_name_change", py::none(), [&playerId, &oldName, &newName](py::object func) {
            return func(playerId, oldName, newName);
        });
    };

    vcalls->OnPlayerStateChange = [](int32_t playerId, vcmpPlayerState oldState, vcmpPlayerState newState) -> void {
        handlePythonFunction("player_state_change", py::none(), [&playerId, &oldState, &newState](py::object func) {
            return func(playerId, (int)oldState, (int)newState);
        });
    };

    vcalls->OnPlayerActionChange = [](int32_t playerId, int32_t oldAction, int32_t newAction) -> void {
        handlePythonFunction("player_action_change", py::none(), [&playerId, &oldAction, &newAction](py::object func) {
            return func(playerId, oldAction, newAction);
        });
    };

    vcalls->OnPlayerOnFireChange = [](int32_t playerId, uint8_t isOnFire) -> void {
        handlePythonFunction("player_on_fire_change", py::none(), [&playerId, &isOnFire](py::object func) {
            return func(playerId, isOnFire);
        });
    };

    vcalls->OnPlayerCrouchChange = [](int32_t playerId, uint8_t isCrouching) -> void {
        handlePythonFunction("player_crouch_change", py::none(), [&playerId, &isCrouching](py::object func) {
            return func(playerId, isCrouching);
        });
    };

    vcalls->OnPlayerGameKeysChange = [](int32_t playerId, uint32_t oldKeys, uint32_t newKeys) -> void {
        handlePythonFunction("player_game_keys_change", py::none(), [&playerId, &oldKeys, &newKeys](py::object func) {
            return func(playerId, oldKeys, newKeys);
        });
    };

    vcalls->OnPlayerBeginTyping = [](int32_t playerId) -> void {
        handlePythonFunction("player_begin_typing", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });
    };

    vcalls->OnPlayerEndTyping = [](int32_t playerId) -> void {
        handlePythonFunction("player_end_typing", py::none(), [&playerId](py::object func) {
            return func(playerId);
        });
    };

    vcalls->OnPlayerAwayChange = [](int32_t playerId, uint8_t isAway) -> void {
        handlePythonFunction("player_away_change", py::none(), [&playerId, &isAway](py::object func) {
            return func(playerId, isAway);
        });
    };

    vcalls->OnPlayerMessage = [](int32_t playerId, const char* message) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_message", DEFAULT_RETURN, [&playerId, &message](py::object func) {
                return func(playerId, message);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_message");
        }
        return 1;
    };
    vcalls->OnPlayerCommand = [](int32_t playerId, const char* message) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_command", DEFAULT_RETURN, [&playerId, &message](py::object func) {
                return func(playerId, message);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_command");
        }
        return 1;
    };

    vcalls->OnPlayerPrivateMessage = [](int32_t playerId, int32_t targetPlayerId, const char* message) -> uint8_t {
        try {
            auto ret = handlePythonFunction("player_private_message", DEFAULT_RETURN, [&playerId, &targetPlayerId, &message](py::object func) {
                return func(playerId, targetPlayerId, message);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function player_private_message");
        }
        return 1;
    };
    vcalls->OnPlayerKeyBindDown = [](int32_t playerId, int32_t bindId) -> void {
        handlePythonFunction("player_key_bind_down", py::none(), [&playerId, &bindId](py::object func) {
            return func(playerId, bindId);
        });
    };
    vcalls->OnPlayerKeyBindUp = [](int32_t playerId, int32_t bindId) -> void {
        handlePythonFunction("player_key_bind_up", py::none(), [&playerId, &bindId](py::object func) {
            return func(playerId, bindId);
        });
    };
    vcalls->OnPlayerSpectate = [](int32_t playerId, int32_t targetPlayerId) -> void {
        handlePythonFunction("player_spectate", py::none(), [&playerId, &targetPlayerId](py::object func) {
            return func(playerId, targetPlayerId);
        });
    };
    vcalls->OnPlayerCrashReport = [](int32_t playerId, const char* report) -> void {
        handlePythonFunction("player_crash_report", py::none(), [&playerId, &report](py::object func) {
            return func(playerId, report);
        });
    };
    vcalls->OnVehicleUpdate = [](int32_t vehicleId, vcmpVehicleUpdate updateType) -> void {
        handlePythonFunction("vehicle_update", py::none(), [&vehicleId, &updateType](py::object func) {
            return func(vehicleId, (int)updateType);
        });
    };
    vcalls->OnVehicleExplode = [](int32_t vehicleId) -> void {
        handlePythonFunction("vehicle_explode", py::none(), [&vehicleId](py::object func) {
            return func(vehicleId);
        });
    };
    vcalls->OnVehicleRespawn = [](int32_t vehicleId) -> void {
        handlePythonFunction("vehicle_respawn", py::none(), [&vehicleId](py::object func) {
            return func(vehicleId);
        });
    };
    vcalls->OnObjectShot = [](int32_t objectId, int32_t playerId, int32_t weaponId) -> void {
        handlePythonFunction("object_shot", py::none(), [&objectId, &playerId, &weaponId](py::object func) {
            return func(objectId, playerId, weaponId);
        });
    };
    vcalls->OnObjectTouched = [](int32_t objectId, int32_t playerId) -> void {
        handlePythonFunction("object_touched", py::none(), [&objectId, &playerId](py::object func) {
            return func(objectId, playerId);
        });
    };
    vcalls->OnPickupPickAttempt = [](int32_t pickupId, int32_t playerId) -> uint8_t {
        try {
            auto ret = handlePythonFunction("pickup_pick_attempt", DEFAULT_RETURN, [&pickupId, &playerId](py::object func) {
                return func(pickupId, playerId);
            }).cast<uint8_t>();
            return ret;
        } catch (...) {
            raiseException("Failed to call Python function pickup_pick_attempt");
        }
        return 1;
    };
    vcalls->OnPickupPicked = [](int32_t pickupId, int32_t playerId) -> void {
        handlePythonFunction("pickup_picked", py::none(), [&pickupId, &playerId](py::object func) {
            return func(pickupId, playerId);
        });
    };
    vcalls->OnPickupRespawn = [](int32_t pickupId) -> void {
        handlePythonFunction("pickup_respawn", py::none(), [&pickupId](py::object func) {
            return func(pickupId);
        });
    };
    vcalls->OnCheckpointEntered = [](int32_t checkPointId, int32_t playerId) -> void {
        handlePythonFunction("checkpoint_entered", py::none(), [&checkPointId, &playerId](py::object func) {
            return func(checkPointId, playerId);
        });
    };
    vcalls->OnCheckpointExited = [](int32_t checkPointId, int32_t playerId) -> void {
        handlePythonFunction("checkpoint_exited", py::none(), [&checkPointId, &playerId](py::object func) {
            return func(checkPointId, playerId);
        });
    };
    vcalls->OnEntityPoolChange = [](vcmpEntityPool entityType, int32_t entityId, uint8_t isDeleted) -> void {
        handlePythonFunction("entity_pool_change", py::none(), [&entityType, &entityId, &isDeleted](py::object func) {
            return func((int)entityType, entityId, isDeleted);
        });
    };
    vcalls->OnServerPerformanceReport = [](size_t entryCount, const char** descriptions, uint64_t* times) -> void {
        // Convert the descriptions to a list of strings
        py::list pyDescriptions = py::list();
        py::list pyTimes = py::list();
        for (size_t i = 0; i < entryCount; ++i) {
            pyDescriptions.append(std::string(descriptions[i]));
            pyTimes.append(times[i]);
        }
        handlePythonFunction("server_performance_report", py::none(), [&entryCount, &pyDescriptions, &pyTimes](py::object func) {
            return func(entryCount, pyDescriptions, pyTimes);
        });
    };

    vcalls->OnPlayerModuleList = [](int32_t playerId, const char* list) -> void {
        handlePythonFunction("player_module_list", py::none(), [&playerId, &list](py::object func) {
            return func(playerId, list);
        });
    };
}

PYBIND11_EMBEDDED_MODULE(__vcmp, m) {
	pfunctions = py::module("functions");
	pcallbacks = py::module("callbacks");
    // setattr
    m.attr("functions") = pfunctions;
    m.attr("callbacks") = pcallbacks;

    logger.debug("module registered");

    // last call
    bindVCMPFunctions();
	//bindVCMPCallbacks();
}

void initVCMP(PluginFuncs* vcmpFuncs, PluginCallbacks* vcmpCallbacks) {
    vfuncs = vcmpFuncs;
    vcalls = vcmpCallbacks;
    //bindVCMPFunctions();
    bindVCMPCallbacks();

}

#endif