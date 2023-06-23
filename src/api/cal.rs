use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentPEInfo {
    /// 性别 0: 男 1: 女
    gender: i32,

    /// 年级 0: 大一大二 1: 大三大四
    grade: i32,

    /// 身高 (m)
    height: f64,

    /// 体重 (kg)
    weight: f64,

    /// 肺活量
    vc: i32,

    /// 50m
    r50m: f64,

    /// 坐位体前屈 10%
    sff: f64,

    /// 立定跳远 10%
    slj: f64,

    /// 引体向上(男)
    pull_up: Option<i32>,

    /// 1min 仰卧起坐(女)
    sit_up: Option<i32>,

    /// 1km/800m 20%
    long_run: f64,
}

/// 各项目分数及总分
#[derive(Serialize, Deserialize)]
pub struct StudentPEScore {
    err_msg: Option<String>,

    /// 体重指数(BMI) 15%
    bmi: f64,

    /// 肺活量 15%
    vc: f64,

    /// 50m 跑 20%
    r50m: f64,

    /// 坐位体前屈 10%
    sff: f64,

    /// 立定跳远 10%
    slj: f64,

    /// 引体向上(男)
    pull_up: Option<f64>,

    /// 1min 仰卧起坐(女) 10%
    sit_up: Option<f64>,

    /// 1km/800m 20%
    long_run: f64,

    /// 总分
    total: f64,
}

impl StudentPEScore {
    pub fn with_message(msg: &str) -> StudentPEScore {
        StudentPEScore {
            err_msg: Some(String::from(msg)),
            bmi: 0.,
            vc: 0.,
            r50m: 0.,
            sff: 0.,
            slj: 0.,
            pull_up: None,
            sit_up: None,
            long_run: 0.,
            total: 0.,
        }
    }

    pub fn new(
        bmi: f64,
        vc: f64,
        r50m: f64,
        sff: f64,
        slj: f64,
        pull_up: Option<f64>,
        sit_up: Option<f64>,
        long_run: f64,
    ) -> Self {
        let mut ret = Self {
            err_msg: None,
            bmi,
            vc,
            r50m,
            sff,
            slj,
            pull_up,
            sit_up,
            long_run,
            total: 0.,
        };

        ret.total = Self::cal_total(&ret);
        ret
    }

    fn cal_total(s: &StudentPEScore) -> f64 {
        s.bmi * 0.15
            + s.vc * 0.15
            + s.r50m * 0.2
            + s.sff * 0.1
            + s.long_run * 0.2
            + if let Some(pull_up) = s.pull_up {
                pull_up * 0.1
            } else {
                s.sit_up.unwrap() * 0.1
            }
    }
}

