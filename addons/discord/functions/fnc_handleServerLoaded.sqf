#include "script_component.hpp"

if (hasInterface) then {
	QGVAR(members) addPublicVariableEventHandler {
		call FUNC(handleMembersRefreshed);	
	};
	call FUNC(handleMembersRefreshed);	
};
