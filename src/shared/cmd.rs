use bossy::{ExitStatus, Output};
use color_eyre::{
	eyre::{eyre, Report},
	Section, SectionExt,
};
use ext_trait::extension;

// pub trait CmdReportExt {
// 	fn run_and_wait_for_output_report
// }

// impl bossy::Command {
// 	fn run_and_wait_for_output_report(&mut self) -> std::result::Result<Output, Report> {
// 		let result = self.run_and_wait_for_output()?;

// 		if !result.success() {
// 			let exit_code = result.status().code();
// 			let err = eyre!(
// 				"Command {:?} exited with a non-zero status code {:?}",
// 				self.display(),
// 				exit_code
// 			)
// 			.with_section(|| {
// 				String::from_utf8_lossy(result.stdout())
// 					.trim()
// 					.to_string()
// 					.header("Stdout: ")
// 			}).with_section(|| {
// 				String::from_utf8_lossy(result.stderr())
// 					.trim()
// 					.to_string()
// 					.header("Stderr: ")
// 			});
// 			Err(err)
// 		} else {
// 			Ok(result)
// 		}
// 	}
// }
