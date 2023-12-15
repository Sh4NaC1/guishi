#ifndef _RC4_H_
#define _RC4_H_
#include <windows.h>
typedef struct
{
	unsigned int i;
	unsigned int j;
	unsigned char s[256];

} Rc4Context;

void rc4Init(Rc4Context* context, const unsigned char* key, size_t length);

VOID PrintHexData(LPCSTR Name, PBYTE Data, SIZE_T Size);
void rc4Cipher(Rc4Context* context, const unsigned char* input, unsigned char* output, size_t length);
#endif // _AES_H_
