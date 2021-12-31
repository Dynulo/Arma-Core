#include "script_component.hpp"

params ["_name"];

GVAR(components) set [_name, false];
INFO_1("Component %1 registered", _name);
