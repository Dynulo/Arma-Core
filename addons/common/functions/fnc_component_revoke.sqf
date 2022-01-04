#include "script_component.hpp"

params ["_name"];

if (GVAR(loaded)) exitWith {
	WARNING_1("attempting to revoke %1 after loaded event", _name);
};

GVAR(components) deleteAt _name;
INFO_1("Component %1 revoked", _name);

call FUNC(component_check_ready);
