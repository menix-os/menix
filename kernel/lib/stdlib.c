/*------------------------------------------
Kernel C library - "stdlib.h" implementation
------------------------------------------*/

#include <menix/string.h>
#include <menix/stdint.h>
#include <menix/stdlib.h>

void reverse(char* s)
{
    int i, j;
    char c;

    for (i = 0, j = strlen(s) - 1; i < j; i++, j--)
    {
        c = s[i];
        s[i] = s[j];
        s[j] = c;
    }
}

char* itoa(int32_t value, char* str, uint32_t base)
{
    int32_t i, sign;
    sign = value;

    if (base == 10 && sign < 0)
        value = -value;
    i = 0;
    do
    {
        char c = value % base;
        if (c > 9)
            c += 7; // Skip to the letters for hex. ('9' + 7 = 'A')
        str[i++] = c + '0';
    } while ((value /= base) > 0);
    if (base == 10 && sign < 0)
        str[i++] = '-';
    str[i] = '\0';
    reverse(str);
    return str;
}

char* utoa(uint32_t value, char* str, uint32_t base)
{
    int i, sign;
    i = 0;
    do
    {
        char c = value % base;
        if (c > 9)
            c += 7; // Skip to the letters for hex. ('9' + 7 = 'A')
        str[i++] = c + '0';
    } while ((value /= base) > 0);
    str[i] = '\0';
    reverse(str);
    return str;
}
