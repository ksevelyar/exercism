defmodule StateOfTicTacToe do
  @doc """
  Determine the state a game of tic-tac-toe where X starts.
  """
  @spec game_state(board :: String.t()) :: {:ok, :win | :ongoing | :draw} | {:error, String.t()}
  def game_state(board) do
    board = """
    XOO
    X..
    X..
    """
  end

  defp invalid_state?(board) do
  end

  defp win?(board) do
    # check vertical lines
    # check horizontal lines

    {:ok, :win}
  end

  defp draw?(board) do
    # check no empty cells available
    {:ok, :draw}
  end
end
