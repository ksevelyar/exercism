defmodule RomanNumerals do
  @numerals %{
    1 => "I",
    10 => "X",
    100 => "C",
    1000 => "M",
    5 => "V",
    50 => "L",
    500 => "D"
  }

  @doc """
  Convert the number to a roman number.
  """
  @spec numeral(pos_integer) :: String.t()
  def numeral(number) do
    digits = Integer.digits(number)
    max_pow = Enum.count(digits) - 1

    digits
    |> Enum.with_index()
    |> Enum.map(fn {digit, index} ->
      pow = 10 ** (max_pow - index)
      to_roman(pow, digit)
    end)
    |> Enum.join()
  end

  defp to_roman(_pow, 0) do
    ""
  end

  defp to_roman(pow, number) when number in 1..3 do
    String.duplicate(@numerals[pow], number)
  end

  defp to_roman(pow, 4) do
    "#{@numerals[pow]}#{@numerals[pow * 5]}"
  end

  defp to_roman(pow, 5) do
    @numerals[pow * 5]
  end

  defp to_roman(pow, number) when number in 6..8 do
    "#{@numerals[pow * 5]}#{String.duplicate(@numerals[pow], number - 5)}"
  end

  defp to_roman(pow, 9) do
    "#{@numerals[pow]}#{@numerals[pow * 10]}"
  end
end
