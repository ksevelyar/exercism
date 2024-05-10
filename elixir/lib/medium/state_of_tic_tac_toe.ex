defmodule StateOfTicTacToe do
  @doc """
  Determine the state a game of tic-tac-toe where X starts.
  """
  @spec game_state(board :: String.t()) :: {:ok, :win | :ongoing | :draw} | {:error, String.t()}
  def game_state(board) do
    board = parse_board(board)
    x = Enum.count(board, fn {_, ch} -> ch == "X" end)
    o = Enum.count(board, fn {_, ch} -> ch == "O" end)
    wins = wins(board)

    cond do
      o > x -> {:error, "Wrong turn order: O started"}
      x - o > 1 -> {:error, "Wrong turn order: X went twice"}
      wins > 1 -> {:error, "Impossible board: game should have ended after the game was won"}
      wins == 0 and x + o < 9 -> {:ok, :ongoing}
      wins == 1 -> {:ok, :win}
      true -> {:ok, :draw}
    end
  end

  defp wins(board) do
    (verticals(board) ++ horizontals(board) ++ diagonals(board))
    |> Enum.filter(fn
      ["X", "X", "X"] -> true
      ["O", "O", "O"] -> true
      _ -> false
    end)
    |> Enum.uniq()
    |> Enum.count()
  end

  defp parse_board(board) do
    for {row, row_i} <- board |> String.split(~r/\R/) |> Stream.with_index(),
        {char, col_i} <- row |> String.codepoints() |> Stream.with_index(),
        into: %{} do
      {{row_i, col_i}, char}
    end
  end

  defp verticals(board) do
    Enum.map([0, 1, 2], fn x ->
      [board[{x, 0}], board[{x, 1}], board[{x, 2}]]
    end)
  end

  defp horizontals(board) do
    Enum.map([0, 1, 2], fn y ->
      [board[{0, y}], board[{1, y}], board[{2, y}]]
    end)
  end

  defp diagonals(board) do
    [
      [board[{0, 0}], board[{1, 1}], board[{2, 2}]],
      [board[{0, 2}], board[{1, 1}], board[{2, 0}]]
    ]
  end
end
