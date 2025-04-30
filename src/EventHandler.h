#pragma once

uint8_t OnServerInitialise()
{
	uint8_t retval = 1;

	std::string moduleName;
	bool found = ReadServerConfig("server.cfg", moduleName);
	found |= ReadServerConfig(CONFIG_FILENAME_ARCH, moduleName);

	try
	{
		if (found)
			py::module::import(moduleName.c_str());

		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_server_initialise");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func();
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
				else
					retval = 1;
			}
		}
	}
	catch (...)
	{
		retval = 0;
		PythonExceptionHandler();
	}

	return retval;
}

void OnServerShutdown()
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_server_shutdown");
			if (py::isinstance<py::function>(func))
				func();
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}

	if (moduleCallbacks)
		delete moduleCallbacks;
	moduleCallbacks = nullptr;

	py::finalize_interpreter();
}

void OnServerFrame(float elapsedTime)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_server_frame");
			if (py::isinstance<py::function>(func))
				func(elapsedTime);
		}

		// If there's no on_server_frame handler, python has no chances to process other threads and Ctrl-C events.
		// This allows python to process them.
		Py_BEGIN_ALLOW_THREADS Py_END_ALLOW_THREADS

		if (PyErr_CheckSignals() == -1)
			throw py::error_already_set();
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

uint8_t OnPluginCommand(uint32_t commandIdentifier, const char* message)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_plugin_command");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(commandIdentifier, message);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

