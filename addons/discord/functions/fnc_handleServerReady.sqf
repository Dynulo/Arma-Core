#include "script_component.hpp"

INFO("Loading discord components");

[QEGVAR(common,component_register), QGVAR(roles)] call CBA_fnc_serverEvent;
[QEGVAR(common,component_register), QGVAR(members)] call CBA_fnc_serverEvent;

EXTFUNC("discord:roles:fetch");
EXTFUNC("discord:members:fetch");
