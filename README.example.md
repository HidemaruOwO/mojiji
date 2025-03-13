<!-- YOU SHOULD RUN THIS COMMAND IF YOU USING VIM -->
<!-- :%s;HidemaruOwO/MyRepository;USERNAME/REPONAME;g -->

# MyRepository 📚

A template that gathers the minimal structure of my repository.

## 🚀 Features

- SIMPLE REST API
- COOK APPLE PIE

## 🛠 Installation

```bash
brew install my-repository
```

### 🏗 Install from Source

```sh
git clone https://github.com/HidemaruOwO/MyRepository.git
cd MyRepository

make -j8

install -Dm0755 -t "/usr/local/bin/" "dist/builded-binary"
```

- Arch Linux

```sh
git clone https://github.com/HidemaruOwO/MyRepository.git
cd MyRepository

makepkg -si
```

## 🎯 Usage

```bash
# running local host 3000
MyRepository
```

- To run the service automatically, you can set it up with `systemd`:

```sh
# run as a service.
sudo systemctl enable --now MyRepository.service

# if u alerdy using interception.
sudo systemctl restart MyRepository.service
```

<details>
<summary>MyRepository.service file</summary>

```service
[Unit]
Description=My Repository Web API
After=network.target

[Service]
#User=user
#WorkingDirectory=/home/user/app
ExecStart=/usr/local/bin/MyRepository
Restart=always
StandardOutput=journal
StandardError=journal
Environment=PATH=/usr/bin:/usr/local/bin

[Install]
WantedBy=multi-user.target
```

</details>

## 🌍 For contributer

By contributing to this project, you agree to the following terms:

1. **You grant a license**: You grant the project owner a perpetual, worldwide, non-exclusive, royalty-free, irrevocable license to use, modify, distribute, and sublicense your contributions under the **Apache License 2.0**.
2. **You retain ownership**: You still own the copyright of your contribution, but you waive any claims against the project related to your contribution.
3. **No additional patent rights**: You **do not** grant additional patent rights beyond what is covered by Apache 2.0.
4. **Your contributions are original**: You confirm that your contributions do not violate any third-party rights.

By submitting a pull request, you agree to these terms.

## 📜 License

<div align="left" style="flex: inline" >
<a href="https://www.apache.org/licenses/LICENSE-2.0" >
<img src="https://img.shields.io/badge/License-Apache%20License%202.0-blue.svg" alt="Apache License 2.0"
</a>
<a href="https://github.com/MakeNowJust/sushi-ware" >
<img src="https://img.shields.io/badge/License-SUSHI--WARE%20%F0%9F%8D%A3-blue.svg" alt="SUSHI-WARE LICENSE"
</a>
</div>

This project is dual-licensed under [Apache License 2.0](licenses/APACHE-2.0.txt) and [SUSHI-WARE LICENSE](licenses/SUSHI-WARE.txt).

A reference to the latest license should be used, even if the attached license is outdated of major versions.

## 🤝 Reference

This repository was created using the [MicroRepository](https://github.com/HidemaruOwO/MicroRepository) template.

- [HidemaruOwO/MicroRepository](https://github.com/HidemaruOwO/MicroRepository)
