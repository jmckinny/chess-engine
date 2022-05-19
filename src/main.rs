mod board;
mod piece;
fn main() -> Result<(), &'static str> {
    let board = board::Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    println!("{:?}", board);
    let fen = board.to_fen();
    println!("{}", fen);
    Ok(())
}
