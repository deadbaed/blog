+++
title = "Setup Caddy with a CA and ACME server on Alpine Linux"
date = 2023-06-28
+++

Now that we have a WireGuard VPN with an awesome internal DNS server, let's get a web server with HTTPS!

# Caddy

## Install

You will need to enable the `community` repo first.
```sh
doas apk add caddy
```

## Configuration

Create a folder to serve stuff from, I placed it in
```
/srv/www
```

Create the config in
```sh
/etc/caddy/Caddyfile
```

Here's the config, it's very simple to get started:
```
intra.philt3r:80
root * /srv/www
file_server browse
```

This config will only launch an HTTP server, the HTTPS will come later.

It should work only from the WireGuard peers, since they can resolve the DNS name `intra.philt3r`.

If there is no `index.html` in the folder, it will serve static files directly.

## Script to launch

Caddy already has a service!

- Start: `rc-service caddy start`
- Stop: `rc-service caddy stop`
- Reload configuration without downtime: `rc-service caddy reload`

# Generate keys and certificates

We will generate the Root CA, the Intermediate CA.

Generate these with `openssl` installed on a computer, preferabbly offline.

Make sure the keys are stored in a safe place, I will store mine inside of a KeePassXC keystore.

## OpenSSL Configuration

inside a folder, create a file
```
config.conf
```

In `[CA_root]`, make sure to put your folder `dir`

```ini
# OpenSSL root CA configuration file.

[ ca ]
# `man ca`
default_ca = CA_root

[ CA_root ]
# Directory and file locations.
dir               = /home/phil/ca
certs             = $dir/certs
crl_dir           = $dir/crl
new_certs_dir     = $dir/newcerts
database          = $dir/index.txt
serial            = $dir/serial
RANDFILE          = $dir/private/.rand

# The root key and root certificate.
# Match names with Smallstep naming convention
private_key       = $dir/root_ca_key
certificate       = $dir/root_ca.crt

# For certificate revocation lists.
crlnumber         = $dir/crlnumber
crl               = $dir/crl/ca.crl.pem
crl_extensions    = crl_ext
default_crl_days  = 30

# SHA-1 is deprecated, so use SHA-2 instead.
default_md        = sha256

name_opt          = ca_default
cert_opt          = ca_default
default_days      = 25202
preserve          = no
policy            = policy_strict

[ policy_strict ]
# The root CA should only sign intermediate certificates that match.
# See the POLICY FORMAT section of `man ca`.
countryName             = match
stateOrProvinceName 	= supplied
localityName	    	= supplied
organizationName        = match
commonName              = supplied

[ req ]
# Options for the `req` tool (`man req`).
default_bits        = 4096
distinguished_name  = req_distinguished_name
string_mask         = utf8only

# SHA-1 is deprecated, so use SHA-2 instead.
default_md          = sha256

# Extension to add when the -x509 option is used.
x509_extensions     = v3_ca

[ req_distinguished_name ]
# See <https://en.wikipedia.org/wiki/Certificate_signing_request>.
countryName          = Country (2 letter code)
stateOrProvinceName  = State or Region
localityName         = City
commonName           = Common Name
0.organizationName   = Organization Name

[ v3_ca ]
# Extensions for a typical CA (`man x509v3_config`).
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
basicConstraints = critical, CA:true
keyUsage = critical, digitalSignature, cRLSign, keyCertSign

[ v3_intermediate_ca ]
# Extensions for a typical intermediate CA (`man x509v3_config`).
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
basicConstraints = critical, CA:true, pathlen:0
keyUsage = critical, digitalSignature, cRLSign, keyCertSign

```

After, run
```sh
mkdir newcerts
touch index.txt
echo 1420 > serial
```

We are now ready to generate keys and certificates.

## Root key and certificate

Generate key:
```sh
openssl genrsa -aes256 -out root_ca_key 4096
```
It will ask for a passphrase, I generated mine with my KeePassXC.

Generate root certificate:
```sh
openssl req -config config.conf -key root_ca_key -days 3650 -new -x509 -sha256 -extensions v3_ca -out root_ca.crt
```

My root CA will last for 3650 days (10 years).

Here's the info I provided:
```
Country (2 letter code) []:FR
State or Region []:Bretagne
City []:Rennes
Common Name []:philt3r CA
Organization Name []:philt3r
```

I saved the `root_ca_key` and `root_ca.crt` inside my KeePassXC.

## Intermediate key and certificate

Generate key:
```sh
openssl genrsa -aes256 -out intermediate_ca_key 4096
```
It will ask for a passphrase, I generated mine with my KeePassXC.

Generate certificate request:
```sh
openssl req -config config.conf -new -sha256 -key intermediate_ca_key -out intermediate_ca.csr.pem
```

Here's the info I provided:
```
Country (2 letter code) []:FR
State or Region []:Bretagne
City []:Rennes
Common Name []:philt3r Intermediate CA 
Organization Name []:philt3r
```