const TABLE: [[[[f64; 20]; 6]; 2]; 2] = [
    // 男
    [
        // 大一大二
        [
            // 肺活量
            [
                5040_f64, 4920_f64, 4800_f64, 4550_f64, 4300_f64, 4180_f64, 4060_f64, 3940_f64,
                3820_f64, 3700_f64, 3580_f64, 3460_f64, 3340_f64, 3220_f64, 3100_f64, 2940_f64,
                2780_f64, 2620_f64, 2460_f64, 2300_f64,
            ],
            // 50m
            [
                6.7, 6.8, 6.9, 7.0, 7.1, 7.3, 7.5, 7.7, 7.9, 8.1, 8.3, 8.5, 8.7, 8.9, 9.1, 9.3,
                9.5, 9.7, 9.9, 10.1,
            ],
            // 坐位体前屈
            [
                24.9, 23.1, 21.3, 19.5, 17.7, 16.3, 14.9, 13.5, 12.1, 10.7, 9.3, 7.9, 6.5, 5.1,
                3.7, 2.7, 1.7, 0.7, -0.3, -1.3,
            ],
            // 立定跳远
            [
                273_f64, 268_f64, 263_f64, 256_f64, 248_f64, 244_f64, 240_f64, 236_f64, 232_f64,
                228_f64, 224_f64, 220_f64, 216_f64, 212_f64, 208_f64, 203_f64, 198_f64, 193_f64,
                188_f64, 183_f64,
            ],
            // 引体向上
            [
                19_f64, 18_f64, 17_f64, 16_f64, 15_f64, 14_f64, 14_f64, 13_f64, 13_f64, 12_f64,
                12_f64, 11_f64, 11_f64, 10_f64, 10_f64, 9_f64, 8_f64, 7_f64, 6_f64, 5_f64,
            ],
            // 1000m
            [
                3.17, 3.22, 3.27, 3.34, 3.42, 3.47, 3.52, 3.57, 4.02, 4.07, 4.12, 4.17, 4.22, 4.27,
                4.32, 4.52, 5.12, 5.32, 5.52, 6.12,
            ],
        ],
        // 大三大四
        [
            // 肺活量
            [
                5140_f64, 5020_f64, 4900_f64, 4650_f64, 4400_f64, 4280_f64, 4160_f64, 4040_f64,
                3920_f64, 3800_f64, 3680_f64, 3560_f64, 3440_f64, 3320_f64, 3200_f64, 3030_f64,
                2860_f64, 2690_f64, 2520_f64, 2350_f64,
            ],
            // 50m
            [
                6.6, 6.7, 6.8, 6.9, 7.0, 7.2, 7.4, 7.6, 7.8, 8.0, 8.2, 8.4, 8.6, 8.8, 9.0, 9.2,
                9.4, 9.6, 9.8, 10.0,
            ],
            // 坐位体前屈
            [
                25.1, 23.3, 21.5, 19.9, 18.2, 16.8, 15.4, 14.0, 12.6, 11.2, 9.8, 8.4, 7.0, 5.6,
                4.2, 3.2, 2.2, 1.2, 0.2, -0.8,
            ],
            // 立定跳远
            [
                275_f64, 270_f64, 265_f64, 258_f64, 250_f64, 246_f64, 242_f64, 238_f64, 234_f64,
                230_f64, 226_f64, 222_f64, 218_f64, 214_f64, 210_f64, 205_f64, 200_f64, 195_f64,
                190_f64, 185_f64,
            ],
            // 引体向上
            [
                20_f64, 19_f64, 18_f64, 17_f64, 16_f64, 15_f64, 15_f64, 14_f64, 14_f64, 13_f64,
                13_f64, 12_f64, 12_f64, 11_f64, 11_f64, 10_f64, 9_f64, 8_f64, 7_f64, 6_f64,
            ],
            // 1000m
            [
                3.15, 3.20, 3.25, 3.32, 3.40, 3.45, 3.50, 3.55, 4.00, 4.05, 4.10, 4.15, 4.20, 4.25,
                4.30, 4.50, 5.10, 5.30, 5.50, 6.10,
            ],
        ],
    ],
    // 女
    [
        // 大一大二
        [
            // 肺活量
            [
                3400_f64, 3350_f64, 3300_f64, 3150_f64, 3000_f64, 2900_f64, 2800_f64, 2700_f64,
                2600_f64, 2500_f64, 2400_f64, 2300_f64, 2200_f64, 2100_f64, 2000_f64, 1960_f64,
                1920_f64, 1880_f64, 1840_f64, 1800_f64,
            ],
            // 50m
            [
                7.5, 7.6, 7.7, 8.0, 8.3, 8.5, 8.7, 8.9, 9.1, 9.3, 9.5, 9.7, 9.9, 10.1, 10.3, 10.5,
                10.7, 10.9, 11.1, 11.3,
            ],
            // 坐位体前屈
            [
                25.8, 24.0, 22.2, 20.6, 19.0, 17.7, 16.4, 15.1, 13.8, 12.5, 11.2, 9.9, 8.6, 7.3,
                6.0, 5.2, 4.4, 3.6, 2.8, 2.0,
            ],
            // 立定跳远
            [
                207_f64, 201_f64, 195_f64, 188_f64, 181_f64, 178_f64, 175_f64, 172_f64, 169_f64,
                166_f64, 163_f64, 160_f64, 157_f64, 154_f64, 151_f64, 146_f64, 141_f64, 136_f64,
                131_f64, 126_f64,
            ],
            // 一分钟仰卧起坐
            [
                56_f64, 54_f64, 52_f64, 49_f64, 46_f64, 44_f64, 42_f64, 40_f64, 38_f64, 36_f64,
                34_f64, 32_f64, 30_f64, 28_f64, 26_f64, 24_f64, 22_f64, 20_f64, 18_f64, 16_f64,
            ],
            // 800m
            [
                3.18, 3.24, 3.30, 3.37, 3.44, 3.49, 3.54, 3.59, 4.04, 4.09, 4.14, 4.19, 4.24, 4.29,
                4.34, 4.44, 4.54, 5.04, 5.14, 5.24,
            ],
        ],
        // 大三大四
        [
            // 肺活量
            [
                3450_f64, 3400_f64, 3350_f64, 3200_f64, 3050_f64, 2950_f64, 2850_f64, 2750_f64,
                2650_f64, 2550_f64, 2450_f64, 2350_f64, 2250_f64, 2150_f64, 2050_f64, 2010_f64,
                1970_f64, 1930_f64, 1890_f64, 1850_f64,
            ],
            // 50m
            [
                7.4, 7.5, 7.6, 7.9, 8.2, 8.4, 8.6, 8.8, 9.0, 9.2, 9.4, 9.6, 9.8, 10.0, 10.2, 10.4,
                10.6, 10.8, 11.0, 11.2,
            ],
            // 坐位体前屈
            [
                26.3, 24.4, 22.4, 21.0, 19.5, 18.2, 16.9, 15.6, 14.3, 13.0, 11.7, 10.4, 9.1, 7.8,
                6.5, 5.7, 4.9, 4.1, 3.3, 2.5,
            ],
            // 立定跳远
            [
                208_f64, 202_f64, 196_f64, 189_f64, 182_f64, 179_f64, 176_f64, 173_f64, 170_f64,
                167_f64, 164_f64, 161_f64, 158_f64, 155_f64, 152_f64, 147_f64, 142_f64, 137_f64,
                132_f64, 127_f64,
            ],
            // 一分钟仰卧起坐
            [
                57_f64, 55_f64, 53_f64, 50_f64, 47_f64, 45_f64, 43_f64, 41_f64, 39_f64, 37_f64,
                35_f64, 33_f64, 31_f64, 29_f64, 27_f64, 25_f64, 23_f64, 21_f64, 19_f64, 17_f64,
            ],
            // 800m
            [
                3.16, 3.22, 3.28, 3.35, 3.42, 3.47, 3.52, 3.57, 4.02, 4.07, 4.12, 4.17, 4.22, 4.27,
                4.32, 4.42, 4.52, 5.02, 5.12, 5.22,
            ],
        ],
    ],
];

