#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

if (isServer) then {
	[QEGVAR(discord,feature), FUNC(handleEnabled)] call CBA_fnc_addEventHandler;
};
