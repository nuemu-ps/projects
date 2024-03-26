#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

#[macro_export]
macro_rules! queue {
    ($writer:expr $(, $command:expr)* $(,)?) => {
        $($command.write_ansi($writer)?;)*
    }
}
