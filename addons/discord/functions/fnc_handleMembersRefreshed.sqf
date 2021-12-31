#include "script_component.hpp"

private _myDiscord = [getPlayerUID player] call FUNC(findMemberFromSteam);
if (_myDiscord isEqualTo []) then {
	systemChat "Your account is not linked, launching website";
	[QGVAR(generateLink), [player]] call CBA_fnc_serverEvent;
} else {
	systemChat format ["Welcome %1", _myDiscord#0];
	[QGVAR(linked)] call CBA_fnc_localEvent;
};
