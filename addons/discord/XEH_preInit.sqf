#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

[QEGVAR(common,serverLoaded), FUNC(handleServerLoaded)] call CBA_fnc_addEventHandler;

if (isServer) then {
	[QEGVAR(common,serverReady), FUNC(handleServerReady)] call CBA_fnc_addEventHandler;

	[QGVAR(generateLink), {
		params ["_player"];
		["POST", format ["/members/link/generate/%1", getPlayerUID _player], "", {
			params ["_response", "_args"];
			_args params ["_player"];
			INFO_1("Generated link for %1", _player);
			[QGVAR(linkGenerated), [_response], _player] call CBA_fnc_targetEvent;
		}, [_player]] call EFUNC(common,apicall);
	}] call CBA_fnc_addEventHandler;

	addMissionEventHandler ["ExtensionCallback", {
		params ["_name", "_function", "_data"];
		if ((tolower _name) isNotEqualTo "dynulo_core") exitWith {};

		switch (_function) do {
			case "discord:roles:fetch": {
				private _data = parseSimpleArray _data;
				private _cmd = _data select 0;
				switch (_cmd) do {
					case "clear": {
						GVAR(rolesImporting) = createHashMap;
					};
					case "entry": {
						private _role = _data select 1;
						private _id = _role deleteAt 0;
						GVAR(rolesImporting) set [_id, _role];
					};
					case "done": {
						GVAR(roles) = +GVAR(rolesImporting);
						publicVariable QGVAR(roles);
						[QEGVAR(common,component_ready), QGVAR(roles)] call CBA_fnc_globalEvent;
					};
				};
			};
			case "discord:members:fetch": {
				private _data = parseSimpleArray _data;
				private _cmd = _data select 0;
				switch (_cmd) do {
					case "clear": {
						GVAR(membersImporting) = createHashMap;
					};
					case "entry": {
						private _member = _data select 1;
						private _id = _member select 1;
						GVAR(membersImporting) set [_id, _member];
					};
					case "done": {
						GVAR(members) = +GVAR(membersImporting);
						publicVariable QGVAR(members);
						[QEGVAR(common,component_ready), QGVAR(members)] call CBA_fnc_globalEvent;
					};
				};
			};
			
		};
	}];

	[QGVAR(memberRefresh), {
		EXTFUNC("discord:members:fetch");
	}] call CBA_fnc_addEventHandler;
};

if (hasInterface) then {
	[QGVAR(linkGenerated), FUNC(handleLinkGenerated)] call CBA_fnc_addEventHandler;
};
