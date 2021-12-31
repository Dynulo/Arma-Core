#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(ready) = false;
GVAR(components) = createHashMap;
GVAR(callbacks) = createHashMap;

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_component", "_data"];
	if ((tolower _name) isNotEqualTo "dynulo_log") exitWith {};
	systemChat format ["log: %1", _this];
	(parseSimpleArray _data) params ["_level", "_message"];
	diag_log text format ["[DYNULO] (%1) %2: %3", _component, _level, _message];
}];

if (isServer && {isMultiplayer}) then {
	addMissionEventHandler ["ExtensionCallback", {
		params ["_name", "_function", "_data"];
		if ((tolower _name) isNotEqualTo "dynulo_core") exitWith {};

		switch (_function) do {
			case "core:ready": {
				GVAR(ready) = true;
				GVAR(discord) = _data;
				[QGVAR(serverReady)] call CBA_fnc_serverEvent;
			};
			case "api:call": {
				(parseSimpleArray _data) params ["_id", "_response"];
				INFO_2("API Callback: %1 %2",_id,_response);
				(GVAR(callbacks) getOrDefault [_id, [{},[]]]) params ["_code", "_args"];
				GVAR(callbacks) deleteAt _id;
				INFO_2("API Callback: %1 %2",_code,_args);
				[_response, _args] call _code;
			};
		};
	}];
};
