"""
    Project: Catherine (https://github.com/CatherineFramework)
    Module by: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
"""

import requests, colorama, time, sys
from urllib.parse import urlparse, urljoin
from bs4 import BeautifulSoup

# Colorama config
colorama.init()
GREEN = colorama.Fore.GREEN
RED = colorama.Fore.RED
RESET = colorama.Fore.RESET

# Creates global links for URLs
internal_links = set()
external_links = set()

set_host = sys.argv[1] # str
set_batch = 10

class Parser:
    """
    Parses a URL, locating all associated links, then seperating into internal and external.
    """

    def host_validation(self, host):
        """
        Validates URL
        """

        parse_links = urlparse(host)
        host_scheme = bool(parse_links.scheme)
        host_netloc = bool(parse_links.netloc)

        return host_scheme and host_netloc

    def link_collector(self, host):
        """
        Collects URL links
        """

        # Sets new URL object for collecting a pool of links
        hosts = set()
        website = urlparse(host).netloc

        try:
            soup = BeautifulSoup(requests.get(host).content, "html.parser")
        except requests.exceptions.ConnectionError:
            print(f"Unable to secure a connection for {set_host}")

        for anchor in soup.findAll("a"):
            href = anchor.attrs.get("href")

            if href == "" or href is None or href == "#":
                continue

            href = urljoin(host, href)
            parse_host = urlparse(href)

            # Sets HTTP(s) fragments and scheme
            href = f"{parse_host.scheme}://{parse_host.netloc}{parse_host.path}"

            if not P.host_validation(href):
                continue

            if href in internal_links:
                continue

            if website not in href:
                if href not in external_links:
                    print(f"{GREEN}[+]{RESET}" + f" External Link: {href}")
                    external_links.add(href)
                
                continue

            print(f"{GREEN}[+]{RESET}" + f" Internal Link: {href}")

            # Add all links to the hosts set()
            hosts.add(href)
            internal_links.add(href)
            
        return hosts

    check_max = 0

    def parse_links(self, host, batch_num=int(set_batch)):
        global check_max

        # Increments variable for every link found
        Parser.check_max += 1
        
        links = P.link_collector(host)

        # Parses links up to the max integer
        for link in links:
            if Parser.check_max > batch_num:
                break
            
            P.parse_links(link, batch_num=batch_num)

if __name__ == '__main__':
    P = Parser()

    try:
        print("Validating host...")
        time.sleep(1)

        if P.host_validation(set_host):
            print("Valid host submitted\n")
            P.parse_links(set_host)
        else:
            print("Host is not valid")
            sys.exit()
            
    except KeyboardInterrupt:
        sys.exit()

    print(f"\n{GREEN}[+] {RESET}" + f"Number of Internal Links: {len(internal_links)}")
    print(f"{GREEN}[+] {RESET}" + f"Number of External Links: {len(external_links)}")
    print(f"{GREEN}[+] {RESET}" + f"Total: {len(external_links) + len(internal_links)}")