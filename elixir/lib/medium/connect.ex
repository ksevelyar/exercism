defmodule Connect do
  @doc """
  Calculates the winner (if any) of a board
  using "O" as the white player
  and "X" as the black player
  """

  @black "X"
  @white "O"

  @spec result_for([String.t()]) :: :none | :black | :white
  def result_for(board) do
    max_row = length(board) - 1
    white_cells = parse_board(board, @white)
    start_nodes = Enum.filter(white_cells, fn {_x, y} -> y == 0 end)

    white_result =
      find_neighbours_or_win(start_nodes, white_cells, fn {_x, y} -> y == max_row end, :white)

    max_col = String.length(hd(board)) * 2 - 2
    black_cells = parse_board(board, @black)
    start_nodes = Enum.filter(black_cells, fn {x, y} -> x == y end)

    black_result =
      find_neighbours_or_win(start_nodes, black_cells, fn {x, y} -> x == max_col + y end, :black)

    white_result || black_result || :none
  end

  defp parse_board(board, color) do
    board
    |> Stream.with_index()
    |> Enum.flat_map(fn {line, line_ind} ->
      line
      |> String.codepoints()
      |> Stream.with_index()
      |> Stream.filter(fn {ch, _ind} -> ch == color end)
      |> Enum.map(fn {_ch, ind} -> {ind * 2 + line_ind, line_ind} end)
    end)
  end

  defp find_neighbours_or_win([], _board, _win, _win_color), do: nil

  defp find_neighbours_or_win([{head_x, head_y} | tail] = visited, board, win, win_color) do
    case win.({head_x, head_y}) do
      true ->
        win_color

      false ->
        neighbours =
          find_neighbours(head_x, head_y, board) |> Enum.filter(fn node -> node not in tail end)

        neighbours
        |> Enum.map(fn n -> find_neighbours_or_win([n | visited], board, win, win_color) end)
        |> Enum.at(0)
    end
  end

  defp find_neighbours(head_x, head_y, board) do
    Enum.filter(board, fn {x, y} ->
      case {x, y} do
        {x, ^head_y} when x - 2 == head_x or x + 2 == head_x ->
          true

        {x, y}
        when (x - 1 == head_x or x + 1 == head_x) and (y - 1 == head_y or y + 1 == head_y) ->
          true

        _ ->
          false
      end
    end)
  end
end
