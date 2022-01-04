#include "script_component.hpp"

private _myDiscord = [getPlayerUID player] call FUNC(findMemberFromSteam);
if (_myDiscord isEqualTo []) then {
	systemChat "Your account is not linked, launching website";
	[QGVAR(generateLink), [player]] call CBA_fnc_serverEvent;
} else {
	systemChat format ["Welcome %1", _myDiscord#0];
	[QGVAR(linked)] call CBA_fnc_localEvent;

	player createDiarySubject [QGVAR(diary), "Commander"];

	private _roles = "";
	{
		_roles = format ["%1  - %2<br/>", _roles, (GVAR(roles) getOrDefault [_x, [format ["Invalid Role (%1)", _x]]])#0];
	} forEach _myDiscord#3;

	player createDiaryRecord [QGVAR(diary), [
		"My Account",
		format ["Name: %1<br/>Discord ID: %2<br/>Steam ID: %3<br/><br/>Roles:<br/>%4", _myDiscord#0, _myDiscord#1, _myDiscord#4, _roles]
	]];
};
