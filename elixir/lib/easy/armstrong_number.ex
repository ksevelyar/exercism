defmodule Easy.ArmstrongNumber do
  @moduledoc """
  Provides a way to validate whether or not a number is an Armstrong number
  """

  defp sum_of_powered_digits(digits, pow) do
    Enum.reduce(digits, 0, fn digit, sum -> sum + String.to_integer(digit) ** pow end)
  end

  @spec valid?(integer) :: boolean
  def valid?(number) do
    number_string = Integer.to_string(number)
    pow = String.length(number_string)
    sum = String.graphemes(number_string) |> sum_of_powered_digits(pow)

    sum == number
  end
end
