### 简介

检查`m3u8`和`m3u`格式直播源是否有效。输出文件`valid-iptv.m3u8`和`valid-iptv2.m3u8`。可以使用`vlc`播放器打开文件播放。

-----

### playlists来源
- [IPTV](https://github.com/Free-TV/IPTV.git)
- [Tvlist-awesome-m3u-m3u8](https://github.com/imDazui/Tvlist-awesome-m3u-m3u8)

-----

### 使用方法
```
A tool to extract valid links for m3u8 or m3u file.

Usage: m3u8-checker [OPTIONS]

Options:
  -i, --input-dir <INPUT_DIR>      Input directory [default: .]
  -o, --output-file <OUTPUT_FILE>  Output file [default: valid-iptv.m3u8]
  -t, --timeout <TIMEOUT>          HTTP request timeout [default: 10]
  -h, --help                       Print help
  -V, --version                    Print version
```

