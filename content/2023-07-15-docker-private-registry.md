+++
title = "Setup a private Docker registry"
date = 2023-07-15
+++

My internal infrastructure is complete. I can now work on my projects, but at some point they need to go out to the world!

The platform for most of my projects is the web, and the best tool I found so far to deploy them is Docker.

I want to keep the code on the private infrastructure, but I also want to be in control of where the docker images will be stored.

The perfect solution is a private Docker registry! But it will not be on the internal infrastructure, it will be publicly available on a regular server.

That way, projects can be deployed in their final form whenever and wherever, while the source remaining private.

# Get started locally

To start, I will launch a test registry on my machine to make sure everything works.

I will use these docker images:

- [registry](https://hub.docker.com/_/registry): The official registry made by Docker themselves
- [docker-registry-ui](https://joxit.dev/docker-registry-ui): A nice webui to view and manage images on the registry

Here's the docker-compose file I used to get started:
```yaml
services:

  registry-server:
    image: registry:2.8.2
    ports:
      - 5000:5000
    volumes:
      - ./registry-data:/var/lib/registry
      - ./passwords:/auth/htpasswd
    environment:
      REGISTRY_AUTH: 'htpasswd'
      REGISTRY_AUTH_HTPASSWD_REALM: 'Registry Realm'
      REGISTRY_AUTH_HTPASSWD_PATH: '/auth/htpasswd'
      REGISTRY_HTTP_HEADERS_Access-Control-Origin: '[http://registry.example.com]'
      REGISTRY_HTTP_HEADERS_Access-Control-Allow-Methods: '[HEAD,GET,OPTIONS,DELETE]'
      REGISTRY_HTTP_HEADERS_Access-Control-Credentials: '[true]'
      REGISTRY_HTTP_HEADERS_Access-Control-Allow-Headers: '[Authorization,Accept,Cache-Control]'
      REGISTRY_HTTP_HEADERS_Access-Control-Expose-Headers: '[Docker-Content-Digest]'
      REGISTRY_STORAGE_DELETE_ENABLED: 'true'
    container_name: registry-server

  registry-ui:
    image: joxit/docker-registry-ui:2.5.0
    ports:
      - 8001:80
    environment:
      SINGLE_REGISTRY: true
      REGISTRY_TITLE: Docker Registry UI
      DELETE_IMAGES: true
      SHOW_CONTENT_DIGEST: true
      NGINX_PROXY_PASS_URL: http://registry-server:5000
      SHOW_CATALOG_NB_TAGS: true
      CATALOG_MIN_BRANCHES: 1
      CATALOG_MAX_BRANCHES: 1
      TAGLIST_PAGE_SIZE: 100
      REGISTRY_SECURED: false
      CATALOG_ELEMENTS_LIMIT: 1000
    container_name: registry-ui
```

But don't start the services right away.

# Authentication

I don't want the registry being open to everyone though, let's add some authentication.

To keep things simple, I will use HTTP basic auth. If you want, there's a possibility to have a more [complex setup](https://docs.docker.com/registry/spec/auth/).

Here's a quick script to get passwords in a format that Docker will accept:
```sh
#!/bin/sh
#
# new-password.sh

if [ -z "$1" ]
then
echo "usage: $0 username"
exit 1
fi

echo "creating password for user \"$1\""
htpasswd -nB $1
```

How to use it:
```sh
$ ./new-password.sh phil
creating password for user "phil"
New password: phil
Re-type new password: phil
phil:$2y$05$asxsqfmEQJpg8zuKGyieMOmTirok.Gd/noliF.y48DJXe.97ufGHG
```

Copy the last line in the `passwords` file (see the `docker-compose` file).

Repeat the process for every user you want to give authentication to your registry.

Keep in mind I only cover **AUTHENTICATION** (who can access the registry), and not **AUTHORIZATION** (who can do what on the registry). With this setup, if you have access to the registry, you can do anything on it.

# Use the registry

Start the services with
```sh
docker compose up
```

Now, you can access the webui by going to
```
http://localhost:8001
```
in a web browser and sign-in with your credentials. You should see an empty list. Let's add some images!

## Naming images

Pick an image you want on the registry.

If it's an existing image:
```sh
docker tag name-of-existing-image localhost:5000/existing-image-name
```

If you build the image directly:
```sh
docker build -t localhost:5000/new-image-name
```

The name of the image must have the domain of the registry, in our case it's `localhost:5000`.

## Login to registry

To sign in to the registry, use
```sh
docker login localhost:5000
```
and enter your credentials.

## Push / Pull

Simply run the usual docker command to push or pull images. Docker will know which registry to use based of the image's name.

```sh
docker push localhost:5000/new-image-name
docker pull localhost:5000/existing-image-name
```

That's pretty much it!

# Deploy to production

I use [Caprover](https://caprover.com) to deploy my docker images easily, it comes with a reverse proxy and automatic TLS certificates with Let's encrypt.

Here's the one-click-app config I created for the registry:
```yaml
captainVersion: 4

services:
  $$cap_appname-registry:
    image: registry:$$cap_registry_version
    volumes:
      - $$cap_appname-data:/var/lib/registry
      - $$cap_appname-auth:/auth/
    environment:
      REGISTRY_AUTH: 'htpasswd'
      REGISTRY_AUTH_HTPASSWD_REALM: 'Registry Realm'
      REGISTRY_AUTH_HTPASSWD_PATH: '/auth/htpasswd'
      REGISTRY_HTTP_HEADERS_Access-Control-Origin: '[https://$$cap_appname-registry.$$cap_root_domain, https://$$cap_appname-ui.$$cap_root_domain]'
      REGISTRY_HTTP_HEADERS_Access-Control-Allow-Methods: '[HEAD,GET,OPTIONS,DELETE]'
      REGISTRY_HTTP_HEADERS_Access-Control-Credentials: '[true]'
      REGISTRY_HTTP_HEADERS_Access-Control-Allow-Headers: '[Authorization,Accept,Cache-Control]'
      REGISTRY_HTTP_HEADERS_Access-Control-Expose-Headers: '[Docker-Content-Digest]'
      REGISTRY_STORAGE_DELETE_ENABLED: 'true'
    caproverExtra:
        containerHttpPort: '5000'

  $$cap_appname-ui:
    image: joxit/docker-registry-ui:$$cap_ui_version
    environment:
      SINGLE_REGISTRY: true
      REGISTRY_TITLE: Docker Registry UI
      DELETE_IMAGES: true
      SHOW_CONTENT_DIGEST: true
      NGINX_PROXY_PASS_URL: http://srv-captain--$$cap_appname-registry:5000
      SHOW_CATALOG_NB_TAGS: true
      CATALOG_MIN_BRANCHES: 1
      CATALOG_MAX_BRANCHES: 1
      TAGLIST_PAGE_SIZE: 100
      REGISTRY_SECURED: false
      CATALOG_ELEMENTS_LIMIT: 1000

caproverOneClickApp:
    variables:
        - id: '$$cap_registry_version'
          label: Registry Version
          defaultValue: '2.8.2'
          description: Check out the Docker page for the valid tags https://hub.docker.com/_/registry/tags
          validRegex: "/.{1,}/"
        - id: '$$cap_ui_version'
          label: UI Version
          defaultValue: '2.5.0'
          description: Check out the Docker page for the valid tags https://hub.docker.com/r/joxit/docker-registry-ui/tags
          validRegex: "/.{1,}/"
    instructions:
        start: |-
            A private docker registry, with a webui to see images
        end: |-
            The registry has been deployed! Look in the "auth" volume to update credentials
    displayName: docker-registry-with-ui
    isOfficial: false
    description: A private docker registry, with a webui to see images
    documentation: https://docs.docker.com/registry/
```