#include "script_component.hpp"

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_component", "_data"];
	if ((tolower _name) isNotEqualTo "dynulo_log") exitWith {};
	_data params ["_level", "_message"];
	diag_log text format ["[DYNULO] (%1) %2: %3", _component, _level, _message];
}];

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];
	if ((tolower _name) isNotEqualTo "dynulo_core") exitWith {};

	switch (_function) do {
		case "apicall": {
			(parseSimpleArray _data) params ["_id", "_response"];
			INFO_2("API Callback: %1 %2",_id,_response);
			[_response] call (GVAR(callbacks) getOrDefault [_id, {}]);
		};
	};
}];
