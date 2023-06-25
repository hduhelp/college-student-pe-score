# 杂鱼体质健康得分计算器

杂鱼❤杂鱼~

## 食用の方式

### Api

#### 请求

| 字段     | 说明                          |
| -------- | ----------------------------- |
| gender   | 性别 0: 男，1: 女             |
| grade    | 年级 0: 大一大二，1: 大三大四 |
| height   | 身高 (单位: m)                |
| weight   | 体重 (单位: kg)               |
| vc       | 肺活量                        |
| r50m     | 50m (单位: 秒)                |
| sff      | 坐位体前屈 (单位: cm)         |
| slj      | 立定跳远 (单位: cm)           |
| pull_up  | 引体向上                      |
| sit_up   | 仰卧起坐                      |
| long_run | 800m/1000m (单位: 分钟)       |

#### Example:

```json
{
  "gender": 0,
  "grade": 0,
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

| 字段     | 说明            |
| -------- | --------------- |
| err_msg  | 错误信息        |
| bmi      | BMI 指数        |
| vc       | 肺活量得分      |
| r50m     | 50m 得分        |
| sff      | 坐位体前屈得分  |
| slj      | 立定跳远得分    |
| pull_up  | 引体向上得分    |
| sit_up   | 仰卧起坐得分    |
| long_run | 800m/1000m 得分 |
| total    | 总分            |

#### Example:

```json
{
  "err_msg": null,
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
