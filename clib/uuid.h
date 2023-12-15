#ifndef UUID_H
#define UUID_H

#include <windows.h>

BOOL UuidDeobfuscation(IN CHAR* UuidArray[], IN SIZE_T NmbrOfElements, OUT PBYTE* ppDAddress, OUT SIZE_T* pDSize);
typedef RPC_STATUS (WINAPI* fnUuidFromStringA)(
	RPC_CSTR	StringUuid,
	UUID*		Uuid
);
#endif
