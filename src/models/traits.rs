pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}

pub trait ToCsv<T> {
    fn to_csv_entries(&self) -> Vec<T>;
} 