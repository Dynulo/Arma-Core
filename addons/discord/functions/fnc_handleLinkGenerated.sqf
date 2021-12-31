#include "script_component.hpp"

[{time > 1}, {
	params ["_link"];
	"dynulo_utils" callExtension ["browser", [_link]];
	[{
		[] spawn {
			["Are you done linking your account?", "Account Linking", "Yes", false] call BIS_fnc_guiMessage;
			[QGVAR(memberRefresh)] call CBA_fnc_serverEvent;
		};
	}, [], 10] call CBA_fnc_waitAndExecute;
}, _this] call CBA_fnc_waitUntilAndExecute;
