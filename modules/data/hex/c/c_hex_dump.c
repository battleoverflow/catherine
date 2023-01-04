/*
    Project: Catherine (https://github.com/CatherineFramework)
    Module by: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

/*
    The primary focus of the c_hex_dump module is to offer an alternative to the Rust hex_dump function for dumping hexadecimal information about a file.
*/

#include <stdio.h>
#include <stdlib.h>

int ascii_to_hex(char c) {
    int n = (int) c;
    
    if (n < 58 && n > 47) { return n - 48; }
    if (n < 103 && n > 96) { return n - 87; }

    return n;
}

void collect_hex(char *filename) {

    // Opens file for reading, allowing us to obtain hexadecimal information
    FILE *fp = fopen(filename, "r");

    // Seeks the file to locate the beginning and end of the file (file_len)
    fseek(fp, 0, SEEK_END);
    int file_len = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    
    unsigned char conversion_one, conversion_two;
    unsigned char final_result, return_hex[file_len / 2];

    // Saves hex informatation to a file
    FILE *call_c_lib = fopen("c_hex_dump.hex", "wb+");

    // Hits an error if unable to create the file
    if (call_c_lib == NULL) {
        printf("Unable to create a file\n");
        exit(EXIT_FAILURE);
    }

    int i = 0;

    // Iterates over the file for the length of the file
    for (i = 0; i < file_len / 2; i++) {
        conversion_one = ascii_to_hex(fgetc(fp));
        conversion_two = ascii_to_hex(fgetc(fp));
        final_result = conversion_one << 4 | conversion_two;
        return_hex[i] = final_result;

        // Writes data to a file
        fprintf(call_c_lib, "%02x ", final_result);
        
        // Prints data to stdout
        // printf("%02x ", final_result);
    }

    // Closes file
    fclose(call_c_lib);
    printf("\n");
}