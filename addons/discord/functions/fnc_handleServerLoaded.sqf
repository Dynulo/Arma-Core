#include "script_component.hpp"

INFO("Roles and Members loaded");

if (isServer) then {
	EXTFUNC("features:fetch");
};

if (hasInterface) then {
	QGVAR(members) addPublicVariableEventHandler {
		call FUNC(handleMembersRefreshed);	
	};
	call FUNC(handleMembersRefreshed);	
};
