Pinboard archiver
================

This is a Python script for backing up your bookmarks and saving them to your computer.

The original script was written and posted [in March 2013][1], and rewritten in January 2015. You can still get the original script, but the new version is hopefully more robust.

Requirements:

*   The `requests` and `keyring` module.
*   Your Pinboard API token stored in the system keychain (see below).
*   The [`applescript` module][2] from Dr. Drang

New features:

*   Rather than storing credentials in a plaintext file in the home directory, I'm now using the `keyring` module, which stores them in the OS keychain. This is more secure and convenient. To store a new token:

        >>> import keyring
        >>> keyring.set_password("pinboard", "username", "ABC123")

*   Better error handling and reporting. I run this script nightly, but I'd only realise it had failed if I happened to stumble across the directory where I was keeping the backups and noticed it was out-of-date. Oops.

    Now it integrates with [Dr Drang's applescript module][2] and puts up OS X notification banners if something goes wrong. If you're not on OS X, you can edit the `notification()` function to remove the AppleScript dependency.

*   Overall, it just benefits from nearly two years of additional programming experience (on my part), including several months of writing Python in my new job.

[1]: http://alexwlchan.net/blog/2013/03/pinboard-backups/
[2]: http://www.leancrew.com/all-this/2013/03/combining-python-and-applescript/