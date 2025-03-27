---
title: API Reference
description: A API reference for mojiji.
---

## `/`

**GET**: `/`

[Discover on browser](https://mojiji.v-sli.me?text=もじじを試してみる&font=light_novel&size=200&color=FF5733)

```bash
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=light_novel&size=200&color=FF5733"
```

### Request Parameters

#### `text` (Required)

- Generates an image with the specified text
- Supports multiple languages and character types

#### `font` (Optional)

Default: `rounded_mplus`

Available Fonts:

| Font Name           | Prefixes                         | Language Support  | Style                          |
| ------------------- | -------------------------------- | ----------------- | ------------------------------ |
| **Noto Sans JP**    | `noto`, `noto_sans`              | Japanese, English | Gothic, Bold                   |
| **Rounded M+ 1m**   | `mplus`, `rounded_mplus`         | Japanese, English | Rounded Gothic, Bold           |
| **Memoir**          | `memoir`                         | Japanese, English | Original font                  |
| **Memoir Round**    | `memoir_round`                   | Japanese, English | Original rounded font          |
| **IPA Mincho**      | `mincho`, `ipa_mincho`           | Japanese, English | Mincho (Traditional serif)     |
| **Jua**             | `jua`, `kr_jua`                  | Korean, English   | Gothic                         |
| **Dot Gothic 16**   | `dot`, `dot_gothic_16`           | Japanese, English | Pixel/Dot font                 |
| **Kaisei Decol**    | `kaisei_decol`                   | Japanese, English | Decorative Mincho, Bold        |
| **Rampart One**     | `rampart`, `rampart_one`         | Japanese, English | Pop style                      |
| **Yuji Boku**       | `yuji_boku`                      | Japanese, English | Brush-like Mincho              |
| **Cherry Bomb One** | `cherry_bomb`, `cherry_bomb_one` | Japanese, English | Pop style                      |
| **Hachi Maru Pop**  | `hachi_maru`, `hachi_maru_pop`   | Japanese, English | Handwritten pop style          |
| **Light Novel Pop** | `light_novel`, `light_novel_pop` | Japanese, English | Light novel-inspired pop style |

#### `size` (Optional)

- Default: `100`
- Range: `1` to `500` pixels
- Specifies the font size of the text

#### `color` (Optional)

- Default: `random`
- Specify color in hexadecimal format **without #**
- Examples:
  - `444444` for dark gray
  - If `#` is included, a random color will be generated

### Advanced Usage Notes

- All parameters except `text` are optional
- Experiment with different combinations
- Ideal for creating quick, customized text images

#### Curl Examples

1. Basic Text Rendering

```bash
# Simple text generation
curl "https://mojiji.v-sli.me?text=もじじを試してみる"
```

2. Specific Font Selection

```bash
# Using Noto Sans JP font
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=noto"

# Using Rounded M+ font
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=mplus"

# Using Memoir font
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=memoir"
```

3. Font Size Variations

```bash
# Small text (size 50)
curl "https://mojiji.v-sli.me?text=もじじを試してみる&size=50"

# Large text (size 300)
curl "https://mojiji.v-sli.me?text=もじじを試してみる&size=300"
```

4. Color Customization

```bash
# Specific color (dark gray)
curl "https://mojiji.v-sli.me?text=もじじを試してみる&color=444444"

# Random color (default behavior)
curl "https://mojiji.v-sli.me?text=もじじを試してみる"
```

5. Combining Multiple Parameters

```bash
# Full customization example
curl "https://mojiji.v-sli.me?text=もじじを試してみる&font=cherry_bomb&size=200&color=FF5733"
```

## `/alive`

**GET**: `/alive`

```bash
# Check if Mojiji service is running
curl "https://mojiji.v-sli.me/alive"
```

### Request Description

Simple health check endpoint to verify the service's operational status.

#### Expected Response

```
mojiji is running now.
```

### Request Parameters

- No parameters required
- Simple GET request to check service availability

#### Use Cases

1. Service Monitoring

```bash
# Include in monitoring scripts
curl "https://mojiji.v-sli.me/alive"
```

2. Pre-Request Connectivity Check

```bash
# Verify service before image generation
if curl -s "https://mojiji.v-sli.me/alive"; then
    echo "Service is ready"
    # Proceed with image generation
fi
```

### Advanced Usage Notes

- Lightweight endpoint with minimal computational overhead
- No authentication required
- Instant response
- Useful for automated health checks and system monitoring
