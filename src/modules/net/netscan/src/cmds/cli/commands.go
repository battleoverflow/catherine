/*
    Project: Catherine Framework (https://github.com/battleoverflow/catherine)
    Author: battleoverflow (https://github.com/battleoverflow)
    License: BSD 2-Clause
*/

package commands

import (
	"fmt"
	"log"
	"net"
	"os"

	"github.com/urfave/cli"
)

func NetCommands() {
		controller := cli.NewApp()

		controller.Usage = "Go application created to gather network information about a specific host"

		defaultValues := []cli.Flag {
			cli.StringFlag {
				// Default values for commands
				Name: "host",
				Value: "google.com", // Testing only
			},
		}

		// Command List
		controller.Commands = []cli.Command {
		{
			Name: "ns",
			Usage: "Returns nameserver information about a particular host",
			Flags: defaultValues,

			// ns command activated
			Action: func(cmd *cli.Context) error {
				ns, err := net.LookupNS(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				// Nameservers
				fmt.Println("Nameserver(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(ns); i++ {
						fmt.Println("[*]", ns[i].Host)
					}
				}

				fmt.Println("==============================")

				return nil
			},
		},
		{
			Name: "ip",
			Usage: "Returns IP address(es) associated with a particular host",
			Flags:defaultValues,

			// ip command activated
			Action: func(cmd *cli.Context) error {
				ip, err := net.LookupIP(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				// IP Address(es)
				fmt.Println("IP Address(es):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(ip); i++ {
						fmt.Println("[*]", ip[i])
					}
				}

				fmt.Println("==============================")

				return nil
			},
		},
		{
			Name: "cname",
			Usage: "Returns CNAME record(s) about a particular host",
			Flags:defaultValues,

			// cname command activated
			Action: func(cmd *cli.Context) error {
				cname, err := net.LookupCNAME(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				// CNAME
				fmt.Println("CNAME Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					fmt.Println("[*]", cname)
				}

				fmt.Println("==============================")
				
				return nil
			},
		},
		{
			Name: "mx",
			Usage: "Returns MX record(s) about a particular host",
			Flags:defaultValues,

			// mx command activated
			Action: func(cmd *cli.Context) error {
				mx, err := net.LookupMX(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				// MX Record
				fmt.Println("MX Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(mx); i++ {
						fmt.Println("Host:", mx[i].Host)
						fmt.Println("Priority:", mx[i].Pref)
					}
				}

				fmt.Println("==============================")

				return nil
			},
		},
		{
			Name: "txt",
			Usage: "Returns TXT record(s) about a particular host",
			Flags:defaultValues,

			// txt command activated
			Action: func(cmd *cli.Context) error {
				txt, err := net.LookupTXT(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				// TXT Record
				fmt.Println("TXT Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(txt); i++ {
						fmt.Println("[*]", txt[i])
					}
				}

				fmt.Println("==============================")

				return nil
			},
		},
		{
			Name: "all",
			Usage: "Returns all information about a particular host",
			Flags:defaultValues,

			// all command activated
			Action: func(cmd *cli.Context) error {
				ns, err := net.LookupNS(cmd.String("host"))
				ip, err := net.LookupIP(cmd.String("host"))
				cname, err := net.LookupCNAME(cmd.String("host"))
				mx, err := net.LookupMX(cmd.String("host"))
				txt, err := net.LookupTXT(cmd.String("host"))

				// if err != nil {
				// 	return err
				// }

				fmt.Println("\n")

				// Nameservers
				fmt.Println("Nameserver(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(ns); i++ {
						fmt.Println("[*]", ns[i].Host)
					}
				}

				fmt.Println("\n")

				// IP Address(es)
				fmt.Println("IP Address(es):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(ip); i++ {
						fmt.Println("[*]", ip[i])
					}
				}

				fmt.Println("\n")

				// CNAME
				fmt.Println("CNAME Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					fmt.Println("[*]", cname)
				}
				
				fmt.Println("\n")

				// MX Record
				fmt.Println("MX Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(mx); i++ {
						fmt.Println("Host:", mx[i].Host)
						fmt.Println("Priority:", mx[i].Pref)
					}
				}

				fmt.Println("\n")

				// TXT Record
				fmt.Println("TXT Record(s):")
				fmt.Println("==============================")

				// Logs results to terminal
				if err != nil {
					fmt.Println("[*] Unable to gather information. Are you connected to the internet?\n")

					fmt.Println("Error Encountered:", err)
				} else {
					for i := 0; i < len(txt); i++ {
						fmt.Println("[*]", txt[i])
					}
				}

				return nil
			},
		},
	}

	// Initializing Engine...
	err := controller.Run(os.Args)

	if err != nil {
		log.Fatal(err)
	}
}