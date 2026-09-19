use super::runtime::low_fps;

#[test]
fn one_percent_low_uses_slowest_frame_times() {
    let mut frame_times = vec![10.0; 99];
    frame_times.push(50.0);
    assert_eq!(low_fps(&mut frame_times, 0.01), Some(20.0));
}
