# 杂鱼体质健康得分计算器

杂鱼~❤ 杂鱼~ WIP

## 食用の方式

### Api

#### 请求

| 字段     | 说明 |
| -------- | ---- |
| flag     |      |
| height   |      |
| weight   |      |
| vc       |      |
| r50m     |      |
| sff      |      |
| slj      |      |
| pull_up  |      |
| sit_up   |      |
| long_run |      |

#### Example:

```json
{
  "flag": 11,
  "height": 1.83,
  "weight": 74.2,
  "vc": 4800,
  "r50m": 7.5,
  "sff": 0.0,
  "slj": 0.0,
  "pull_up": 12,
  "sit_up": null,
  "long_run": 3.45
}
```

#### 响应

| 字段     | 说明 |
| -------- | ---- |
| bmi      |      |
| vc       |      |
| r50m     |      |
| sff      |      |
| slj      |      |
| pull_up  |      |
| sit_up   |      |
| long_run |      |
| total    |      |

#### Example:

```json
{
  "bmi": 27.0,
  "vc": 1.0,
  "r50m": 1.0,
  "sff": 0.0,
  "slj": 0.0,
  "pull_up": 1.0,
  "sit_up": null,
  "long_run": 1.0,
  "total": 12.0
}
```

### CLI (todo)

```bash

```
