# **A**utomatic **V**ideo **D**ownloader with **R**SS and **I**FTTT (AVDRI)

- 你是否还在困扰于B站收藏夹中的“已失效视频”？
- 你是否纠结如何将B站你想收藏的视频自动下载到电脑上？

受到@[DIYgod](https://github.com/DIYgod)的启发，正好也想自己尝试写一个Rust的小玩具，就做了一个这样的东西。
AVDRI是一个自动下载B站你收藏/投币的视频的一整套工具，流程为：
```
投币/收藏到指定收藏夹 -> 触发RSS更新 -> IFTTT.com检测RSS更新并通过POST命令发送给我们实现的Rust服务器 -> Rust服务器调用you-get工具下载视频
```

再次感谢@[DIYgod](https://github.com/DIYgod)维护的RSSHub带来的方便的RSS体验，与他的“[优雅地下载我的B站投币视频](https://diygod.me/download-webhook/)”教程，本仓库是对他教程的细化与扩展，本人只是做了一些微小的工作 o-o.
如果大家觉得不错的话请多给RSSHub支持！

还要感谢的@[Mort Yao](https://github.com/soimort)大佬的“[you-get](https://github.com/soimort/you-get)”项目让我们能很方便的下载视频网站上的视频。

## 用到的工具

- 本仓库rss-downloader-rs中的工具
- RSShub
- IFTTT.com
- 一个有域名绑定/固定IP的服务器
- Frp（如果用户本机没有固定IP，而且需要将视频下载到本地而非服务器上）

## 安装与使用

以下为本人的使用方法，不同用户可能因为网络情况（是否有固定IP）和需求情况（下载到本地/服务器/NAS等）步骤有所不同。

### 1. 监控RSS更新
```c
投币/收藏到指定收藏夹 -> 触发RSS更新 //本节为这一步
-> IFTTT.com检测RSS更新并通过POST命令发送给我们实现的Rust服务器 -> Rust服务器调用you-get工具下载视频
```
使用RSSHub服务中[订阅Bilibili的RSS路由](https://docs.rsshub.app/social-media.html#bilibili)的方式获取自己所希望触发视频下载的动作的RSS地址，比较常用的有某用户投稿、某用户默认收藏夹、某用户非默认收藏夹（如果使用RSSHub的路由而非自己搭建的话，应该是获取不到未公开收藏夹的）和某用户投币视频，那么下面的教程就以通过收藏到我的某个专供下载的收藏夹为例来说明。

我的收藏夹地址为
```
https://space.bilibili.com/922919/favlist?fid=919265919
```
对应RSSHub路由为
```
https://rsshub.app/bilibili/fav/922919/919265919
```

准备好了之后可以在浏览器中打开RSSHub路由（截止发稿时访问仍需要梯子），查看一下自己的路由是否存在。

### 2. IFTTT配置
```c
投币/收藏到指定收藏夹 -> 
触发RSS更新 -> IFTTT.com检测RSS更新并通过POST命令发送给我们实现的Rust服务器 //本节为这一步
-> Rust服务器调用you-get工具下载视频
```

### 3. `frp` 配置（非必需）
![Network structure](./meta/Ntwk_structure.png)
### Attention
Windows build 前要安装Visual C++ build tools
## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Roadmap


## Acknowledgment
https://diygod.me/download-webhook/

## References
[【Rust 每周一库】hyper - 底层 http 库](https://www.chainnews.com/articles/617213876836.htm)

## License
[MIT](https://choosealicense.com/licenses/mit/)