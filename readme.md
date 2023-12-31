# 杂鱼体质健康得分计算器

[![Build | Deploy](https://github.com/hduhelp/college-student-pe-score/actions/workflows/deploy.yml/badge.svg)](https://github.com/hduhelp/college-student-pe-score/actions/workflows/deploy.yml)
[![Format](https://github.com/hduhelp/college-student-pe-score/actions/workflows/format.yml/badge.svg)](https://github.com/hduhelp/college-student-pe-score/actions/workflows/format.yml)

杂鱼 ❤ 杂鱼~

## 食用の方式

### Docker:

```shell
$ docker build . -t college-student-pe-score
$ docker run --rm -d -p 8080:8080 --name pe-score-cal college-student-pe-score
```

## Api

### `GET` /ping

> 测试连通性

请求: 无

响应: Pong


### `POST` /

> 计算体育得分

请求

| 字段     | 说明                          |
| -------- | ----------------------------- |
| gender   | 性别 0: 男，1: 女             |
| grade    | 年级 0: 大一大二，1: 大三大四 |
| height   | 身高 (单位: m)                |
| weight   | 体重 (单位: kg)               |
| vc       | 肺活量 (单位: 毫升)           |
| r50m     | 50m (单位: 秒)                |
| sff      | 坐位体前屈 (单位: cm)         |
| slj      | 立定跳远 (单位: cm)           |
| pull_up  | 引体向上                      |
| sit_up   | 仰卧起坐                      |
| long_run | 800m/1000m (单位: 分钟)       |

Example:

```json
{
  "gender": 0,
  "grade": 0,
  "height": 1.68,
  "weight": 56.5,
  "vc": 4800,
  "r50m": 7.43,
  "sff": 12,
  "slj": 256,
  "pull_up": 12,
  "sit_up": null,
  "long_run": 3.43
}
```

响应

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

Example:

```json
{
  "err_msg": null,
  "bmi": 100,
  "vc": 90,
  "r50m": 76,
  "sff": 70,
  "slj": 85,
  "pull_up": 70,
  "sit_up": null,
  "long_run": 78,
  "total": 81.80000000000001
}
```
