#!/bin/bash

set -euo pipefail

if gpg --fingerprint --with-colons 'test@example.com' | grep "example.com" ; then 
  echo Key for test@example.com exists
  exit
fi

todelete() {
  gpg --fingerprint --with-colons 'test@example.com' |\
    grep "^fpr" |\
    sed -n 's/^fpr:::::::::\([[:alnum:]]\+\):/\1/p'
}

#while read line; do
#  gpg --yes --delete-secret-keys "$line" </dev/null
#  gpg --yes --delete-keys "$line" </dev/null
#done < <(todelete)

keydetails() {
  cat <<EOF
    %echo Generating a basic OpenPGP key
    Key-Type: RSA
    Key-Length: 2048
    Subkey-Type: RSA
    Subkey-Length: 2048
    Name-Real: Test User
    Name-Comment: Test User
    Name-Email: test@example.com
    Expire-Date: 0
    %no-ask-passphrase
    %no-protection
    #%pubring pubring.kbx
    #%secring trustdb.gpg
    # Do a commit here, so that we can later print "done" :-)
    %commit
    %echo done
EOF
}

gpg --verbose --batch --gen-key <(keydetails)

# Set trust to 5 for the key so we can encrypt without prompt.
#echo -e "5\ny\n" |  gpg --command-fd 0 --expert --edit-key 'test@example.com' trust;

# Test that the key was created and the permission the trust was set.
gpg --list-secret-keys

# Test encrypting a file
gpg -v --batch -r test@example.com -o /tmp/enc-test.out -e tests/run.sh
