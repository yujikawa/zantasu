use leptos::prelude::*;

use crate::models::hard_worker::HardWorkerStats;

#[component]
pub fn RadarChart(stats: HardWorkerStats) -> impl IntoView {
    let points = stats.calculate_polygon_points();

    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="230" height="230">
        // 背景グリッド（固定5段階）
        { (1..=5).map(|level| {
            let r = (level as f64 / 5.0) * 80.0;
            let center = (125.0, 125.0);
            let angle_step = std::f64::consts::PI * 2.0 / 5.0;
            let polygon = (0..5).map(|i| {
                let angle = angle_step * i as f64 - std::f64::consts::FRAC_PI_2;
                let x = center.0 + r * angle.cos();
                let y = center.1 + r * angle.sin();
                format!("{x},{y}")
            }).collect::<Vec<_>>().join(" ");
            view! {
                <polygon points={polygon} fill="none" stroke="#ccc" stroke-width="1" />
            }
        }).collect_view() }

        // 軸線
        { (0..5).map(|i| {
            let angle_step = std::f64::consts::PI * 2.0 / 5.0;
            let angle = angle_step * i as f64 - std::f64::consts::FRAC_PI_2;
            let x = 125.0 + 80.0 * angle.cos();
            let y = 125.0 + 80.0 * angle.sin();
            view! {
                <line x1="125" y1="125" x2={x.to_string()} y2={y.to_string()} stroke="#aaa" stroke-width="1" />
            }
        }).collect_view() }

        // 能力値ポリゴン
        <polygon points={points} fill="rgba(0, 128, 255, 0.4)" stroke="#0055aa" stroke-width="2" />

        // ラベル
        <text x="125" y="20" font-size="10" fill="#fff" text-anchor="middle">勤勉さ</text>
        <text x="210" y="80" font-size="10" fill="#fff" text-anchor="middle">集中力</text>
        <text x="190" y="205" font-size="10" fill="#fff" text-anchor="middle">段取り力</text>
        <text x="60" y="205" font-size="10" fill="#fff" text-anchor="middle">自己管理</text>
        <text x="40" y="80" font-size="10" fill="#fff" text-anchor="middle">持続力</text>
    </svg>
    }
}