uint8_t OnIncomingConnection(char* playerName, size_t nameBufferSize, const char* userPassword, const char* ipAddress)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_incoming_connection");
			if (py::isinstance<py::function>(func))
			{
				--nameBufferSize;
				py::object resobj = func(playerName, nameBufferSize, userPassword, ipAddress);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
				else if (py::isinstance<py::str>(resobj)) // Change player name.
				{
					size_t len = resobj.cast<std::string>().copy(playerName, nameBufferSize);
					playerName[len] = 0;
				}
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

void OnClientScriptData(int32_t playerId, const uint8_t* data, size_t size)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_client_script_data");
			if (py::isinstance<py::function>(func))
				func(playerId, py::bytes(reinterpret_cast<const char*>(data), size));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerConnect(int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_connect");
			if (py::isinstance<py::function>(func))
				func(playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerDisconnect(int32_t playerId, vcmpDisconnectReason reason)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_disconnect");
			if (py::isinstance<py::function>(func))
				func(playerId, static_cast<int32_t>(reason));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

uint8_t OnPlayerRequestClass(int32_t playerId, int32_t offset)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_request_class");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId, offset);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

uint8_t OnPlayerRequestSpawn(int32_t playerId)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_request_spawn");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

void OnPlayerSpawn(int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_spawn");
			if (py::isinstance<py::function>(func))
				func(playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerDeath(int32_t playerId, int32_t killerId, int32_t reason, vcmpBodyPart bodyPart)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_death");
			if (py::isinstance<py::function>(func))
				func(playerId, killerId, reason, static_cast<int32_t>(bodyPart));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerUpdate(int32_t playerId, vcmpPlayerUpdate updateType)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_update");
			if (py::isinstance<py::function>(func))
				func(playerId, static_cast<int32_t>(updateType));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

uint8_t OnPlayerRequestEnterVehicle(int32_t playerId, int32_t vehicleId, int32_t slotIndex)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_request_enter_vehicle");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId, vehicleId, slotIndex);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

void OnPlayerEnterVehicle(int32_t playerId, int32_t vehicleId, int32_t slotIndex)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_enter_vehicle");
			if (py::isinstance<py::function>(func))
				func(playerId, vehicleId, slotIndex);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerExitVehicle(int32_t playerId, int32_t vehicleId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_exit_vehicle");
			if (py::isinstance<py::function>(func))
				func(playerId, vehicleId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerNameChange(int32_t playerId, const char* oldName, const char* newName)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_name_change");
			if (py::isinstance<py::function>(func))
				func(playerId, oldName, newName);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerStateChange(int32_t playerId, vcmpPlayerState oldState, vcmpPlayerState newState)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_state_change");
			if (py::isinstance<py::function>(func))
				func(playerId, static_cast<int32_t>(oldState), static_cast<int32_t>(newState));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerActionChange(int32_t playerId, int32_t oldAction, int32_t newAction)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_action_change");
			if (py::isinstance<py::function>(func))
				func(playerId, oldAction, newAction);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerOnFireChange(int32_t playerId, uint8_t isOnFire)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_on_fire_change");
			if (py::isinstance<py::function>(func))
				func(playerId, py::bool_(isOnFire != 0));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerCrouchChange(int32_t playerId, uint8_t isCrouching)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_crouch_change");
			if (py::isinstance<py::function>(func))
				func(playerId, py::bool_(isCrouching != 0));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerGameKeysChange(int32_t playerId, uint32_t oldKeys, uint32_t newKeys)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_game_keys_change");
			if (py::isinstance<py::function>(func))
				func(playerId, oldKeys, newKeys);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerBeginTyping(int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_begin_typing");
			if (py::isinstance<py::function>(func))
				func(playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerEndTyping(int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_end_typing");
			if (py::isinstance<py::function>(func))
				func(playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerAwayChange(int32_t playerId, uint8_t isAway)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_away_change");
			if (py::isinstance<py::function>(func))
				func(playerId, py::bool_(isAway != 0));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

uint8_t OnPlayerMessage(int32_t playerId, const char* message)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_message");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId, message);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

uint8_t OnPlayerCommand(int32_t playerId, const char* message)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_command");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId, message);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

uint8_t OnPlayerPrivateMessage(int32_t playerId, int32_t targetPlayerId, const char* message)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_private_message");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(playerId, targetPlayerId, message);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

void OnPlayerKeyBindDown(int32_t playerId, int32_t bindId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_key_bind_down");
			if (py::isinstance<py::function>(func))
				func(playerId, bindId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerKeyBindUp(int32_t playerId, int32_t bindId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_key_bind_up");
			if (py::isinstance<py::function>(func))
				func(playerId, bindId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerSpectate(int32_t playerId, int32_t targetPlayerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_spectate");
			if (py::isinstance<py::function>(func))
				func(playerId, targetPlayerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerCrashReport(int32_t playerId, const char* report)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_crash_report");
			if (py::isinstance<py::function>(func))
				func(playerId, report);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnVehicleUpdate(int32_t vehicleId, vcmpVehicleUpdate updateType)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_vehicle_update");
			if (py::isinstance<py::function>(func))
				func(vehicleId, static_cast<int32_t>(updateType));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnVehicleExplode(int32_t vehicleId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_vehicle_explode");
			if (py::isinstance<py::function>(func))
				func(vehicleId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnVehicleRespawn(int32_t vehicleId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_vehicle_respawn");
			if (py::isinstance<py::function>(func))
				func(vehicleId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnObjectShot(int32_t objectId, int32_t playerId, int32_t weaponId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_object_shot");
			if (py::isinstance<py::function>(func))
				func(objectId, playerId, weaponId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnObjectTouched(int32_t objectId, int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_object_touched");
			if (py::isinstance<py::function>(func))
				func(objectId, playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

uint8_t OnPickupPickAttempt(int32_t pickupId, int32_t playerId)
{
	uint8_t retval = 1;
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_pickup_pick_attempt");
			if (py::isinstance<py::function>(func))
			{
				py::object resobj = func(pickupId, playerId);
				if (py::isinstance<py::bool_>(resobj))
					retval = resobj.cast<uint8_t>();
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
	return retval;
}

void OnPickupPicked(int32_t pickupId, int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_pickup_picked");
			if (py::isinstance<py::function>(func))
				func(pickupId, playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPickupRespawn(int32_t pickupId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_pickup_respawn");
			if (py::isinstance<py::function>(func))
				func(pickupId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnCheckpointEntered(int32_t checkPointId, int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_checkpoint_entered");
			if (py::isinstance<py::function>(func))
				func(checkPointId, playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnCheckpointExited(int32_t checkPointId, int32_t playerId)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_checkpoint_exited");
			if (py::isinstance<py::function>(func))
				func(checkPointId, playerId);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnEntityPoolChange(vcmpEntityPool entityType, int32_t entityId, uint8_t isDeleted)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_entity_pool_change");
			if (py::isinstance<py::function>(func))
				func(static_cast<int32_t>(entityType), entityId, py::bool_(isDeleted != 0));
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnServerPerformanceReport(size_t entryCount, const char** descriptions, uint64_t* times)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_server_performance_report");
			if (py::isinstance<py::function>(func))
			{
				py::dict dict;
				for (size_t i = 0; i < entryCount; i++) {
					dict[descriptions[i]] = times[i];
				}
				func(dict);
			}
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void OnPlayerModuleList(int32_t playerId, const char* list)
{
	try
	{
		if (moduleCallbacks)
		{
			auto func = moduleCallbacks->attr("on_player_module_list");
			if (py::isinstance<py::function>(func))
				func(playerId, list);
		}
	}
	catch (...)
	{
		PythonExceptionHandler();
	}
}

void RegisterCallbacks(PluginCallbacks* callbacks)
{
	callbacks->OnServerInitialise = OnServerInitialise;
	callbacks->OnServerShutdown = OnServerShutdown;
	callbacks->OnServerFrame = OnServerFrame;

	callbacks->OnPluginCommand = OnPluginCommand;
	callbacks->OnIncomingConnection = OnIncomingConnection;
	callbacks->OnClientScriptData = OnClientScriptData;

	callbacks->OnPlayerConnect = OnPlayerConnect;
	callbacks->OnPlayerDisconnect = OnPlayerDisconnect;

	callbacks->OnPlayerRequestClass = OnPlayerRequestClass;
	callbacks->OnPlayerRequestSpawn = OnPlayerRequestSpawn;
	callbacks->OnPlayerSpawn = OnPlayerSpawn;
	callbacks->OnPlayerDeath = OnPlayerDeath;
	callbacks->OnPlayerUpdate = OnPlayerUpdate;

	callbacks->OnPlayerRequestEnterVehicle = OnPlayerRequestEnterVehicle;
	callbacks->OnPlayerEnterVehicle = OnPlayerEnterVehicle;
	callbacks->OnPlayerExitVehicle = OnPlayerExitVehicle;

	callbacks->OnPlayerNameChange = OnPlayerNameChange;
	callbacks->OnPlayerStateChange = OnPlayerStateChange;
	callbacks->OnPlayerActionChange = OnPlayerActionChange;
	callbacks->OnPlayerOnFireChange = OnPlayerOnFireChange;
	callbacks->OnPlayerCrouchChange = OnPlayerCrouchChange;
	callbacks->OnPlayerGameKeysChange = OnPlayerGameKeysChange;
	callbacks->OnPlayerBeginTyping = OnPlayerBeginTyping;
	callbacks->OnPlayerEndTyping = OnPlayerEndTyping;
	callbacks->OnPlayerAwayChange = OnPlayerAwayChange;

	callbacks->OnPlayerMessage = OnPlayerMessage;
	callbacks->OnPlayerCommand = OnPlayerCommand;
	callbacks->OnPlayerPrivateMessage = OnPlayerPrivateMessage;

	callbacks->OnPlayerKeyBindDown = OnPlayerKeyBindDown;
	callbacks->OnPlayerKeyBindUp = OnPlayerKeyBindUp;
	callbacks->OnPlayerSpectate = OnPlayerSpectate;
	callbacks->OnPlayerCrashReport = OnPlayerCrashReport;

	callbacks->OnVehicleUpdate = OnVehicleUpdate;
	callbacks->OnVehicleExplode = OnVehicleExplode;
	callbacks->OnVehicleRespawn = OnVehicleRespawn;

	callbacks->OnObjectShot = OnObjectShot;
	callbacks->OnObjectTouched = OnObjectTouched;

	callbacks->OnPickupPickAttempt = OnPickupPickAttempt;
	callbacks->OnPickupPicked = OnPickupPicked;
	callbacks->OnPickupRespawn = OnPickupRespawn;

	callbacks->OnCheckpointEntered = OnCheckpointEntered;
	callbacks->OnCheckpointExited = OnCheckpointExited;

	callbacks->OnEntityPoolChange = OnEntityPoolChange;
	callbacks->OnServerPerformanceReport = OnServerPerformanceReport;

	callbacks->OnPlayerModuleList = OnPlayerModuleList;
}
