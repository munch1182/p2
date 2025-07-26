# tarui+vue+rust

## vue

1. [vue](https://vuejs.org/guide/introduction.html)
2. [tailwindcss](https://tailwindcss.com/docs/installation/using-vite)
3. [shadcn/vue](https://www.shadcn-vue.com/docs/introduction.html)
4. [tauri](https://tauri.app/plugin/)
5. [crates](https://crates.io/crates)

### 修改 shadcn 的样式配置

让 shadcn 使用 tailwind 变量

1. 定义变量

```css
/* index.css */
:root {
  --search-height: 12.5rem;
}
```

2. 配置变量

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      height: {
        search: "var(--search-height)",
      },
    },
  },
};
```

3. 使用变量

```vue
<div class="h-search"></div>
```

## tauri

```json
// tauri.conf.json
{
  "app": {
    "window": {
      "decorations": false, // 去掉默认标题栏
      "center": true, // 窗口居中
      "alwaysOnTop": true, // 窗口置顶
      "skipTaskbar": true, // 隐藏任务栏图标
      "backgroundColor": "#00000000", // 透明背景
      "transparent": true // 透明窗口 // 比如界面圆角时，如果设备较差或者启动较长，会先显示窗口的直角，再切换的页面的圆角，所以需要透明窗口
    }
  }
}
```
