package main

import (
	"flag"
	"fmt"
	"github.com/bigkevmcd/go-configparser"
	"github.com/manifoldco/promptui"
	"io/ioutil"
	"os/user"
	"runtime"
)

func checkError(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	user, err := user.Current()
	checkError(err)
	credsPtr := flag.String("creds", fmt.Sprintf("%s/.aws/credentials", user.HomeDir), "AWS credentials file location")

	var prefix string

	var tmpfilePtr *string
	if runtime.GOOS == "windows" {
		prefix = "set"
		tmpfilePtr = flag.String("tmp", "/AppData/Local/Temp/awsprofileswitcher.bat", "Temporary file location in windows")
	} else {
		prefix = "export"
		tmpfilePtr = flag.String("tmp", "/tmp/awsprofileswitcher.profile", "Temporary file location in linux")
	}

	flag.Parse()

	// Parse the aws credentials
	p, err := configparser.NewConfigParserFromFile(*credsPtr)
	checkError(err)
	sections := p.Sections()

	// Display the menu
	prompt := promptui.Select{
		Label: "Select AWS profile",
		Items: sections,
	}
	_, result, err := prompt.Run()
	checkError(err)

	// Return the selected option
	data := []byte(fmt.Sprintf("%s AWS_PROFILE=%s", prefix, result))
	err = ioutil.WriteFile(*tmpfilePtr, data, 0755)
	checkError(err)
}
