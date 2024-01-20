defmodule Spiral do
  def matrix(0), do: []

  def matrix(dimension) do
    matrix(1, dimension ** 2, {0, 0}, dimension, :right, %{})
  end

  defp matrix(value, max_value, _cell, dimension, _direction, visited_cells)
       when value > max_value do
    visited_cells
    |> Enum.sort_by(fn {{a, b}, _} -> {b, a} end)
    |> Enum.map(fn {_cell, value} -> value end)
    |> Enum.chunk_every(dimension)
  end

  defp matrix(step, max_step, cell, dimension, direction, visited_cells) do
    visited_cells = Map.put(visited_cells, cell, step)
    {direction, next_cell} = next_cell(cell, dimension, direction, visited_cells)

    matrix(step + 1, max_step, next_cell, dimension, direction, visited_cells)
  end

  defp next_in_direction(direction, {x, y}) do
    case direction do
      :right -> {x + 1, y}
      :bottom -> {x, y + 1}
      :left -> {x - 1, y}
      :top -> {x, y - 1}
    end
  end

  defp next_cell(cell, dimension, direction, visited_cells) do
    next_cell = next_in_direction(direction, cell)

    if path_blocked?(next_cell, dimension, visited_cells) do
      direction =
        case direction do
          :right -> :bottom
          :bottom -> :left
          :left -> :top
          :top -> :right
        end

      {direction, next_in_direction(direction, cell)}
    else
      {direction, next_cell}
    end
  end

  defp path_blocked?({x, y}, dimension, visited_cells) do
    bounds? = x > dimension - 1 || y > dimension - 1 || x < 0 || y < 0

    bounds? || Map.has_key?(visited_cells, {x, y})
  end
end
