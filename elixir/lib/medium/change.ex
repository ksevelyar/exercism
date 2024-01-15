defmodule Change do
  @doc """
    Determine the least number of coins to be given to the user such
    that the sum of the coins' value would equal the correct amount of change.
    It returns {:error, "cannot change"} if it is not possible to compute the
    right amount of coins. Otherwise returns the tuple {:ok, list_of_coins}

    ## Examples

      iex> Change.generate([5, 10, 15], 3)
      {:error, "cannot change"}

      iex> Change.generate([1, 5, 10], 18)
      {:ok, [1, 1, 1, 5, 10]}

  """

  @spec generate(list, integer) :: {:ok, list} | {:error, String.t()}
  def generate(coins, target) do
    acc =
      Enum.reduce(coins, %{0 => []}, fn coin, acc ->
        iterate_coins(target - coin + 1, coin, acc)
      end)

    case acc[target] do
      nil -> {:error, "cannot change"}
      list -> {:ok, Enum.sort(list)}
    end
  end

  defp iterate_coins(max_target, coin, acc) do
    Enum.reduce(0..max_target, acc, fn target, acc ->
      if best_variant?(acc[target], acc[target + coin]) do
        Map.put(acc, target + coin, [coin | acc[target]])
      else
        acc
      end
    end)
  end

  defp best_variant?(cache, target) do
    cache != nil && (target == nil || length(target) > length(cache))
  end
end
