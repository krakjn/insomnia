package main

import (
	"bufio"
	"fmt"
	"os"
	"syscall"

	"github.com/godbus/dbus/v5"
)

func main() {
	conn, err := dbus.SystemBus()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to connect to system bus: %v\n", err)
		return
	}

	// Get a proxy object
	obj := conn.Object("org.freedesktop.login1", dbus.ObjectPath("/org/freedesktop/login1"))

	what := "sleep"
	who := "insomnia"
	why := "Preventing system sleep for operational reasons"
	mode := "block" // Mode can be 'block' or 'delay'

	// Call the Inhibit method
	var fd dbus.UnixFD
	err = obj.Call("org.freedesktop.login1.Manager.Inhibit", 0, what, who, why, mode).Store(&fd)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to call Inhibit: %v\n", err)
		return
	}

	fmt.Println("Sleep inhibited. Press Enter to release inhibition.")
	reader := bufio.NewReader(os.Stdin)
	_, _ = reader.ReadString('\n') // Wait for user input

	// Release the inhibition by closing the file descriptor using syscall
	syscall.Close(int(fd))
	fmt.Println("Inhibition released.")
}

