# notp
![Continuous Integration](https://github.com/kondanta/notp/workflows/Continuous%20Integration/badge.svg?branch=master&event=push)

notp is a CLI tool for generating one time passwords from CLI. Highly influenced by 2FA repository from rsc.

### Installation
You can clone the repository and build it from the source, download using `crate.io` or just download the binary that goes well with your OS
from the latest release

### Usage
```bash
# Let's see what we have here.
$ notp --help
# notp supports --key and --stdin exclusively. If you want to keep your secret secret,
# I urge you to use --stdin flag. --key is also usefull when you want to combine notp with
# shell tools and don't want to pass your secret using echo and such. Also, for the sake of 
# simplictiy I'll keep using --key flag throughout this documentation.
# Alrigh, lets say I'd like to add a Google account.
$ notp --add taylan-google --key superSecretKey
# you have to provide secret code when stdin prompts.
# OR
$ notp --add taylan-google --stdin
# Key: <you will type your encryption key here>
# Enter secret prompt: <You will paste your OTP secret here>
$ notp --list
$ notp --get <name> --key superSecretKey 
```
OTP generation only supports 6 digit codes, for now.

notp uses default config path on the OS that you're using. Such as `~/.config/notp` for Linux, `\AppData\Roaming\notp` for
Windows and `~/Library/Preferences/notp` for iOS and encrypts the file AES with the given key from CLI as an argument.

The default time-based authentication codes are derived from a hash of the key and the current time, 
so it is important that the system clock have at least one-minute accuracy.

### Example

Adding new secret:
```bash
$ notp --add AWS --key ttaayyllaann 
Please enter the secret: NRNM7KGFTR6SUMPBAEMBETM2WGKVUWHH6Y4VEGNPZON3GMVXBHF...
$
# There won't be any confirmation messages that indicates successfull insertion
# but there will be error messages if something wents wrong.
```

Checking existing secrets you have:
```bash
$ notp --list
1. AWS
2. Google
$
```

Generating the OTP code:
```bash
$ notp --get AWS --key ttaayyllaann
OTP code for the AWS: 442659
$ 
```

Deleting a secret:
```bash
$ notp --delete AWS
AWS Deleted!
```

Using with CLI:
```bash
$ notp --key ttaayyllaann -q --get AWS
442659
```
