defmodule GameOfLife do
  def tick(matrix) do
    map = parse(matrix)

    matrix
    |> Stream.with_index()
    |> Enum.map(fn {row, row_ind} ->
      row
      |> Stream.with_index()
      |> Enum.map(fn {_, col_ind} -> calc_cell_state({col_ind, row_ind}, map) end)
    end)
  end

  defp calc_cell_state({x, y}, map) do
    neighbours = [
      {x - 1, y + 1},
      {x, y + 1},
      {x + 1, y + 1},
      {x - 1, y},
      {x + 1, y},
      {x - 1, y - 1},
      {x, y - 1},
      {x + 1, y - 1}
    ]

    alive_neighbours = Enum.count(neighbours, fn n -> map[n] == 1 end)

    case map[{x, y}] do
      0 when alive_neighbours == 3 -> 1
      1 when alive_neighbours in [2, 3] -> 1
      _ -> 0
    end
  end

  defp parse(matrix) do
    for {row, y} <- Enum.with_index(matrix),
        {value, x} <- Enum.with_index(row),
        into: %{} do
      {{x, y}, value}
    end
  end
end
