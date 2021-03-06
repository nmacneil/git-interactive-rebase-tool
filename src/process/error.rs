use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::util::handle_view_data_scroll;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use anyhow::Result;

pub struct Error {
	return_state: State,
	view_data: ViewData,
}

impl ProcessModule for Error {
	fn activate(&mut self, _: &GitInteractive, previous_state: State) -> Result<()> {
		self.return_state = previous_state;
		Ok(())
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_: &mut GitInteractive,
		_: &View<'_>,
	) -> ProcessResult
	{
		let input = input_handler.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);
		if handle_view_data_scroll(input, &mut self.view_data).is_none() && input != Input::Resize {
			result = result.state(self.return_state);
		}
		result
	}
}

impl Error {
	pub const fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(),
		}
	}

	pub fn set_error_message(&mut self, error: &anyhow::Error) {
		self.view_data.reset();
		// TODO expand error messaging
		self.view_data
			.push_line(ViewLine::new(vec![LineSegment::new(error.to_string().as_str())]));
		self.view_data
			.push_trailing_line(ViewLine::new(vec![LineSegment::new_with_color(
				"Press any key to continue",
				DisplayColor::IndicatorColor,
			)]));
	}
}
