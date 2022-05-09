fn main() {
  windows::build!(
    Windows::Web::Syndication::*,
    Windows::Foundation::*,
    Windows::Foundation::Collections::*,
    Windows::Win32::WindowsAndMessaging::MessageBoxA,
  );
}
