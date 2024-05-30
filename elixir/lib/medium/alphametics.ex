defmodule Alphametics do
  @type puzzle :: binary
  @type solution :: %{required(?A..?Z) => 0..9}

  @doc """
  Takes an alphametics puzzle and returns a solution where every letter
  replaced by its number will make a valid equation. Returns `nil` when
  there is no valid solution to the given puzzle.

  ## Examples

    iex> Alphametics.solve("I + BB == ILL")
    %{?I => 1, ?B => 9, ?L => 0}

    iex> Alphametics.solve("A == B")
    nil
  """
  @spec solve(puzzle) :: solution | nil
  def solve(puzzle) do
    [sum, result] = String.split(puzzle, " == ")
    sum = String.split(sum, " + ") |> Enum.map(&String.to_charlist/1)
    result = String.to_charlist(result)
    words = [result | sum]
    non_zero_letters = Enum.map(words, &hd/1)
    unique_letters = words |> List.flatten() |> Enum.uniq()

    find_set(sum, result, unique_letters, non_zero_letters)
  end

  defp find_set(sum, result, unique_letters, non_zero_letters) do
    sum = sum |> Enum.map(&(&1 |> Enum.reverse() |> Enum.with_index()))
    result = result |> Enum.reverse() |> Enum.with_index()
    sets = stream_sets(length(unique_letters) - 1, Stream.map(0..9, fn x -> [x] end))

    Enum.find_value(sets, &verify_dict(&1, sum, result, unique_letters, non_zero_letters))
  end

  defp stream_sets(0, acc), do: acc

  defp stream_sets(n, acc) do
    stream_sets(
      n - 1,
      for x <- 0..9, y <- acc, x not in y do
        [x | y]
      end
    )
  end

  defp verify_dict(set, sum, result, letters, non_zero_letters) do
    map = Enum.zip(letters, set) |> Map.new()

    if valid_letters?(non_zero_letters, map) and valid_sum?(map, result, sum) do
      map
    end
  end

  defp valid_letters?(non_zero_letters, map) do
    Enum.all?(non_zero_letters, fn letter -> map[letter] != 0 end)
  end

  defp valid_sum?(map, result, sum) do
    result = word_to_number(map, result)

    sum =
      Enum.reduce_while(sum, 0, fn x, acc ->
        if acc <= result do
          {:cont, acc + word_to_number(map, x)}
        else
          {:halt, nil}
        end
      end)

    sum == result
  end

  defp word_to_number(map, word) do
    Enum.reduce(word, 0, fn {ch, ind}, acc ->
      acc + map[ch] * 10 ** ind
    end)
  end
end
