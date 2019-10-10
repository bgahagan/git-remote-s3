Git Remote S3 Helper
====================

Push and pull git repos to/from an s3 bucket.
Uses gpg to encrypt the repo contents (but not branch names!) before sending
to s3.

This likely most useful for small teams who don't want to host their own
private repository, but still want to manage their own encryption.
For example, my use case is preiodically backing up a repo from a desktop
and pull to a laptop to develop remotely.


Example Usage
-------------

Add a remote using the `s3` transport:
```
git remote add s3remote s3://my_bucket/prefix
```

And then you can push/pull to the remote as usual:

```
git pull s3remote master

git push s3remote
```

Or even clone from s3:
```
git clone s3://my_bucket/prefix
```


Installation
------------

* Put `git-remote-s3` in your path
  * Download the latest release [here](https://github.com/bgahagan/git-remote-s3/releases/latest).
* Make sure s3 credentials are setup
  * See [here](https://docs.rs/rusoto_credential/0.40.0/rusoto_credential/struct.ChainProvider.html) for details on how the rusuto library loeas as credentials (similar to the aws command line).
* Setup gpg
  * gpg encryption will be attempted using `git config user.email` as a recipient. You'll want to ensure you have public and private keys setup for this user.
  * Alternatively, you can set a list of space-delimnated recipients using the `remote.<name>.gpgRecipients`config.

Design Notes
------------
Due to the eventual consistancey behaviour of s3, the symantics of pushing are
slightly different when pushing to a 'proper' git repository.
An attempt is made to prevent non-force pushes that do not include the current
head as an ancestor (as proper git repos do), but evantual consistency means
this is not guaranteed.
Its possible for multiple heads to exist for the same branch, in which case
the clients consider the newest head to be the truth.
All heads for a branch can be seen using `git ls-remote` - the latest (newest)
head the have the branch's name; older head will be shown using the nameing
scheme: `<branch_name>__<sha>`.
An old head is retained until a new head is pushed that includes the old head
as an ancestor, at which point the old head is deleted.
This prevents any data loss, but puts the burden on the user to manually merge
in old braches.

Each branch is stored (after being bundled with `git bundle` and encrypted with
`gpg`) on s3 using the key `s3://bucket/prefix/<ref_name>/<sha>.bundle`.
On average, a `git push` will incur two list, a put and a delete s3 operation.
A `git pull` will incur a list and a get s3 operation.


Future improvments
------------------

* A better way to notify the user there are multiple heads on s3.
  * Show warning when attempting to push/fetch and there are multiple heads for a branch?
* Allow disabling gpg with `remote.<name>.gpg`
* use `gpg.program`
