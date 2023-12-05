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
  def generate(_coins, 0), do: {:ok, []}
  def generate(_coins, target) when target < 0, do: {:error, "cannot change"}

  def generate(coins, target) do
    acc =
      Enum.reduce(1..target, %{}, fn current_target, acc ->
        iterate_coins(coins, current_target, acc)
      end)

    case acc[target] do
      nil -> {:error, "cannot change"}
      list -> {:ok, Enum.reverse(list)}
    end
  end

  defp iterate_coins([], _current_target, acc), do: acc
  defp iterate_coins([coin | coins], target, acc) do
    case target - coin do
      0 ->
        iterate_coins(coins, target, Map.put(acc, target, [coin]))

      rem when rem > 0 ->
        case acc[rem] do
          nil ->
            iterate_coins(coins, target, acc)

          calculated_coins ->
            iterate_coins(coins, target, Map.put(acc, target, [coin | calculated_coins]))
        end

      _ ->
        iterate_coins(coins, target, acc)
    end
  end
end
