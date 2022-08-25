defmodule AllYourBase do
  @doc """
  Given a number in input base, represented as a sequence of digits, converts it to output base,
  or returns an error tuple if either of the bases are less than 2
  """

  def convert(_, _input_base, output_base) when output_base < 2 do
    {:error, "output base must be >= 2"}
  end

  def convert(_, input_base, _output_base) when input_base < 2 do
    {:error, "input base must be >= 2"}
  end

  def convert(digits, input_base, output_base) do
    case digits_valid?(digits, input_base) do
      true -> {:ok, digits |> base_10(input_base) |> output_base_digits(output_base, [])}
      false -> {:error, "all digits must be >= 0 and < input base"}
    end
  end

  defp digits_valid?(digits, input_base) do
    Enum.all?(digits, fn digit -> digit < input_base and digit >= 0 end)
  end

  defp base_10(digits, input_base) do
    digits
    |> Enum.reverse()
    |> Enum.with_index()
    |> Enum.reduce(0, fn {digit, pow}, sum -> digit * input_base ** pow + sum end)
  end

  defp output_base_digits(0, _output_base, []), do: [0]
  defp output_base_digits(0, _output_base, digits), do: digits

  defp output_base_digits(value_10, output_base, digits) do
    digit = rem(value_10, output_base)
    remainder = div(value_10, output_base)

    output_base_digits(remainder, output_base, [digit | digits])
  end
end
