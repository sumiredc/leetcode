#include <stdio.h>
#include <string.h>
#include <openssl/sha.h>

int main () {
    char data[] = "muhoho";
    size_t length = strlen(data);

    unsigned char hash[SHA_DIGEST_LENGTH];
    SHA1((unsigned char*)data, length, hash);
    
    for (int i = 0; i < SHA_DIGEST_LENGTH; i++) {
        printf("%02x", hash[i]);
    }
    printf("\n");
    
    return 0;
}

