defmodule Luhn do
  require Integer

  def valid?(string) do
    number_without_whitespaces = String.replace(string, ~r/\s/, "")

    valid_length?(number_without_whitespaces) and valid_sum?(number_without_whitespaces)
  end

  defp valid_length?(string) do
    String.length(string) > 1
  end

  defp valid_sum?(string) do
    sum =
      string
      |> String.graphemes()
      |> Enum.reverse()
      |> Enum.with_index()
      |> Enum.reduce_while(0, fn char, acc ->
        value = char_value(char)
        if value == :error, do: {:halt, :error}, else: {:cont, acc + value}
      end)

    case sum do
      :error -> false
      _ -> rem(sum, 10) == 0
    end
  end

  defp char_value({char, index}) do
    case Integer.parse(char) do
      {digit, _} ->
        digit = if Integer.is_odd(index), do: digit * 2, else: digit
        if digit > 9, do: digit - 9, else: digit

      _ ->
        :error
    end
  end
end
