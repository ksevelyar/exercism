defmodule KillerSudokuHelper do
  @doc """
  Return the possible combinations of `size` distinct numbers from 1-9 excluding `exclude` that sum up to `sum`.
  """
  @spec combinations(cage :: %{exclude: [integer], size: integer, sum: integer}) :: [[integer]]
  def combinations(%{exclude: exclude, size: size, sum: sum}) do
    available = Enum.reject(1..9, fn num -> num in exclude end)

    [] |> fill(available, size, sum, %{}) |> Map.values()
  end

  defp fill(cage, available, size, sum, acc) do
    cond do
      length(cage) == size and Enum.sum(cage) == sum ->
        cage = Enum.sort(cage)
        Map.put(acc, cage, cage)

      length(cage) >= size || Enum.sum(cage) >= sum ->
        %{}

      true ->
        Enum.reduce(available, %{}, fn num, acc ->
          combination = fill([num | cage], available -- [num], size, sum, acc)

          Map.merge(combination, acc)
        end)
    end
  end
end
