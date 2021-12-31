#include "script_component.hpp"

params ["_method", "_path", "_body", ["_callback", {}], ["_args", []]];

GVAR(callbacks) set [
	EXT callExtension ["api:call", [_method, _path, _body]] select 0,
	[_callback, _args]
];
