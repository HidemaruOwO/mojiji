<!-- YOU SHOULD RUN THIS COMMAND IF YOU USING VIM -->
<!-- :%s;HidemaruOwO/mojiji;USERNAME/REPONAME;g -->

# mojiji 🎨

A cute way to turn text into an image that looks like an emoji 🎨

> [!NOTE]
> This repository is development now!!

## 🚀 Features

- Simple REST API
- Text to image with good-looks fonts

## 📚 API Reference

### **GET**: `/`

```bash
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=mplus&size=100&color=444444"
```

**parameters**

- `text`: Specifies the text to be displayed in the generated image.
- `font` (optional): Specifies the font to be used. (default: `rounded_mplus`)

<details>
<summary>Available Fonts</summary>

- **Noto Sans JP**

  - `Prefix`: `noto` `noto_sans`
  - `Support`: 日本語（ひらがな、カタカナ、漢字）、英数字
  - `Style`: ゴシック体、太字

- **Rounded M+ 1m**

  - `Prefix`: `mplus` `rounded_mplus`
  - `Support`: 日本語、英数字
  - `Style`: 丸ゴシック体、太字

- **Memoir**

  - `Prefix`: `memoir`
  - `Support`: 日本語、英数字
  - `Style`: オリジナルフォント

- **Memoir Round**

  - `Prefix`: `memoir_round`
  - `Support`: 日本語、英数字
  - `Style`: オリジナル丸フォント

- **IPAフォント明朝体**

  - `Prefix`: `mincho` `ipa_mincho`
  - `Support`: 日本語、英数字
  - `Style`: 明朝体

- **Jua**

  - `Prefix`: `jua` `kr_jua`
  - `Support`: 韓国語（ハングル）、英数字
  - `Style`: ゴシック体

- **ドットゴシック16**

  - `Prefix`: `dot` `dot_gothic_16`
  - `Support`: 日本語、英数字
  - `Style`: ドット（ピクセル）フォント

- **解星 Decol**

  - `Prefix`: `kaisei_decol`
  - `Support`: 日本語、英数字
  - `Style`: 装飾的な明朝体、太字

- **Rampart One**

  - `Prefix`: `rampart` `rampart_one`
  - `Support`: 日本語、英数字
  - `Style`: ポップ体

- **遊明朝**

  - `Prefix`: `yuji_boku`
  - `Support`: 日本語、英数字
  - `Style`: 毛筆風明朝体

- **Cherry Bomb One**

  - `Prefix`: `cherry_bomb` `cherry_bomb_one`
  - `Support`: 日本語、英数字
  - `Style`: ポップ体

- **はちまるポップ**

  - `Prefix`: `hachi_maru` `hachi_maru_pop`
  - `Support`: 日本語、英数字
  - `Style`: 手書き風ポップ体

- **ライトノベルポップ**
  - `Prefix`: `light_novel` `light_novel_pop`
  - `Support`: 日本語、英数字
  - `Style`: ライトノベル風ポップ体

</details>

- `size` (optional): Specifies the font size in pixels. (default: `100`, range: `1~500`)
- `color` (optional): Specifies the text color in **hexadecimal format without #** (e.g., 444444 for dark gray). **If `#` is included, a random color will be used instead.** (default: `random`)

### **GET**: `/alive`

```bash
curl https://mojiji.v-sli.me/alive

# mojiji is running now.
```

## 🛠 Installation

<!-- ```bash -->
<!-- brew install my-repository -->
<!-- ``` -->

<!-- | distribution         | command                         | -->
<!-- | -------------------- | ------------------------------- | -->
<!-- | Ubuntu               | `apt-get install <package>`     | -->
<!-- | Debian               | `apt install <package>`         | -->
<!-- | Arch Linux           | `pacman -S <package>`           | -->
<!-- | Fedora               | `dnf install <package>`         | -->
<!-- | CentOS               | `yum install <package>`         | -->
<!-- | openSUSE             | `zypper install <package>`      | -->
<!-- | Alpine Linux         | `apk add <package>`             | -->
<!-- | Gentoo               | `emerge <package>`              | -->
<!-- | NixOS                | `nix-env -iA nixpkgs.<package>` | -->
<!-- | macOS                | `brew install <package>`        | -->
<!-- | Windows (winget)     | `winget install <package>`      | -->
<!-- | Windows (Chocolatey) | `choco install <package>`       | -->

### 🏗 Install from Source

```sh
git clone https://github.com/HidemaruOwO/mojiji.git
cd mojiji

cargo build --release

install -Dm0755 -t "/usr/local/bin/" "target/release/mojiji"
````

## 🎯 Usage

```bash
# running local host 3000
mojiji
```

- To run the service automatically, you can set it up with `systemd`:

```sh
# run as a service.
sudo systemctl enable --now mojiji.service

# if u alerdy using interception.
sudo systemctl restart mojiji.service
```

<details>
<summary>mojiji.service file</summary>

```service
[Unit]
Description=Mojiji Web API
After=network.target

[Service]
#User=user
#WorkingDirectory=/home/user/app
ExecStart=/usr/local/bin/mojiji
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
- [@emoji ボットの使い方 (ja)](https://nzws.notion.site/emoji-ja-23238c8c0946453c8acdadc78fc9acb9)
