use sysinfo::{ System, Pid };
use tui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, symbols, widgets::{Block, Borders, Chart, Dataset, Axis}, text::{Span}, Terminal};
use std::{collections::VecDeque, io::{self}, thread, time::{Instant, Duration}};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const MAX_POINTS: usize = 100;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // println!("输入获取的进程的 PID 编号: ");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    // let pid_u32 = input.trim().parse::<u32>()?;
    // let pid = Pid::from(pid_u32 as usize);

    let mut sys = System::new_all();
    let pid = Pid::from(48965);
    let mut data: VecDeque<(f64, f64)> = VecDeque::new();
    let start_time = Instant::now();

    terminal.clear()?;
    loop {
        sys.refresh_process(pid);

        let elapsed = start_time.elapsed().as_secs_f64();
        // 准备要显示的文本内容
        let memory_usage_text = match sys.process(pid) {
            Some(process) => process.memory() as f64 / 1_048_576.0,
            None => {
                println!("Process with PID {} not found", pid);
                break;
            }
        };
        if data.len() >= MAX_POINTS {
            data.pop_front();
        }
        data.push_back((elapsed, memory_usage_text));
        // // 获取进程信息
        // if let Some(process) = sys.process(pid) {
        //     let memory_value = process.memory();
        //     // let gb = memory_value as f64 / 1_073_741_824.0;
        //     let mb = memory_value as f64 / 1_048_576.0;
        //     // 打印进程的内存使用情况
        //     println!("Process memory usage: {:.3} MB", mb);
        //     format!(" Process memory usage: {:.3}", mb);
        // } else {
        //     println!("Process with PID {} not found", pid);
        // }
        
        let binding = data.iter().map(|&(x, y)| (x, y)).collect::<Vec<_>>();

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            let datasets = vec![
                Dataset::default()
                    .name("Memory Usage")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    // .data(&data.iter().map(|&(x, y)| (x, y)).collect::<Vec<_>>()),
                    .data(&binding),
            ];

            let chart = Chart::new(datasets)
                .block(Block::default().borders(Borders::ALL).title("Memory Usage Over Time"))
                .x_axis(Axis::default()
                    .title("Time (s)")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, data.back().map_or(1.0, |&(x, _)| x)])
                    .labels(vec![
                        Span::raw("0"),
                        Span::raw("20"),
                        Span::raw("40"),
                        Span::raw("60"),
                        Span::raw("80"),
                        Span::raw("100"),
                    ])
                )
                .y_axis(Axis::default()
                    .title("Memory (MB)")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0,  data.iter().map(|&(_, y)| y).fold(0.0 / 0.0, f64::max) + 10.0]) // 给最大值增加10MB的余量
                    .labels(vec![
                        Span::raw("0"),
                        Span::raw("20"),
                        Span::raw("40"),
                        Span::raw("60"),
                        Span::raw("80"),
                        Span::raw("100"),
                    ])
                );
            f.render_widget(chart, chunks[0]);
        })?;
        thread::sleep(Duration::from_secs(5));
    }
    disable_raw_mode()?;
    Ok(())
}
