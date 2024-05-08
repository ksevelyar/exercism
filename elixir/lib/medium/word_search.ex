defmodule WordSearch do
  defmodule Location do
    defstruct [:from, :to]

    @type t :: %Location{
            from: %{row: integer, column: integer},
            to: %{row: integer, column: integer}
          }
  end

  @doc """
  Find the start and end positions of words in a grid of letters.
  Row and column positions are 1 indexed.
  """
  @spec search(grid :: String.t(), words :: [String.t()]) :: %{String.t() => nil | Location.t()}
  def search(grid, words) do
    grid = parse_grid(grid)

    Enum.map(words, fn word ->
      {word, word |> String.codepoints() |> search_word(grid)}
    end)
    |> Map.new()
  end

  defp parse_grid(grid) do
    grid
    |> String.split(~r/\R/)
    |> Stream.with_index()
    |> Enum.flat_map(fn {line, line_ind} ->
      line
      |> String.codepoints()
      |> Stream.with_index()
      |> Enum.map(fn {ch, ind} -> {{ind + 1, line_ind + 1}, ch} end)
    end)
    |> Map.new()
  end

  defp search_word([first_ch | tail], grid) do
    start_positions = Enum.filter(grid, fn {{_x, _y}, ch} -> first_ch == ch end)

    start_and_last =
      Enum.find_value(start_positions, fn position ->
        last_position =
          [
            check(:right, tail, grid, position),
            check(:left, tail, grid, position),
            check(:top, tail, grid, position),
            check(:bottom, tail, grid, position),
            check(:to_right_and_bottom, tail, grid, position),
            check(:to_left_and_top, tail, grid, position),
            check(:to_right_and_top, tail, grid, position),
            check(:to_left_and_bottom, tail, grid, position)
          ]
          |> Enum.find(&(!is_nil(&1)))

        if last_position, do: {position, last_position}
      end)

    if start_and_last do
      {{{start_col, start_row}, _}, {{last_col, last_row}, _}} = start_and_last

      %Location{
        from: %{column: start_col, row: start_row},
        to: %{column: last_col, row: last_row}
      }
    end
  end

  defp next_char_key(direction, x, y) do
    case direction do
      :right -> {x + 1, y}
      :left -> {x - 1, y}
      :top -> {x, y - 1}
      :bottom -> {x, y + 1}
      :to_right_and_top -> {x + 1, y + 1}
      :to_right_and_bottom -> {x + 1, y - 1}
      :to_left_and_bottom -> {x - 1, y - 1}
      :to_left_and_top -> {x - 1, y + 1}
    end
  end

  defp check(_direction, [], _grid, prev), do: prev

  defp check(direction, [current_ch | tail], grid, {{prev_x, prev_y}, _vch}) do
    key = next_char_key(direction, prev_x, prev_y)
    next_ch = grid[key]

    case next_ch do
      ^current_ch -> check(direction, tail, grid, {key, next_ch})
      _ -> nil
    end
  end
end
