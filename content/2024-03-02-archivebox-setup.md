+++
title = "First setup of Archivebox"
date = 2024-03-02
+++

I discovered [Archivebox](https://archivebox.io) and decided to install it on my server using containers.

I just had to make a couple of adjustements, because all the content on the instance is publically available. I do not want that, I want to restrict access with user accounts.

To do so, start by finding out the container name, and open a shell:

```shell
docker exec --user=archivebox -it container-name-goes-here bash
```

Go into the directory where Archivebox data is stored, and you will be able to run command to manage the instance.

## Hide everything from the public

```shell
archivebox config --set SAVE_ARCHIVE_DOT_ORG=False
archivebox config --set PUBLIC_INDEX=False
archivebox config --set PUBLIC_SNAPSHOTS=False
archivebox config --set PUBLIC_ADD_VIEW=False
```

## Create first user account

Now that you cut outside world access to your instance, create an admin user to access and add new content:

```shell
archivebox manage createsuperuser
```

Once finished, exit the shell and restart Archivebox.