Sign certificate request with Root key:
```sh
openssl ca -config config.conf -keyfile root_ca_key -cert root_ca.crt -extensions v3_intermediate_ca -days 1825 -notext -md sha256 -in intermediate_ca.csr.pem -out intermediate_ca.crt
```
My Intermediate certificate will last for 1825 days (5 years).

Save these files, I saved them in my KeePassXC:
- `intermediate_ca_key`
- `intermediate_ca.csr.pem`
- `intermediate_ca.crt`

Once everything is saved and backed up, delete everything from your computer securely.

# CA and ACME server

I discovered [Smallstep](https://smallstep.com/), which allows to become your own ACME server.

## Install

They provide packages for Alpine, but one of them is only in the testing repos.

Edit
```
/etc/apk/repositories
```

And add:
```
@testing http://mirrors.ircam.fr/pub/alpine/edge/testing
```

Afterwards, run
```sh
apk update
```
to refresh the packages.

Install the packages with
```sh
apk add step-cli step-certificates@testing
```

The `@testing` is to tell `apk` to pull the package from the testing repo.

## Configuration

Start by creating the folder where `step` will save all the configs:
```sh
mkdir /etc/step-ca -p
```

Let's configure `step-ca`!
```sh
STEPPATH=/etc/step-ca step ca init --name="philt3r" --acme --address="10.131.111.1:444" --provisioner="philt3r" --deployment-type standalone
```

I ask it to run on the address `10.131.111.1` (the WireGuard ip) and on the port `444`. The port `443` will be used for a https server, so I picked 443 + 1.

Since I want an ACME server, I asked to get one.

Step will ask what IP address the clients will use to reach your ca, reply with `10.131.111.1`, because only WireGuard peers and the server should be allowed.

This will prompt a password, put one.

Step will generate a root and intermediate key, as well as an intermediate certificate. We don't want that, since we already generated our own.

Copy these files in `/etc/step-ca/certs`:
- `root_ca.crt`
- `intermediate_ca.crt`

Copy `intermediate_ca_key` in `/etc/step-ca/secrets` folder. I use the key directly, but in a safe environment use a Yubikey, but I don't have one.

## Start the CA/ACME server

Run
```sh
step-ca /etc/step-ca/config/ca.json
```
to start the server. It will ask your password to decrypt the `intermediate_ca_key`. Provide the password.

The server should start, stop it.

We will now create a file containing the password of the `intermediate_ca_key`, since we want to have the ACME server starting when Alpine will boot.

Why put the password inside a file? Well, simply because we can't type the password at boot. Again, in an ideal environment, use a Yubikey.

Create a file at
```
/etc/step-ca/password.txt
```
and place the password inside that file.

`step` should run as the user `step-ca`, so update the permissions on the config folder:
```sh
chown step-ca:step-ca -Rv /etc/step-ca/
```

To verify that everything worked, run:
```sh
step-ca /etc/step-ca/config/ca.json --password-file=/etc/step-ca/password.txt
```

Stop the server again.


## Script to launch

Step already has a service!

- Start: `rc-service step-ca start`
- Stop: `rc-service step-ca stop`

# Use ACME with Caddy

Now let's tell Caddy to get TLS certificates with our ACME server.

Edit the `/etc/caddy/Caddyfile`:
```
# global
{
        # step-ca ACME server
        acme_ca https://10.131.111.1:444/acme/acme/directory
}

intra.philt3r intra.philt3r:80 {
        root * /srv/www
        file_server browse
}
```

Make sure `step-ca` is started, and restart Caddy to make sure everything is good:
```sh
rc-service caddy restart
```

Now we need to tell our system to trust the certificates.

Download the file containing the certificates. It is available at this URL:
```
https://10.131.111.1:444/roots.pem
```

On every device you want to trust your certificates, you will need to download the file on the device, then you will need to tell your operating system to trust it.

- OSX: [Trust the certificate](https://tosbourn.com/getting-os-x-to-trust-self-signed-ssl-certificates/)
- iOS: Download the certificate from the device and [trust it](https://support.apple.com/en-us/HT204477)
- Firefox: Install the certificate on your system and [tell firefox to trust it](https://support.mozilla.org/en-US/kb/setting-certificate-authorities-firefox)
- Ubuntu (but should work for other linux distros): [Trust the certificate](https://ubuntu.com/server/docs/security-trust-store)

# Start on boot

Start `caddy` and `step-ca` on startup with:
```sh
rc-update add step-ca
rc-update add caddy
```

Reboot to make sure everything works.

# Resources

- [https://wiki.alpinelinux.org/wiki/Repositories](https://wiki.alpinelinux.org/wiki/Repositories)
- Awesome guide that helped me a lot: [https://www.apalrd.net/posts/2023/network_acme/](https://www.apalrd.net/posts/2023/network_acme/)
