# totp
topt is a CLI tool for generating one time passwords from CLI. Highly influenced by 2FA repository from rsc.

### Installation
You can clone the repository and build it from the source or just download the binary that goes well with your OS
from the latest release

### Usage
```bash
totp --help
totp --add <name> --key superSecretKey
# you have to provide secret code when stdin prompts.
totp --key superSecretKey --list
totp --key superSecretKey --get <name>
```
OTP generation only supports 6 digit codes, for now.

totp uses default config path on the OS that you're using. Such as `~/.config/totp` for Linux, `\AppData\Roaming\totp` for
Windows and `~/Library/Preferences/totp` for iOS and encrypts the file AES with the given key from CLI as an argument.

The default time-based authentication codes are derived from a hash of the key and the current time, 
so it is important that the system clock have at least one-minute accuracy.

### Example

Adding new secret:
```bash
$ totp --key ttaayyllaann --add AWS
Please enter the secret: NRNM7KGFTR6SUMPBAEMBETM2WGKVUWHH6Y4VEGNPZON3GMVXBHF...
$
# There won't be any confirmation messages that indicates successfull insertion
# but there will be error messages if something wents wrong.
```

Checking existing secrets you have:
```bash
$ totp --key ttaayyllaann --list
1. AWS
2. Google
$
```

Generating the OTP code:
```bash
$ totp --key ttaayyllaann --get AWS
OTP code for the AWS: 442659
$ 
```

Deleting a secret:
> Since totp does not support deleting a key, you can manually remove the secret from file. Each key has splitted with a
> comma and the order is the same as the output you see when you use --list command.
```bash
# Example for the macOS
$ cat ~/Library/Preferences/totp/totp
asldkasld==,lbvkjcksdf==
$ vim ~/Library/Preferences/totp/totp
# delete the one that starts lb and and ==. That means you have deleted the 2. key that is listed when --list used.
```

Using with CLI:
```bash
$ totp --key ttaayyllaann -q --get AWS
442659
```