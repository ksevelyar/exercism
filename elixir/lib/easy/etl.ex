defmodule ETL do
  @doc """
  Transforms an old Scrabble score system to a new one.

  ## Examples

    iex> ETL.transform(%{1 => ["A", "E"], 2 => ["D", "G"]})
    %{"a" => 1, "d" => 2, "e" => 1, "g" => 2}
  """
  @spec transform(map) :: map
  def transform(input) do
    Enum.reduce(input, %{}, fn {score, letters}, map -> add_letters(map, letters, score) end)
  end

  defp add_letters(map, [], _score), do: map

  defp add_letters(map, [letter | letters], score) do
    downcased_letter = String.downcase(letter)
    Map.put(map, downcased_letter, score) |> add_letters(letters, score)
  end
end
