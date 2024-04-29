defmodule Rectangles do
  @doc """
  Count the number of ASCII rectangles.
  """
  @spec count(input :: String.t()) :: integer
  def count(input) do
    input
    |> parse_dots()
    |> find_rectangles()
    |> Enum.uniq()
    |> length()
  end

  defp parse_dots(input) do
    rows = String.split(input, ~r/\R/)

    rows
    |> Enum.with_index()
    |> Enum.flat_map(fn {col, y} ->
      col
      |> String.codepoints()
      |> Enum.with_index()
      |> Enum.map(fn {d, x} -> {d, x, y} end)
    end)
  end

  defp find_rectangles(dots) do
    corners = for {"+", x, y} <- dots, do: {x, y}

    Enum.flat_map(corners, &find_dot_rectangles(&1, corners, dots))
  end

  defp find_dot_rectangles({origin_x, origin_y}, corners, dots) do
    diagonals = for {x, y} <- corners, x > origin_x and y > origin_y, do: {x, y}

    diagonals_with_corners =
      Enum.filter(diagonals, fn {x, y} ->
        {origin_x, y} in corners and {x, origin_y} in corners
      end)

    diagonals_with_valid_connections =
      Enum.filter(diagonals_with_corners, fn {diag_x, diag_y} ->
        hor =
          Enum.filter(dots, fn {dot, x, y} ->
            dot in ["+", "-"] and (y == origin_y || y == diag_y) and (x > origin_x and x < diag_x)
          end)

        hor = hor |> length() == (diag_x - origin_x - 1) * 2

        ver =
          Enum.filter(dots, fn {dot, x, y} ->
            dot in ["+", "|"] and (x == origin_x || x == diag_x) and (y > origin_y and y < diag_y)
          end)

        ver = ver |> length() == (diag_y - origin_y - 1) * 2

        hor && ver
      end)

    Enum.map(diagonals_with_valid_connections, fn x -> {{origin_x, origin_y}, x} end)
  end
end
