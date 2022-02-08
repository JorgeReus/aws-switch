# AWS profile switcher

<div id="top"></div>
<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="./.md/aws-switch.gif" />
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#installation">Instalation</a>
      <ul>
        <li><a href="#static-releases">Static</a></li>
        <li><a href="#from-source">From source</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



## About

Opinioned CLI for switching aws profiles.
God forbid selecting your `AWS_PROFILE` via the env variable manually, it gets disastrous when you have dozens of roles.
So this simple tool lets you select a profile in a fuzzy finder window, picture the previous dozens profiles I talked about, you have to find the profile 1 in one of the dev accounts, with this, just type `profile 1 dev` and *voilà* reduce your search space to a minimum!.

This tool reads both `~/.aws/config` && `~/.aws/credentials` and also has a very opinionated wait of displaying properties, please take a look at [aws-sso-creds](https://github.com/JorgeReus/aws-sso-creds) for a more complete solution regarding `AWS SSO` CLI configuration.

### Built With 
* [Rust](https://www.rust-lang.org/)
* [Skim](https://github.com/lotabout/skim)


<!-- GETTING STARTED -->
## Installation
> :warning: **This tool only works on Linux/MacOS**: Waiting till [issue](https://github.com/lotabout/skim/issues/293) is closed to support windows. If you want this on windows please take a look at  [aws-sso-creds](https://github.com/JorgeReus/aws-sso-creds)
### Static Releases
Download the binary based on your OS in [The releases section](https://github.com/JorgeReus/aws-switch/releases)
### From source
#### Prerequisites
- rustc 1.58.0


<!-- USAGE EXAMPLES -->
## Usage
```
aws-switch
```
Yup, as simple as that, you can create a neat alias for your profile like this `alias aps='eval "export AWS_PROFILE=$(aws-switch)"'`

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Jorge Reus - [LinkedIn](www.linkedin.com/in/JorgeGReus)

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments
* [TLDR Legal](https://tldrlegal.com/)
* [Best README Template](https://github.com/othneildrew/Best-README-Template)