/// 肺活量
const ITEM_VC: usize = 0;
/// 50m
const ITEM_50M: usize = 1;
/// 坐位体前屈
const ITEM_SFF: usize = 2;
/// 立定跳远
const ITEM_SL_JUMP: usize = 3;
/// 引体向上/仰卧起坐
const ITEM_PULL_SIT_UP: usize = 4;
/// 1000m/800m
const ITEM_LONG_RUN: usize = 5;

async fn cal(info: StudentPEInfo) -> StudentPEScore {
    let bmi = cal_bmi(info.height, info.weight);

    let gender = info.gender as usize;
    let grade = info.grade as usize;
    let item_level = &TABLE[gender][grade];

    /// 用于测量值越高分越高的项目
    ///
    /// 比如: 肺活量，坐位体前屈，立定跳远，一分钟仰卧起坐
    macro_rules! level_filter {
        ($target:expr, $selection:expr, $level:expr) => {
            match $target {
                t if t >= $level[$selection][0] => 100.,
                t if $level[$selection][1] <= t && t < $level[$selection][0] => 95.,
                t if $level[$selection][2] <= t && t < $level[$selection][1] => 90.,
                t if $level[$selection][3] <= t && t < $level[$selection][2] => 85.,
                t if $level[$selection][4] <= t && t < $level[$selection][3] => 80.,
                t if $level[$selection][5] <= t && t < $level[$selection][4] => 78.,
                t if $level[$selection][6] <= t && t < $level[$selection][5] => 76.,
                t if $level[$selection][7] <= t && t < $level[$selection][6] => 74.,
                t if $level[$selection][8] <= t && t < $level[$selection][7] => 72.,
                t if $level[$selection][9] <= t && t < $level[$selection][8] => 70.,
                t if $level[$selection][10] <= t && t < $level[$selection][9] => 68.,
                t if $level[$selection][11] <= t && t < $level[$selection][10] => 66.,
                t if $level[$selection][12] <= t && t < $level[$selection][11] => 64.,
                t if $level[$selection][13] <= t && t < $level[$selection][12] => 62.,
                t if $level[$selection][14] <= t && t < $level[$selection][13] => 60.,
                t if $level[$selection][15] <= t && t < $level[$selection][14] => 50.,
                t if $level[$selection][16] <= t && t < $level[$selection][15] => 40.,
                t if $level[$selection][17] <= t && t < $level[$selection][16] => 30.,
                t if $level[$selection][18] <= t && t < $level[$selection][17] => 20.,
                t if $level[$selection][19] <= t && t < $level[$selection][18] => 10.,
                t if t < $level[$selection][19] => 0.,
                _ => unreachable!("no such a level"),
            }
        };
    }

    /// 用于测量值越低分越高的项目
    ///
    /// 比如: 50m, 800m/1000m
    macro_rules! level_filter2 {
        ($target:expr, $selection:expr, $level:expr) => {
            match $target {
                t if t < $level[$selection][0] => 100.,
                t if $level[$selection][0] < t && t <= $level[$selection][1] => 95.,
                t if $level[$selection][1] < t && t <= $level[$selection][2] => 90.,
                t if $level[$selection][2] < t && t <= $level[$selection][3] => 85.,
                t if $level[$selection][3] < t && t <= $level[$selection][4] => 80.,
                t if $level[$selection][4] < t && t <= $level[$selection][5] => 78.,
                t if $level[$selection][5] < t && t <= $level[$selection][6] => 76.,
                t if $level[$selection][6] < t && t <= $level[$selection][7] => 74.,
                t if $level[$selection][7] < t && t <= $level[$selection][8] => 72.,
                t if $level[$selection][8] < t && t <= $level[$selection][9] => 70.,
                t if $level[$selection][9] < t && t <= $level[$selection][10] => 68.,
                t if $level[$selection][10] < t && t <= $level[$selection][11] => 66.,
                t if $level[$selection][11] < t && t <= $level[$selection][12] => 64.,
                t if $level[$selection][12] < t && t <= $level[$selection][13] => 62.,
                t if $level[$selection][13] < t && t <= $level[$selection][14] => 60.,
                t if $level[$selection][14] < t && t <= $level[$selection][15] => 50.,
                t if $level[$selection][15] < t && t <= $level[$selection][16] => 40.,
                t if $level[$selection][16] < t && t <= $level[$selection][17] => 30.,
                t if $level[$selection][17] < t && t <= $level[$selection][18] => 20.,
                t if $level[$selection][18] < t && t <= $level[$selection][19] => 10.,
                t if $level[$selection][19] < t => 0.,
                _ => unreachable!("no such a level"),
            }
        };
    }

    // ===== 肺活量 =====
    let vc = info.vc as f64;
    let vc = level_filter!(vc, ITEM_VC, item_level);

    // ===== 坐位体前屈 =====
    let sff = info.sff as f64;
    let sff = level_filter!(sff, ITEM_SFF, item_level);

    // ===== 立定跳远 =====
    let slj = info.slj as f64;
    let slj = level_filter!(slj, ITEM_SL_JUMP, item_level);

    // ===== 引体向上/仰卧起坐 =====
    let ps_up = match gender {
        0 => info.pull_up.unwrap(),
        1 => info.sit_up.unwrap(),
        _ => unreachable!("check in api is too lose"),
    } as f64;
    let ps_up = level_filter!(ps_up, ITEM_PULL_SIT_UP, item_level);

    // ===== 50m =====
    let r50m = info.r50m as f64;
    let r50m = level_filter2!(r50m, ITEM_50M, item_level);

    // ===== 1000m/800m =====
    let long_run = info.long_run as f64;
    let long_run = level_filter2!(long_run, ITEM_LONG_RUN, item_level);

    // 男生
    if gender == 0 {
        StudentPEScore::new(bmi, vc, r50m, sff, slj, Some(ps_up), None, long_run)
    }
    // 女生
    else {
        StudentPEScore::new(bmi, vc, r50m, sff, slj, None, Some(ps_up), long_run)
    }
}

