#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_component", "_data"];
	if ((tolower _name) isNotEqualTo "dynulo_log") exitWith {};
	(parseSimpleArray _data) params ["_level", "_message"];
	diag_log text format ["[DYNULO] (%1) %2: %3", _component, _level, _message];
}];

if (isServer && {isMultiplayer}) then {
	GVAR(ready) = false;
	GVAR(loaded) = false;

	GVAR(components) = createHashMap;
	GVAR(callbacks) = createHashMap;

	[QGVAR(component_ready), FUNC(component_ready)] call CBA_fnc_addEventHandler;
	[QGVAR(component_register), FUNC(component_register)] call CBA_fnc_addEventHandler;
	[QGVAR(component_revoke), FUNC(component_revoke)] call CBA_fnc_addEventHandler;

	addMissionEventHandler ["ExtensionCallback", {
		params ["_name", "_function", "_data"];
		if ((tolower _name) isNotEqualTo "dynulo_core") exitWith {};

		switch (_function) do {
			case "core:ready": {
				GVAR(ready) = true;
				GVAR(discord) = _data;
				[QGVAR(serverReady)] call CBA_fnc_serverEvent;
				EXTFUNC("features:fetch");
			};
			case "features:fetch": {
				private _data = parseSimpleArray _data;
				INFO_1("Features: %1", _data);
				[QGVAR(features), [_data]] call CBA_fnc_serverEvent;
				{
					[QGVAR(feature), _x] call CBA_fnc_globalEvent;
				} forEach _data;
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
