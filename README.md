# Emotion Mixer from OK

## 🔥 Quick Start
You can use our generator at: [mix.andcool.ru](https://mix.andcool.ru/).

## 📖 API Documentation
The API is available at: `https://mix.andcool.ru/api/`

### 🎭 Emote Generation
```http
GET /api/mix_image.gif?left=<top_part>&right=<bottom_part>
```
**Description:** Generates an emote by combining the upper and lower parts.

**Parameters:**
- `left` — upper part (eyes and eyebrows)
- `right` — lower part (mouth)

**Available values:**
```
grin, flush, he, mad, plead, sad, sg, shock, sl_smile, sleep, smiley, tong, unamus, wink, zany
```
These values can be used for both the upper (`left`) and lower (`right`) parts of the emote.

**Example requests:**
```http
GET /api/mix_image.gif?left=mad&right=smiley
```
```http
GET /api/mix_image.gif?left=wink&right=tong
```

## 🚀 Running

### Building from Source (Rust)
```sh
git clone https://github.com/PepsiCommunity/ok-face-mixer-rs.git
cd ok-face-mixer-rs
cargo build --release
./target/release/ok-face-mixer-api
```

---
**Created by gimura**