/// kg/m^2
#[inline]
fn cal_bmi(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}

pub enum RequestParamError {
    UnsupportedGender,
    UnsupportedGrade,
}

enum Gender {
    Male,
    Female,
}

impl TryFrom<i32> for Gender {
    type Error = RequestParamError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Gender::Male),
            1 => Ok(Gender::Female),
            _ => Err(RequestParamError::UnsupportedGender),
        }
    }
}

enum Grade {
    Primary,
    Senior,
}

impl TryFrom<i32> for Grade {
    type Error = RequestParamError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Grade::Primary),
            1 => Ok(Grade::Senior),
            _ => Err(RequestParamError::UnsupportedGrade),
        }
    }
}

pub async fn calculate_pe_score(
    Json(info): Json<StudentPEInfo>,
) -> (StatusCode, Json<StudentPEScore>) {
    let mut valid = false;
    if let Ok(gender) = Gender::try_from(info.gender) {
        match gender {
            Gender::Male => {
                if info.pull_up.is_some() {
                    valid = true;
                }
            }
            Gender::Female => {
                if info.sit_up.is_some() {
                    valid = true;
                }
            }
        }
    }

    if !valid {
        (
            StatusCode::BAD_REQUEST,
            Json(StudentPEScore::with_message("unsupported gender")),
        )
    } else if Grade::try_from(info.grade).is_err() {
        (
            StatusCode::BAD_REQUEST,
            Json(StudentPEScore::with_message("invalid grade")),
        )
    } else {
        (StatusCode::OK, Json(cal(info).await))
    }
}

#[cfg(test)]
mod tests {

    // TODO: 测试
    #[test]
    fn test_cal() {}
}
