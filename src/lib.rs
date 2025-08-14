/// Route64 ライブラリ
/// テストやモジュール間の依存関係のためにモジュールを公開
pub mod app;
pub mod app_impl;
pub mod audio;
pub mod save;
pub mod scenario;
pub mod states;
pub mod ui;
pub mod ui_impl;

// よく使用される型を再エクスポート
pub use app_impl::create_app;
pub use save::SaveManager;
pub use scenario::{Choice, Current, Ending, Node, ScenarioData};
pub use states::{AppState, BeginNewGame, ContinueGame, MakeChoice, RestartGame};
pub use states::{BeginNewButton, ChoiceButton, ContinueButton, RestartButton};
pub use states::{EndingUI, PlayingUI, TitleUI};
