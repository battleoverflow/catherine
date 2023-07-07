# -*- coding: utf-8 -*-

"""
    Project: Catherine Framework (https://github.com/azazelm3dj3d/catherine)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
"""

import redis, time, sys, os, platform, argparse, colorama
from subprocess import getoutput
from prettytable import PrettyTable

r = redis.Redis()

# Colorama config
colorama.init()
RED = colorama.Fore.RED
RESET = colorama.Fore.RESET

class RedisAnalysis:
    def real_time_render():
        parser = argparse.ArgumentParser()
        parser.add_argument('-s', '--start_server', help="Choose specific keys to investigate in real-time", action='store_true', default=True, required=False)
        args = parser.parse_args()

        def clear():
            if platform.system() == "Windows":
                os.system("cls")
            else:
                os.system("clear")
        
        timer = 0

        if args.start_server:
            print("Example: key1 key2 key3")
            user_keys = input("Enter Search Criteria (seperated by spaces): ")

            key_list = user_keys.split()

            try:
                while True:
                    # 0.1 = 100ms
                    time.sleep(0.1)
                    clear()

                    state_header = PrettyTable(["Keys", "Values"])
                    
                    for key in key_list:
                        state_value = r.mget(key)
                        state_header.add_row([key, state_value])
                    
                    print(state_header.get_string(title="Real Time Redis Data"))

                    timer += 100

                    print("\nTime Elapsed: {0}ms".format(timer))
                    print("\nYou can exit by pressing Ctrl + C")
            except redis.exceptions.ConnectionError:
                print("Unable to connect to Redis")

    def redis_comms():
        whoami = getoutput("whoami")
        print("Connecting to db shell...")
        time.sleep(0.5)

        while True:
            try:
                command = input(f"{whoami}@{RED}redis_analysis{RESET}[🦀 Catherine Framework 🦀]:~$ ")

                if command == "h" or command == "help":
                    print("q, quit      Quit program")
                    print("h, help      Displays help menu")
                    print("k, key       Search for a specific Redis key")
                    print("v, version   Check version of Redis & RediSea")
                    print("c, clear     Clears all data in the terminal")
                    print("d, dump      Dump entire Redis database (keys)")
                    print("df, dumpf    Dump entire Redis database (keys) into a file")
                    print("i, info      Return general information about the Redis instance")
                    print("r, remote    Remotely connect to a Redis instance")
                    print("rt, realtime View Redis data update in real-time")
                elif command == "q" or command == "quit" or command == "exit":
                    print("Disconnecting...")
                    time.sleep(0.2)
                    sys.exit()
                elif command == "v" or command == "version":
                    try:
                        print(f"Redis Version: {r.execute_command('INFO')['redis_version']}")
                    except redis.exceptions.ConnectionError:
                        print("Unable to connect to Redis")
                elif command == "k" or command == "key":
                    key = input("Key: ")
                    
                    try:
                        key_output = r.mget(key)
                        print(f"Key: {key}\nValue: {key_output}")
                    except redis.exceptions.ConnectionError:
                        print("Unable to connect to Redis")
                elif command == "c" or command == "clear":
                    system_info = platform.system()

                    if system_info == 'Windows':
                        os.system("cls")
                    else:
                        os.system("clear")
                elif command == "dump" or command == "d":
                    try:
                        for key in r.scan_iter("*"):
                            print(key)
                    except redis.exceptions.ConnectionError:
                        print("Unable to connect to Redis")
                elif command == "df" or command == "dumpf":
                        try:
                            with open('redis_dump.log', 'w') as f:
                                for key in r.scan_iter("*"):
                                    f.write(str(key) + "\n")
                        
                            print(f"{RED}[+]{RESET} Data successfully dumped!")
                        except redis.exceptions.ConnectionError:
                            print("Unable to connect to Redis")
                elif command == "i" or command == "info":
                    try:
                        redis_data = r.execute_command('CLIENT LIST')
                        redis_data_str = str(redis_data)
                        print(redis_data_str)
                    except redis.exceptions.ConnectionError:
                        print("Unable to connect to Redis")
                elif command == "r" or command == "remote":
                    ip_address = input("IP Address: ")
                    port = input("Port: ")
                    
                    confirm_choice = input("Are you sure you would like to continue (y/n)? ")

                    if confirm_choice == "y":
                        # Connect to the Redis database
                        os.system(f"redis-cli -h {ip_address} -p {port}")
                    elif confirm_choice == "n":
                        print("Exiting...")
                    else:
                        print("Please choose y/n")
                elif command == "rt" or command == "realtime":
                    try:
                        RedisAnalysis().real_time_render()
                    except:
                        print("Unable to connect to Redis")
                else:
                    print("Unrecognized Command\n")
                    print("Available commands:")
                    print("q, quit      Quit program")
                    print("h, help      Displays help menu")
                    print("k, key       Search for a specific Redis key")
                    print("v, version   Check version of Redis & RediSea")
                    print("c, clear     Clears all data in the terminal")
                    print("d, dump      Dump entire Redis database (keys)")
                    print("df, dumpf    Dump entire Redis database (keys) into a file")
                    print("i, info      Return general information about the Redis instance")
                    print("r, remote    Remotely connect to a Redis instance")
                    print("rt, realtime View Redis data update in real-time\n")
            
            except KeyboardInterrupt:
                time.sleep(0.2)
                sys.exit()

if __name__ == '__main__':
    RedisAnalysis.redis_comms()