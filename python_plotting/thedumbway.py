
import struct

def check_endianness():
    # Pack 0x01020304 as a 4-byte integer in "native" order
    native_bytes = struct.pack('I', 0x01020304)  # 'I' = unsigned int (4 bytes)

    # Check if native bytes match big-endian (>) or little-endian (<)
    if native_bytes == b'\x01\x02\x03\x04':
        return "Big-Endian"
    elif native_bytes == b'\x04\x03\x02\x01':
        return "Little-Endian"
    else:
        return "Unknown"

print("System Endianness:", check_endianness())




