use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Segment {
    pub start: f32,
    pub end: f32,
    pub text: String,
}

impl Segment {
    pub fn duration(&self) -> f32 {
        self.end - self.start
    }

    pub fn cps(&self) -> f32 {
        let d = self.duration();
        if d <= 0.0 {
            return 0.0;
        }
        self.text.chars().count() as f32 / d
    }

    /// Làm sạch text: xóa khoảng trắng thừa, newline, tab...
    pub fn cleaned_text(&self) -> String {
        self.text
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    /// Tự động ngắt dòng thông minh (Word-Wrap)
    /// max_line_len: số ký tự tối đa trên 1 dòng (thường là 42 cho SRT)
    pub fn wrapped_text(&self, max_line_len: usize) -> String {
        let clean = self.cleaned_text();
        let mut result = String::new();
        let mut current_len = 0;

        for word in clean.split_whitespace() {
            let word_len = word.chars().count();

            // Nếu thêm từ này sẽ vượt quá giới hạn, xuống dòng
            if current_len + word_len + 1 > max_line_len && current_len > 0 {
                result.push('\n');
                current_len = 0;
            }

            if current_len > 0 {
                result.push(' ');
                current_len += 1;
            }

            result.push_str(word);
            current_len += word_len;
        }

        result
    }
}

fn format_srt_time(seconds: f32) -> String {
    let total_ms = (seconds * 1000.0).round() as u64;
    let h = total_ms / 3_600_000;
    let m = (total_ms % 3_600_000) / 60_000;
    let s = (total_ms % 60_000) / 1000;
    let ms = total_ms % 1000;
    format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
}

pub fn to_srt(segments: &[Segment]) -> String {
    let mut output = String::new();
    for (i, seg) in segments.iter().enumerate() {
        output.push_str(&format!("{}\n", i + 1));
        output.push_str(&format!(
            "{} --> {}\n",
            format_srt_time(seg.start),
            format_srt_time(seg.end)
        ));

        // Tự động ngắt dòng đẹp (tối đa 42 ký tự/dòng)
        output.push_str(&seg.wrapped_text(42));

        output.push_str("\n\n");
    }
    output
}

pub fn to_txt(segments: &[Segment]) -> String {
    segments
        .iter()
        .map(|s| s.cleaned_text())
        .collect::<Vec<String>>()
        .join("\n")
}
