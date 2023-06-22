//! 大学牲体育成绩计算器

use axum::{http::StatusCode, routing::{post, get}, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", post(calculate_pe_score))
        .route("/ping", get(pong));

    let addr = "127.0.0.1:9991".parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn pong() -> (StatusCode, String) {
    (StatusCode::OK, "Pong".to_owned())
}

#[derive(Serialize, Deserialize)]
struct StudentPEInfo {
    /// 性别和年级
    ///
    /// - `00`: 男 / 大一大二
    /// - `01`: 男 / 大三大四
    /// - `10`: 女 / 大一大二
    /// - `11`: 女 / 大三大四
    flag: i32,

    /// 身高 (m)
    height: f64,

    /// 体重 (kg)
    weight: f64,

    /// 肺活量
    vc: i32,

    /// 50m
    r50m: f64,

    /// 坐位体前屈 10%
    sitting_forward_flexion: f64,

    /// 立定跳远 10%
    standing_long_jump: f64,

    /// 引体向上(男)
    pull_up: Option<i32>,

    /// 1min 仰卧起坐(女)
    sit_up: Option<i32>,

    /// 1km/800m 20%
    long_run: f64,
}

/// 各项目分数及总分
#[derive(Serialize)]
struct StudentPEScore {
    /// 体重指数(BMI) 15%
    bmi: f64,

    /// 肺活量 15%
    vc: f64,

    /// 50m 跑 20%
    r50m: f64,

    /// 坐位体前屈 10%
    sitting_forward_flexion: f64,

    /// 立定跳远 10%
    standing_long_jump: f64,

    /// 引体向上(男)
    pull_up: Option<f64>,

    /// 1min 仰卧起坐(女) 10%
    sit_up: Option<f64>,

    /// 1km/800m 20%
    long_run: f64,

    /// 总分
    total: f64,
}

async fn cal(info: StudentPEInfo) -> StudentPEScore {
    let _ = info;
    todo!()
}

async fn calculate_pe_score(
    Json(stu_pe_info): Json<StudentPEInfo>,
) -> (StatusCode, Json<StudentPEScore>) {
    (StatusCode::OK, Json(cal(stu_pe_info).await))
}

#[cfg(test)]
mod tests {
    use crate::{StudentPEInfo, StudentPEScore};

    #[test]
    fn test_dto_json_conv() {
        let info_str = serde_json::to_string(&INFO).unwrap();
        let score_str = serde_json::to_string(&SCORE).unwrap();

        println!("{}", info_str);
        println!("{}", score_str);
    }

    static INFO: StudentPEInfo = StudentPEInfo {
        flag: 11,
        height: 1.83,
        weight: 74.2,
        vc: 4800,
        r50m: 7.5,
        sitting_forward_flexion: 14.0,
        standing_long_jump: 3.0,
        pull_up: Some(12),
        sit_up: None,
        long_run: 3.45,
    };
    static SCORE: StudentPEScore = StudentPEScore {
        bmi: 27.0,
        vc: 1.0,
        r50m: 1.0,
        sitting_forward_flexion: 1.0,
        standing_long_jump: 1.0,
        pull_up: Some(1.0),
        sit_up: None,
        long_run: 1.0,
        total: 12.0,
    };
}
