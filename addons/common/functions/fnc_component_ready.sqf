#include "script_component.hpp"

params ["_name"];

if (GVAR(loaded)) exitWith {
	WARNING_1("attempting to ready %1 after loaded event", _name);
};

GVAR(components) set [_name, true];
INFO_1("Component %1 ready", _name);

call FUNC(component_check_ready);
