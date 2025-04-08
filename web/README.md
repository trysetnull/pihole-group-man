# Integrating with Pi-Hole

The following explains how to host this website alongside the Pi-Hole admin web interface.
It's largely based on the information
from a [post by i6nfinity](https://discourse.pi-hole.net/t/host-website-alongside-pi-hole-interface/31832).

We will be hosting a simple page with two buttons to enable/disable DNS resolution for a client (fire-tv).

## Prerequisites

Create the following in the Pi-Hole admin website:

1. Navigate to the `Groups` tab, create a group named 'Unresolved'
2. In the `Clients` tab Select the client you wish to block (via MAC address) and assign it *unique* comment;
   We'll use this to identify the client when using `pihole-group-man`
3. Finally, under the tab `Domains` create the following:
   |Domain/Regex|Type|Status|Comment|Group assignment|
   |------------|----|------|-------|----------------|
   | .*         | Regex blacklist | Enabled | Block all | Unresolved |

## Create a new folder for the website and copy content

Create the folder `/var/www/html/fire-tv` with the appropriate permissions:

```shell
sudo mkdir -p /var/www/html/fire-tv
sudo chown www-data:www-data /var/www/html/fire-tv
```

Copy the files `index.html` and `execute.php` into this directory.

## Enable virtual hosts with lighttpd

```shell
sudo lighty-enable-mod simple-vhost
sudo service lighttpd force-reload
```

## Create a lighttpd configuration

An example is provided in the file `20-fire-tv-external.conf`
Copy this to `/etc/lighttpd/sites-available`, check that it is valid, then create a symbolic link:

```shell
lighttpd -t -f /etc/lighttpd/sites-available/20-fire-tv-external.conf

# If this passes go ahead and create a symbolic link.
cd /etc/lighttpd/sites-enabled
sudo ln -s ../sites-available/20-fire-tv-external.conf ./
```

## Restart lighttpd and configure local DNS record in Pi-Hole

Reload lighttpd to server the new site:

```shell
sudo systemctl restart lighttpd
```

In the Pi-Hole admin web interface navigate to `Local DNS > DNS Records` and add the appropriate entry:

| Domain: | IP Address:   |
|---------|---------------|
| fire.tv | 192.168.0.94* |

\* Default Pi-Hole address, adjust accordingly.

Navigate to <http://fire.tv> whenever you'd like to enable/disable DNS for the fire-tv.
