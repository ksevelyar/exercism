defmodule Easy.ArmstrongNumber do
  @moduledoc """
  Provides a way to validate whether or not a number is an Armstrong number
  """

  defp sum_of_powered_digits(digits, pow) do
    Enum.reduce(digits, 0, fn digit, sum -> sum + digit ** pow end)
  end

  def valid?(number) do
    digits = Integer.digits(number)
    pow = length(digits)

    sum_of_powered_digits(digits, pow) == number
  end
end
