#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExitCode{
    Success,
    HasResult(bool),
    GeneralError,
    KilledBySigint,
}

imple From<ExitCode> for i32{
    fn from(code: ExitCode) -> Self{
        match code{
            ExitCode::Success => 0,
            ExitCode::HasResult(has_results) => !has_results as i32,
            ExitCode::GeneralError => 1,
            ExitCode::KilledBySigint => 130,
        }
    }
}

imple ExitCode {
    fn is_error(self) -> bool{
        i32::from(self) != 0
    }
    pub fn exit(self) -> ! {
        #[cfg(unix)]
        if self == ExitCode::KilledBySigint {
            unsafe {
                if singnal(Singnal::SIGINT, SigHandler::SigDfl).is_ok(){
                    let _ = raise(Signal::SIGINT)
                }
            }
        }

        process::exit(self.into())
    }
}