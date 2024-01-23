defmodule PalindromeProducts do
  @doc """
  Generates all palindrome products from an optionally given min factor (or 1) to a given max factor.
  """
  @spec generate(non_neg_integer, non_neg_integer) :: map
  def generate(max_factor, min_factor \\ 1)

  def generate(max_factor, min_factor) when max_factor < min_factor do
    raise ArgumentError
  end

  def generate(max_factor, min_factor) do
    min = find_palindrome(min_factor, max_factor)
    max = find_palindrome(max_factor, min_factor)

    [min, max] |> Enum.reject(&is_nil/1) |> Map.new()
  end

  defp factors(num, range) do
    range
    |> Stream.map(fn factor1 ->
      [factor1, div(num, factor1)]
    end)
    |> Enum.filter(fn [factor1, factor2] -> rem(num, factor1) == 0 and factor2 in range end)
    |> Enum.map(&Enum.sort/1)
    |> Enum.uniq()
  end

  defp find_palindrome(min_factor, max_factor) do
    (min_factor * min_factor)..(max_factor * max_factor)
    |> Stream.filter(&palindrome?/1)
    |> Stream.map(fn num ->
      {num, factors(num, min_factor..max_factor)}
    end)
    |> Enum.find(fn {_, factors} -> Enum.any?(factors) end)
  end

  defp palindrome?(num) do
    digits = Integer.digits(num)

    digits == Enum.reverse(digits)
  end
end
