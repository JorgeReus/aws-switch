package main
import (
	"fmt"
  "github.com/bigkevmcd/go-configparser"
  "github.com/manifoldco/promptui"
  "io/ioutil"
  "flag"
  "os/user"
)
func main() {
  user, userErr := user.Current()
  if userErr != nil {
    panic(userErr)
  }
  credsPtr := flag.String("creds", fmt.Sprintf("%s/.aws/credentials", user.HomeDir), "AWS credentials file location")
  tmpfilePtr := flag.String("tmp", "/tmp/awsprofileswitcher.profile", "Temporary file location")
  flag.Parse()
  p, err := configparser.NewConfigParserFromFile(*credsPtr)
  if err != nil {
    panic(err)
  }
  sections := p.Sections()
  prompt := promptui.Select{
    Label: "Select AWS profile",
    Items: sections,
  }
  _, result, err := prompt.Run()
  if err != nil {
    fmt.Printf("Prompt failed %v\n", err)
    return
  }
  data := []byte(fmt.Sprintf("export AWS_PROFILE=%s", result))
  if err := ioutil.WriteFile(*tmpfilePtr, data, 0644); err != nil {
    panic(err)
  }
}
