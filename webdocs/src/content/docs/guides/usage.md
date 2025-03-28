---
title: Quick Start
description: A usage as a quick start guide for mojiji.
---

`mojiji` is a simple RESTful API that generates a logo based on the provided text.  
It can be used in various ways, such as embedding it in a GitHub README, using it as an OGP image for microblogs, or any other application depending on the user's needs.

## Quick Start

- **Purpose**: Generate text images instantly
- **How to Use**: Send a web request with your text

## Basic Usage Examples

### Default Image

[Discover on browser](https://mojiji.v-sli.me?text=HelloWorld)

```bash
# Generate image with default settings
curl "https://mojiji.v-sli.me?text=HelloWorld"
```

### Custom Font and Size

[Discover on browser](https://mojiji.v-sli.me?text=HelloWorld&font=cherry_bomb&size=200)

```bash
# Use cherry bomb one font, larger size
curl "https://mojiji.v-sli.me?text=HelloWorld&font=cherry_bomb&size=200"
```

### Specific Color

[Discover on browser](https://mojiji.v-sli.me?text=HelloWorld&color=FF5733)

```bash
# light red color
curl "https://mojiji.v-sli.me?text=HelloWorld&color=FF5733"
```

## Quick Tips

- Text can be in Japanese, English, or other characters
- Choose from many different fonts. If you need more, please let me know on [github issues](https://github.com/HidemaruOwO/mojiji/issues/new).
- Adjust size from 1 to 500 pixels
- Set custom colors or get random colors
