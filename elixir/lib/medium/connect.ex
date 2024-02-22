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
    max_col = String.length(hd(board)) - 1
    board = dbg(parse_board(board))

    start_nodes = Enum.filter(board, fn {x, y} -> y == 0 end)
    find_neighbours_or_win(start_nodes, board, fn {_x, y} -> y == max_row end)

    # start_nodes = Enum.filter(board, fn {x, y} -> x == 0 end)
    # find_neighbours(start_nodes, board, fn {x, y} -> y == max_col end)
  end

  defp parse_board(board) do
    board
    |> Stream.with_index()
    |> Enum.flat_map(fn {line, line_ind} ->
      line
      |> String.codepoints()
      |> Stream.with_index()
      |> Stream.filter(fn {ch, _ind} -> ch == @white end)
      |> Enum.map(fn {_ch, ind} -> {ind * 2 + line_ind, line_ind} end)
    end)
    |> dbg()
  end

  defp find_neighbours_or_win([{head_x, head_y} | tail] = visited, board, win) do
    dbg(visited)

    case win.({head_x, head_y}) do
      true ->
        :white

      false ->
        neighbours =
          find_neighbours(visited, board) |> Enum.filter(fn node -> node not in tail end)

        dbg(neighbours)

        Enum.map(neighbours, fn n -> find_neighbours_or_win([n|visited], board, win) end)
    end
  end

  defp find_neighbours([{head_x, head_y} | _tail] = visited, board) do
    Enum.filter(board, fn {x, y} ->
      case {x, y} do
        {x, ^head_y} when x - 2 == head_x or x + 2 == head_x ->
          true

        {x, y} when (x - 1 == head_x or x + 1 == head_x) and (y - 1 == head_y or y + 1 == head_y) ->
           true

        _ ->
          false
      end
    end)
  end
end
