//
// SQImports: to be used by plugins that wish to interface with the
//     Squirrel host.
//
// Requires SQModule.h for full functionality of HSQAPI.
//

#pragma once
#include "SQModule.h"

typedef HSQAPI * (*Sq_GetSquirrelAPI) (void);
typedef HSQUIRRELVM * (*Sq_GetSquirrelVM) (void);

struct SquirrelImports
{
	unsigned int		uStructSize;
	Sq_GetSquirrelAPI	GetSquirrelAPI;
	Sq_GetSquirrelVM	GetSquirrelVM;
};