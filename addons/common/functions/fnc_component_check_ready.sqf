#include "script_component.hpp"

if (GVAR(loaded)) exitWith {};

scopeName "all_true";
private _all_true = true;
{
	if !(_y) exitWith {
		_all_true = false;
		breakTo "all_true";
	};
} forEach GVAR(components);

if (_all_true) then {
	INFO("All components loaded");
	[QGVAR(serverLoaded)] call CBA_fnc_serverEvent;
	GVAR(loaded) = true;
};
