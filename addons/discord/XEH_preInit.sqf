#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

[QEGVAR(common,serverLoaded), FUNC(handleServerLoaded)] call CBA_fnc_addEventHandler;

if (isServer) then {
	[QGVAR(roles)] call EFUNC(common,component_register);
	[QGVAR(members)] call EFUNC(common,component_register);

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
						GVAR(rolesImporting) = [];
					};
					case "entry": {
						GVAR(rolesImporting) pushBack (_data select 1);
					};
					case "done": {
						GVAR(roles) = +GVAR(rolesImporting);
						publicVariable QGVAR(roles);
						[QGVAR(roles)] call EFUNC(common,component_ready);
					};
				};
			};
			case "discord:members:fetch": {
				private _data = parseSimpleArray _data;
				private _cmd = _data select 0;
				switch (_cmd) do {
					case "clear": {
						GVAR(membersImporting) = [];
					};
					case "entry": {
						GVAR(membersImporting) pushBack (_data select 1);
					};
					case "done": {
						GVAR(members) = +GVAR(membersImporting);
						publicVariable QGVAR(members);
						[QGVAR(members)] call EFUNC(common,component_ready);
					};
				};
			};
			case "features:fetch": {
				private _data = parseSimpleArray _data;
				INFO_1("Features: %1", _data);
				{
					[QGVAR(feature), _x] call CBA_fnc_globalEvent;
				} forEach _data;
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
