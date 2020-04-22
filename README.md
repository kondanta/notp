# notp
notp is a CLI tool for generating one time passwords from CLI. Highly influenced by 2FA repository from rsc.

### Installation
You can clone the repository and build it from the source, download using `crate.io` or just download the binary that goes well with your OS
from the latest release

### Usage
```bash
notp --help
notp --add <name> --key superSecretKey
# you have to provide secret code when stdin prompts.
notp --key superSecretKey --list
notp --key superSecretKey --get <name>
```
OTP generation only supports 6 digit codes, for now.

notp uses default config path on the OS that you're using. Such as `~/.config/notp` for Linux, `\AppData\Roaming\notp` for
Windows and `~/Library/Preferences/notp` for iOS and encrypts the file AES with the given key from CLI as an argument.

The default time-based authentication codes are derived from a hash of the key and the current time, 
so it is important that the system clock have at least one-minute accuracy.

### Example

Adding new secret:
```bash
$ notp --key ttaayyllaann --add AWS
Please enter the secret: NRNM7KGFTR6SUMPBAEMBETM2WGKVUWHH6Y4VEGNPZON3GMVXBHF...
$
# There won't be any confirmation messages that indicates successfull insertion
# but there will be error messages if something wents wrong.
```

Checking existing secrets you have:
```bash
$ notp --key ttaayyllaann --list
1. AWS
2. Google
$
```

Generating the OTP code:
```bash
$ notp --key ttaayyllaann --get AWS
OTP code for the AWS: 442659
$ 
```

Deleting a secret:
> Since notp does not support deleting a key, you can manually remove the secret from file. Each key has splitted with a
> comma and the order is the same as the output you see when you use --list command.
```bash
# Example for the macOS
$ cat ~/Library/Preferences/notp/notp
asldkasld==,lbvkjcksdf==
$ vim ~/Library/Preferences/notp/notp
# delete the one that starts lb and and ==. That means you have deleted the 2. key that is listed when --list used.
```

Using with CLI:
```bash
$ notp --key ttaayyllaann -q --get AWS
442659
```