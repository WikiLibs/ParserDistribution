#include <stdio.h>

const char g_apikey[37] = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const char g_user[37] = "UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU";

int main()
{
    printf("API_KEY: %s\n", g_apikey);
    printf("USER_UUID: %s\n", g_user);
    return (0);
}
