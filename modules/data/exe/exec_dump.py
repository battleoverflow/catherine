"""
    Project: Catherine (https://github.com/CatherineFramework)
    Module by: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
"""

import pefile, datetime, sys

try:
    file_loc = sys.argv[1]
except IndexError:
    print("Missing file location argument")
    sys.exit()

try:
    try:
        pe = pefile.PE(str(file_loc))
    except FileNotFoundError:
        print("Unable to locate file. Please make sure the path is correct")
        sys.exit()
except pefile.PEFormatError:
    print("Wrong execution format. Module only accepts Windows execution format")
    sys.exit()

class ExecDumpWin:
    """
    This module dumps Windows executable information, mainly targeting binaries, DLLs, and similar files.
    Thanks to the `pefile` library, we can dump magic numbers, header information, and much more.
    """
    
    def bit_identifier() -> bool:
        if hex(pe.OPTIONAL_HEADER.Magic) == '0x10b':
            return False
        elif hex(pe.OPTIONAL_HEADER.Magic) == '0x20b':
            return True

    def exe_dump(self):
        print("\nFile Information")
        print(f"Magic Number (int): {pe.OPTIONAL_HEADER.Magic} (hex: {hex(pe.OPTIONAL_HEADER.Magic)})")

        if ExecDumpWin.bit_identifier():
            print(f"Binary info: {hex(pe.OPTIONAL_HEADER.Magic)} (64-bit)")
        elif ExecDumpWin.bit_identifier() != True:
            print(f"Binary info: {hex(pe.OPTIONAL_HEADER.Magic)} (32-bit)")

        print(f"TimeDateStamp: {pe.FILE_HEADER.dump_dict()['TimeDateStamp']['Value'].split('[')[1][:-1]}\n")

        # Dumps header info to a log file
        with open(f"exec_dump.log", "w") as f:
            f.write(pe.dump_info())

if __name__ == '__main__':
    try:
        ExecDumpWin.exe_dump()
    except NameError:
        pass