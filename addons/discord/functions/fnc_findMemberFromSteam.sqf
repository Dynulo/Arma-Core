#include "script_component.hpp"

params ["_steam"];

private _index = (values GVAR(members)) findIf {
	_x params ["", "", "", "", "_s"];
	_s isEqualTo _steam
};

if (_index == -1) then {
	[]
} else {
	GVAR(members) getOrDefault [((keys GVAR(members)) select _index), ["","","","",""]]
}
