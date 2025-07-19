#include <vector>
typedef bool (*fn_StartRecordingPlayerData)(int32_t playerId, uint8_t recordtype, std::string recordname, uint32_t flags);
typedef bool (*fn_StopRecordingPlayerData)(int32_t playerId);
typedef bool (*fn_CallNPCClient)(const char* szName, const char* szScript, bool bConsoleInputEnabled,
	const char* host , const char* plugins, const char* execArg, const char* loc, std::vector<const char*>params );
typedef bool (*fn_IsPlayerNPC)(uint8_t playerId);
typedef uint8_t (*fn_StartRecordingAllPlayerData)(uint8_t recordtype, uint32_t flags, bool recordnewplayers);
typedef uint8_t (*fn_StopRecordingAllPlayerData)();
typedef bool (*fn_IsPlayerRecording)(uint8_t bytePlayerId);
typedef bool (*fn_PutServerInRecordingMode)(uint8_t recordtype, uint32_t flags, std::string filename);
typedef bool (*fn_StopServerInRecordingMode)();
typedef bool (*fn_IsServerInRecordingMode)();
typedef bool (*fn_StopConnectingMultipleNpcs)();
typedef uint8_t (*fn_ConnectMultipleNpcs)(std::string filename, std::string host, uint32_t flags, std::string execArg);
struct NPCExports{
	unsigned int uStructSize;
	fn_StartRecordingPlayerData StartRecordingPlayerData;
	fn_StopRecordingPlayerData StopRecordingPlayerData;
	fn_CallNPCClient CallNPCClient;
	fn_IsPlayerNPC IsPlayerNPC;
	fn_StartRecordingAllPlayerData StartRecordingAllPlayerData;
	fn_StopRecordingAllPlayerData StopRecordingAllPlayerData;
	fn_IsPlayerRecording IsPlayerRecording;
	fn_PutServerInRecordingMode PutServerInRecordingMode;
	fn_StopServerInRecordingMode StopServerInRecordingMode;
	fn_IsServerInRecordingMode IsServerInRecordingMode;
	fn_StopConnectingMultipleNpcs StopConnectingMultipleNpcs;
	fn_ConnectMultipleNpcs ConnectMultipleNpcs;
};
