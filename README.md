# FFXIV-OTP

This is just a nifty little tool that sends TOTP codes to XIVLauncher.Core.

**Note**: If you value account security, this tool is not for you. You need to pass the TOTP secret as a parameter, which is plain insecure.

Maybe in the future, I will revisit this tool to interface with secret management tools but until then, this remains as a convenience tool.

## Installing

You need Rust installed on your machine. Last tested version was 1.65.0.

```bash
cargo install ffxiv-otp
```

## How to use

If your XIVLauncher.Core isn't modified, the following should work:

```bash
ffxiv-otp -s [your TOTP secret]
```

In order to get your TOTP secret, you may need to extract it from your TOTP tool.
How that works is different from tool to tool.
I also recommend storing that secret in some sort of secret management software and extract the secret from there.

Personally I use secret-tool for this, which stores secrets in a password-protected store.

```bash
ffxiv-otp -s $(secret-tool lookup service my-ffxiv-otp-secret)
```

My TOTP secret is stored in a service called `my-ffxiv-otp-secret` and in order to extract it, I need to enter my password.

Again, don't come here looking for a secure solution. This is all just convenience.

It should just work fine on Windows the same way, though it's not tested.

You can also get the raw TOTP code by passing -S to it:

```bash
ffxiv-otp -s [your TOTP secret] -S
```

This prints your TOTP code into stdout.
You could pipe this into sclip to have your TOTP code in your clipboard, which you can paste into the launcher as opposed to having it sent.
