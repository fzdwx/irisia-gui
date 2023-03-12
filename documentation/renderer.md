## 渲染器
渲染器是将窗口后端和前端连接起来的协调者。

它的简要结构为：
```
渲染器<T>{
    根节点（组件管理器<T>）,
    画布,
}
```

它对前端负责的部分有：

1. 提供画布
2. 将窗口事件发送给根节点
3. 定期触发重绘检查事件