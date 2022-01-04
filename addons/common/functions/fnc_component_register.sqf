#include "script_component.hpp"

params ["_name"];

if (GVAR(loaded)) exitWith {
	WARNING_1("attempting to register %1 after loaded event", _name);
};

GVAR(components) set [_name, false];
INFO_1("Component %1 registered", _name);
