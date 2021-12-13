#include "script_component.hpp"

params ["_method", "_path", "_body", "_callback"];

private _id = EXT callExtension "uuid";

GVAR(callbacks) set [_id, _callback];

EXT callExtension ["apicall", [_id, _method, _path, _body]];
