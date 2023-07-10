# Dot32 intro

The port of my love2d intro into rust. 

Press `ctrl`+`r` on windows or `cmd`+`r` on mac to replay the animation. (This is currently dissabled, but it can be reenabled by setting RESTARTABLE in lib.rs to true)

<img width="912" alt="Screen Shot 2022-05-15 at 13 48 22" src="https://user-images.githubusercontent.com/61964090/168459582-38e43c84-8312-462d-8010-85e50251589c.png">

![image of intro animation](https://user-images.githubusercontent.com/61964090/168785042-728b8934-35aa-4af1-9c49-8634f00d8ce3.gif)

Installation of the intro is as simple as 
```toml
[dependencies]
dot32_intro = { git = "https://github.com/Dot32IsCool/dot32-intro-rs", rev = "8bc8bd5"}
```
Make sure to include the git rev of the version you're using, see version table below.

Add `use dot32_intro::*;` to the top of your file and add the `Intro` plugin to your bevy app. The order in which plugins/systems are added appears to matter, so play around with that. <br>

Lastly, add a folder called fonts to your assets folder, with PT_Sans inside if you want the text to show (which you should, what's the point otherwise)

I've no idea why you might want to use my intro library but maybe this could be useful for anyone wishing to make their own intro library 🗿

|Bevy version|dot32_intro version|dot32_intro git rev 
|---|---|---|
|0.11|0.6|`8bc8bd5`
|0.10|0.5|`a704742`
|0.9|0.4|`49de846`
|0.8|0.3.1|`02551fc`
|0.8|0.2|`2e10211`
|0.7|0.1|`10d737d`
