defmodule GoCounting do
  @type position :: {integer, integer}
  @type owner :: %{owner: atom, territory: [position]}
  @type territories :: %{white: [position], black: [position], none: [position]}

  @doc """
  Return the owner and territory around a position
  """
  @spec territory(board :: String.t(), position :: position) :: {:ok, owner} | {:error, String.t()}
  def territory(board, pos) do
    dots = parse_dots(board)
    map = Map.new(dots)

    if map[pos] do
      territory = pos |> get_dot_territory(map, MapSet.new()) |> MapSet.to_list()
      owner = get_dot_owner(territory, map)

      {:ok, %{territory: territory, owner: owner}}
    else
      {:error, "Invalid coordinate"}
    end
  end

  @doc """
  Return all white, black and neutral territories
  """
  @spec territories(board :: String.t()) :: territories
  def territories(board) do
    dots = parse_dots(board)
    map = Map.new(dots)

    territory_dots = Enum.filter(dots, fn {_, ch} -> ch == "_" end)

    Enum.map(territory_dots, fn {pos, _} ->
      territory = pos |> get_dot_territory(map, MapSet.new()) |> MapSet.to_list()
      owner = get_dot_owner(territory, map)

      %{territory: territory, owner: owner}
    end)
    |> Enum.uniq_by(& &1)
    |> Enum.reduce(%{black: [], none: [], white: []}, fn territory, acc ->
      %{acc | territory.owner => acc[territory.owner] ++ territory.territory}
    end)
  end

  defp get_dot_owner(territory, map) do
    stones =
      Stream.flat_map(territory, fn pos ->
        find_neighbours(pos, &(map[&1] in ["B", "W"]))
      end)

    owner =
      Enum.reduce_while(stones, nil, fn stone, acc ->
        value = map[stone]

        cond do
          is_nil(acc) ->
            {:cont, value}

          acc == value ->
            {:cont, acc}

          true ->
            {:halt, nil}
        end
      end)

    case owner do
      "B" -> :black
      "W" -> :white
      _ -> :none
    end
  end

  defp get_dot_territory(pos, map, territory) do
    if map[pos] == "_" do
      neighbours = find_neighbours(pos, &(map[&1] == "_" and &1 not in territory))

      Enum.reduce(neighbours, MapSet.put(territory, pos), fn neighbour, acc ->
        MapSet.union(get_dot_territory(neighbour, map, acc), acc)
      end)
    else
      territory
    end
  end

  defp find_neighbours({x, y}, matcher) do
    Enum.filter([{x + 1, y}, {x, y - 1}, {x - 1, y}, {x, y + 1}], fn pos ->
      matcher.(pos)
    end)
  end

  defp parse_dots(board) do
    String.split(board, "\n", trim: true)
    |> Stream.with_index()
    |> Stream.flat_map(fn {line, y} ->
      String.split(line, "", trim: true)
      |> Stream.with_index()
      |> Enum.map(fn {ch, x} -> {{x, y}, ch} end)
    end)
  end
end
