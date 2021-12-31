#include "script_component.hpp"

if !(isServer) exitWith {};

GVAR(courses) = createHashMap;

addMissionEventHandler ["ExtensionCallback", {
	params ["_name", "_function", "_data"];
	
	if ((tolower _name) isNotEqualTo "dynulo_core") exitWith {};
	if ((tolower _function) isNotEqualTo "features:courses") exitWith {};

	INFO_1("courses: %1", _data);
}];

EXTFUNC("features:courses:fetch");
