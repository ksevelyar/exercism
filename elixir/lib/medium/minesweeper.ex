defmodule Minesweeper do
  @doc """
  Annotate empty spots next to mines with the number of mines next to them.
  """
  @spec annotate([String.t()]) :: [String.t()]
  def annotate(board) do
    map = parse(board)

    board
    |> Stream.with_index()
    |> Enum.map(fn {row, row_index} ->
      String.codepoints(row)
      |> Stream.with_index()
      |> Stream.map(fn {cell, col_index} -> annotate_cell(cell, {row_index, col_index}, map) end)
      |> Enum.join()
    end)
  end

  defp annotate_cell("*", _, _), do: "*"

  defp annotate_cell(" ", {row_index, col_index}, map) do
    neighbours = [
      {row_index, col_index - 1},
      {row_index, col_index + 1},
      {row_index - 1, col_index - 1},
      {row_index - 1, col_index},
      {row_index - 1, col_index + 1},
      {row_index + 1, col_index - 1},
      {row_index + 1, col_index},
      {row_index + 1, col_index + 1}
    ]

    mines = Enum.count(neighbours, fn key -> map[key] == "*" end)

    if mines == 0, do: " ", else: mines
  end

  defp parse(board) do
    board
    |> Stream.with_index()
    |> Enum.flat_map(fn {row, row_index} ->
      String.codepoints(row)
      |> Stream.with_index()
      |> Enum.map(fn {cell, col_index} -> {{row_index, col_index}, cell} end)
    end)
    |> Map.new()
  end
end
