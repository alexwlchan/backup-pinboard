#!/usr/bin/python

from datetime import datetime
import optparse
import os
import sys

import keyring
import requests

import applescript

#------------------------------------------------------------------------------
# Global constants
#------------------------------------------------------------------------------
PINBOARD_API = 'https://api.pinboard.in/v1/'
VALID_FORMATS = ['json', 'xml']

#------------------------------------------------------------------------------
# Utility functions
#------------------------------------------------------------------------------
def notification(message_str):
    """Makes a notification about the backup status, and prints the error to
    stdout.
    """
    print(message_str)
    osacommand = ''.join([
        'display notification "%s" ' % message_str,
        'with title "Pinboard backup"'
    ])
    applescript.asrun(osacommand)

def error_notification(error_str):
    notification(error_str)
    sys.exit(1)

def backup_file(backup_dir, dumpformat="xml"):
    """Returns the name of the backup file, and creates the intermediate
    directories.
    """
    # Get the year and month-day strings
    year = datetime.now().strftime("%Y")
    today = datetime.now().strftime("%m-%d")

    # Backups are organised by year directory
    year_dir = os.path.join(backup_dir, year)
    if not os.path.isdir(year_dir):
        os.makedirs(year_dir)

    filename = "pinboard-backup.{}.{}".format(today, dumpformat)
    backup_file = os.path.join(year_dir, filename)

    return backup_file

def authentication_token(username):
    """Returns the authentication token for Pinboard, or raises an exception if
    it isn't found.
    """
    auth_token = keyring.get_password("pinboard", username)

    # Check whether it found an API token
    if auth_token is None:
        error_notification("No Pinboard API token found for user %s." % username)

    return auth_token

def backup_all_posts(auth_token, outfile, dumpformat="xml"):
    """Download all the posts from the Pinboard API, and write them out to a
    file.
    """
    payload = {
        "auth_token": auth_token,
        "format": dumpformat
    }

    # Download all the posts from the API
    req = requests.get(
        PINBOARD_API + 'posts/all',
        params=payload
    )

    # Check we have a successful status code
    if not req.status_code == requests.codes.ok:
        error_notification("Bad return code from the Pinboard API.")

    # Otherwise try to write the backup file
    try:
        with open(outfile, 'w') as f:
            f.write(req.text.encode('utf-8'))
    except IOError:
        error_notification("Couldn't create new backup file at %s" % outfile)

    notification("Bookmarks successfully backed up to %s!" % outfile)

#------------------------------------------------------------------------------
# Mainline program function
#------------------------------------------------------------------------------
def main():

    #--------------------------------------------------------------------------
    # Set up command-line options
    #--------------------------------------------------------------------------
    parser = optparse.OptionParser(description="""A script for backing up
Pinboard bookmarks.""")
    parser.add_option("-d", "--directory",
                      help="Directory for bookmark file (default ~)")
    parser.add_option("-f", "--format",
                      help="Format for dump (xml or json, default xml)")
    parser.add_option("-u", "--username",
                      help="Username of the Pinboard account.")
    (options, args) = parser.parse_args()

    #--------------------------------------------------------------------------
    # Validate user input
    #--------------------------------------------------------------------------
    if options.directory is None:
        options.directory = os.environ['HOME']

    if options.format is None:
        options.format = 'xml'
    else:
        if options.format not in VALID_FORMATS:
            error_notification("Invalid dump format %s." % options.format)

    if options.username is None:
        error_notification("No username supplied.")

    #--------------------------------------------------------------------------
    # Get the bookmarks
    #--------------------------------------------------------------------------
    outfile = backup_file(options.directory, options.format)
    auth_token = authentication_token(options.username)
    backup_all_posts(auth_token=auth_token, outfile=outfile)

if __name__ == '__main__':
    main()