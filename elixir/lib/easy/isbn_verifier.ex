defmodule IsbnVerifier do
  @doc """
    Checks if a string is a valid ISBN-10 identifier

    ## Examples

      iex> IsbnVerifier.isbn?("3-598-21507-X")
      true

      iex> IsbnVerifier.isbn?("3-598-2K507-0")
      false

  """
  @spec isbn?(String.t()) :: boolean
  def isbn?(isbn) do
    isbn
    |> String.graphemes()
    |> Enum.reject(&(&1 == "-"))
    |> validate_symbols()
  end

  defp validate_symbols(symbols) when length(symbols) == 10 do
    sum =
      symbols
      |> Enum.with_index()
      |> Enum.reduce_while(0, fn x, acc ->
        value = symbol_value(x)
        if value == :error, do: {:halt, :error}, else: {:cont, acc + value}
      end)

    sum != :error && rem(sum, 11) == 0
  end
  defp validate_symbols(_symbols), do: false


  defp symbol_value({"X", 9}), do: 10
  defp symbol_value({symbol, index}) when symbol in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] do
    String.to_integer(symbol) * (10 - index)
  end
  defp symbol_value({_symbol, _index}), do: :error
end
