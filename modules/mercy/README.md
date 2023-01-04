# Mercy Extension Module

Mercy is a unique module created to be the main source of all things decryption, decoding, and string manipulation. This module is an extension of the Mercy Rust crate.

The following methods are currently supported:

|  Method     |  Ability  |
|-------------|-----------|
|  Base64     |  Decode   |
|  Rot13      |  Decode   |
|  Base32     |  Decode   |
|  JWT Token  |  Decode   |
|  Fernet     |  Decrypt  |

NOTE: We do not currently support encoding or encrypting since the core functionality of the framework is oriented toward defensive-type operations.