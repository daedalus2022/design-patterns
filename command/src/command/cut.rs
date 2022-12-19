use cursive::views::EditView;

use crate::AppContext;

use super::Command;

#[derive(Default)]
pub struct CutCommand{
    backup: String,
}

impl Command for CutCommand{
    fn execute(&mut self, app: &mut cursive::Cursive)->bool {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext|{
            self.backup = editor.get_content().to_string();
            context.clipboard = self.backup.clone();
            editor.set_content("".to_string());
        });

        true
    }

    fn undo(&mut self, app:&mut cursive::Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}